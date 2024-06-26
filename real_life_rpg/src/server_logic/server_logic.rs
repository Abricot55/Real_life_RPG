use crate::database_logic::database_logic::*;
use crate::util::{is_valid_email, json_to_hashmap};
use hyper::body::to_bytes;
use hyper::StatusCode;
use hyper::{Body, Response};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use warp::filters::path::param;
use warp::reply::Reply;
/*
*@brief Custom error
*/
#[derive(Debug)]
struct MyError {
    details: String,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

/**
 * @brief This function take the string of a body in a request and tranform it in hashmap.
 * @param The body of the request.
 * @return an option which can contain the hashmap if the conversion worked.
 */
pub async fn get_body_map(body: Body) -> Option<HashMap<String, Value>> {
    match to_bytes(body).await {
        Ok(body_bytes) => match String::from_utf8(body_bytes.to_vec()) {
            Ok(body_str) => match serde_json::from_str(&body_str) {
                Ok(body_map) => return Some(body_map),
                Err(_) => return None,
            },
            Err(_) => return None,
        },
        Err(_) => return None,
    }
}


/**
 * @brief This function convert a map containing the user information into a User struture.
 * @param params -> A map with all necessary information. This map must contains the fields name, email, birthday and password.
 * @return It return the resulting User.
 */
fn convert_hash_to_user(params: HashMap<String, String>) -> Result<UserType, String> {
    let name: String = match params.get("name") {
        Some(value) => value.clone(),
        None => return Err("Aucun nom fourni!".to_string()),
    };
    let pseudo: String = match params.get("pseudo") {
        Some(value) => value.clone(),
        None => name.clone(),
    };
    let email: String = match params.get("email") {
        Some(value) => {
            if is_valid_email(value.clone().as_str()) {
                value.clone()
            } else {
                return Err("Email invalide fourni".to_string());
            }
        }
        None => return Err("Aucun email fourni!".to_string()),
    };
    let birth: String = match params.get("birthday") {
        Some(value) => value.clone(),
        None => return Err("Aucune date de naissance fournie!".to_string()),
    };
    let level = match params.get("level") {
        Some(value) => value
            .parse::<i32>()
            .map_err(|_| "Le level doit être un nombre")?,
        None => 0,
    };
    let password = match params.get("password") {
        Some(value) => value.clone(),
        None => return Err("Aucun mot de passe fourni".to_string()),
    };
    return Ok(UserType {
        name,
        pseudo,
        email,
        birth_date: birth,
        level,
        password,
    });
}

/**
 * @brief This function convert a map containing the photo information into a PhotoType struture.
 * @param params -> A map with all necessary information. This map must contains the field image.
 * @return It return the resulting Photo.
 */
fn convert_hash_photo(params: HashMap<String, String>) -> Result<PhotoType, String> {
    let image = match params.get("image") {
        Some(value) => value.clone(),
        None => return Err("There is no image".to_string()),
    };
    let title: String = match params.get("title") {
        Some(value) => value.clone(),
        None => "".to_string(),
    };
    let likes: i32 = match params.get("likes") {
        Some(value) => value
            .parse::<i32>()
            .map_err(|_| "The number of like need to be a number")?,
        None => 0,
    };
    let comments: Vec<String> = match params.get("comments") {
        Some(value) => {
            let split_strings: Vec<&str> = value[1..value.len() - 1].split(',').collect();
            split_strings
                .iter()
                .map(|s| s.trim_matches('"').to_string())
                .collect()
        }
        None => [].to_vec(),
    };
    let shared: i32 = match params.get("shared") {
        Some(value) => value
            .parse::<i32>()
            .map_err(|_| "The number of shares need to be a number")?,
        None => 0,
    };
    let description: String = match params.get("description") {
        Some(value) => value.clone(),
        None => "".to_string(),
    };
    return Ok(PhotoType {
        image,
        title,
        likes,
        comments,
        shared,
        description,
    });
}

/**
 * @brief This function is called when a add user request is made to the server. It also create a photo collection with the same key as the user.
 * @param params -> A map containing all the user information. The map must have the name, email, birthday and password field.
 * @return A String which indicate the state of the request.
 */
pub async fn add_user_function(
    params: HashMap<String, String>,
) -> Result<Response<Body>, warp::Rejection> {
    match convert_hash_to_user(params) {
        Ok(user) => {
            let pseudo = user.pseudo.clone();
            match add_document_to_collection(
                DocumentType::User(user),
                "Users".to_string(),
                "MainDB".to_string(),
            )
            .await
            {
                Ok(_) => {
                    let mut map: HashMap<String, Value> = HashMap::new();
                    map.insert("pseudo".to_string(), serde_json::Value::String(pseudo));
                    match search_field(map, "users_view".to_string(), "MainDB".to_string()).await{
                        Ok(values) => if values.len() > 0 {
                            match values[0].get("_key"){
                                Some(key) => match add_document_to_collection(DocumentType::Photos(PhotoListType { _key: Some(key.to_string().clone()), photos: [].to_vec() }), "Photos".to_string(), "MainDB".to_string()).await{
                                    Ok(_) => Ok(warp::reply::with_status("All document have been created!", StatusCode::NOT_ACCEPTABLE).into_response()),
                                    Err(_) => Ok(warp::reply::with_status("The photo document couldn't be added to the collection", StatusCode::NOT_FOUND).into_response()),
                                },
                                None => Ok(warp::reply::with_status("The document did not have a _key field", StatusCode::NOT_FOUND).into_response()),
                            }
                        }else{
                            Ok(warp::reply::with_status("More than one user have been found", StatusCode::NOT_FOUND).into_response())
                        },
                        Err(_) => Ok(warp::reply::with_status("L'utilisateur n'a pas été trouvé", StatusCode::NOT_FOUND).into_response()),
                    }},
                Err(e) => Ok(warp::reply::with_status(e, StatusCode::NOT_FOUND).into_response()),
            }
        }
        Err(e) => Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response()),
    }
}

/**
 * @brief Function called when the get/user request is sent to the server.
 * @param params -> A Hashmap containoing different information usefull for the function.
 * @return A String which indicate the state of the request.
 */
pub async fn get_user_function(
    mut params: HashMap<String, String>,
) -> Result<Response<Body>, warp::Rejection> {
    match params.remove("key") {
        Some(key) => {
            match get_document_in_collection(key, "Users".to_string(), "MainDB".to_string()).await {
                Ok(document) => Ok(Response::new(Body::from(document.to_string()))),
                Err(e) => Ok(warp::reply::with_status(e, StatusCode::NOT_FOUND).into_response()),
            }
        }
        None => Ok(warp::reply::with_status("No key given", StatusCode::NOT_FOUND).into_response()),
    }
}

/**
 * @brief This function search a user based on the key of the hashmap passed as parameter. It return a Response which contains the list of users who correspond to the search fields.
 * @param params -> the hashmap ofthe search fields.
 * @return A Response<Body> containing the differents users.
 */
pub async fn search_user_function(
    params: HashMap<String, Value>,
) -> Result<Response<Body>, warp::Rejection> {
    match search_field(params, "users_view".to_string(), "MainDB".to_string()).await {
        Ok(document) => match serde_json::to_string(&document) {
            Ok(json) => Ok(Response::new(Body::from(json))),
            Err(_) => Ok(warp::reply::with_status(
                "Serialization error",
                StatusCode::NOT_ACCEPTABLE,
            )
            .into_response()),
        },
        Err(e) => Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response()),
    }
}
/**
 * @brief This function search the relevants user based on the key of the hashmap passed as parameter. It return a Response which contains the list of users who correspond to the search fields.
 * @param params -> the hashmap ofthe search fields.
 * @return A Response<Body> containing the differents users.
 */
pub async fn relevant_search_user_function(
    params: HashMap<String, Value>,
) -> Result<Response<Body>, warp::Rejection> {
    match relevant_search_field(params, "users_view".to_string(), "MainDB".to_string()).await {
        Ok(document) => match serde_json::to_string(&document) {
            Ok(json) => Ok(Response::new(Body::from(json))),
            Err(_) => Ok(warp::reply::with_status(
                "Serialization error",
                StatusCode::NOT_ACCEPTABLE,
            )
            .into_response()),
        },
        Err(e) => Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response()),
    }
}

/**
 * @brief This function add a photo in the document with the key provided as a parameter. The map passed in argument must contains the photo and key field for this function to work.
 * @param params -> A map with all the informations needed. Must contains the key and photo fields.
 * @return A response containing a message indicating if the operation was successfull.
 */
pub async fn add_photo_user(
    params: HashMap<String, String>,
) -> Result<Response<Body>, warp::Rejection> {
    match params.get("key") {
        Some(key) => match convert_hash_photo(params.clone()) {
            Ok(photo) => {
                match get_document_in_collection(
                    key.clone(),
                    "Photos".to_string(),
                    "MainDB".to_string(),
                )
                .await
                {
                    Ok(document) => match json_to_hashmap(document.as_str().unwrap()) {
                        Ok(map) => match map.get("photos") {
                            Some(value) => match serde_json::from_value::<Vec<PhotoType>>(value.clone()) {
                                Ok(photos) => {
                                    let mut temp_photos = photos.clone();
                                    temp_photos.push(photo);
                                    let photo_list: PhotoListType = PhotoListType {
                                        _key: Some(key.clone()),
                                        photos : temp_photos,
                                    };
                                    update_document_in_collection(key.clone(), DocumentType::Photos(photo_list), "Photos".to_string(), "MainDB".to_string()).await;
                                    return Ok(warp::reply::with_status(
                                        "Photo Succesfully added",
                                        StatusCode::NOT_ACCEPTABLE,
                                    )
                                    .into_response());
                                }
                                Err(_) => {
                                    return Ok(warp::reply::with_status(
                                        "cannot get the photo as vector",
                                        StatusCode::NOT_ACCEPTABLE,
                                    )
                                    .into_response())
                                }
                            },
                            None => {
                                return Ok(warp::reply::with_status(
                                    "Cannot convert to photo list",
                                    StatusCode::NOT_ACCEPTABLE,
                                )
                                .into_response())
                            }
                        },
                        Err(e) => {
                            return Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE)
                                .into_response());
                        }
                    },
                    Err(e) => {
                        return Ok(
                            warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response()
                        );
                    }
                }
            }
            Err(e) => {
                return Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response());
            }
        },
        None => {
            return Ok(warp::reply::with_status(
                "No key provided".to_string(),
                StatusCode::NOT_ACCEPTABLE,
            )
            .into_response());
        }
    }
}


/**
 * @brief This function get a list of photo in the database depending of the key passed as an argument.
 * @param params -> a Map that contains the key of the list of the document in the photo collection.
 * @return A response containing the document or a string if it didn't worked.
 */
pub async fn get_photo_list(
    params: HashMap<String, String>,
) -> Result<Response<Body>, warp::Rejection> {
    match params.get("key") {
        Some(key) => {
            match get_document_in_collection(
                key.clone(),
                "Photos".to_string(),
                "MainDB".to_string(),
            )
            .await
            {
                Ok(document) => match serde_json::to_string(&document) {
                    Ok(json) => Ok(Response::new(Body::from(json))),
                    Err(_) => Ok(warp::reply::with_status(
                        "Serialization error",
                        StatusCode::NOT_ACCEPTABLE,
                    )
                    .into_response()),
                },
                Err(e) => {
                    return Ok(
                        warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response()
                    )
                }
            }
        }
        None => {
            return Ok(
                warp::reply::with_status("No Key provided", StatusCode::NOT_ACCEPTABLE)
                    .into_response(),
            )
        }
    }
}

/*
/**
 * @brief Function called when the update request is sent to the server.
 * @param args -> A vector of string that contains the request to the server
 * @return A String which indicate the state of the request.
 */
async fn update_function(args: &Vec<String>) -> Response<Body> {
    if args.len() >= 2 {
        match args[1].to_lowercase().as_str() {
            //"database" => print!("update database"),
            //"collection" => print!("update collection"),
            "document" => {
                if args.len() >= 6 {
                    //document key, collection name, database name, docutment type, value...
                    match get_document(&args, 5) {
                        Ok(doc) => {
                            update_document_in_collection(
                                args[2].clone(),
                                doc,
                                args[3].clone(),
                                args[4].clone(),
                            )
                            .await;
                            return Response::new(Body::from(
                                "Le document à été mis à jour!".to_string(),
                            ));
                        }
                        Err(e) => return status(StatusCode::NOT_ACCEPTABLE),
                    }
                } else {
                    return status(StatusCode::NOT_ACCEPTABLE);
                }
            }
            //"relation" => print!("update relation"),
            _other => return status(StatusCode::ACCEPTED),
        }
    }
    return status(StatusCode::NOT_ACCEPTABLE);
}
*/
/**
 * @brief Function called when the delete request is sent to the server.
 * @param args -> A vector of string that contains the request to the server
 * @return A String which indicate the state of the request.
 */
async fn delete_function(args: &Vec<String>) -> Response<Body> {
    if args.len() >= 2 {
        match args[1].to_lowercase().as_str() {
            "document" => {
                if args.len() >= 5 {
                    //document key, collection name, database name
                    delete_document_in_collection(
                        args[2].clone(),
                        args[3].clone(),
                        args[4].clone(),
                    )
                    .await;
                    return Response::new(Body::from("Le document à été supprimé".to_string()));
                } else {
                    return status(StatusCode::NOT_ACCEPTABLE);
                }
            }
            _other => return status(StatusCode::ACCEPTED),
        }
    }
    return status(StatusCode::NOT_ACCEPTABLE);
}

/**
 * @brief This function convert a status code into a Repsponse<Body>.
 * @param code -> The status code.
 * @return The Response<Body>
 */
fn status(code: StatusCode) -> Response<Body> {
    let mut the_status: Response<Body> = Response::default();
    *the_status.status_mut() = code;
    return the_status;
}

/**
 * @brief module use to link tests to this librairy
 */
#[cfg(test)]
mod tests {
    use super::*;
    include!("tests_server_logic.rs");
}
