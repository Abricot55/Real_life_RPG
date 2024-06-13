use crate::database_logic::database_logic::*;
use crate::util::is_valid_email;
use hyper::body::to_bytes;
use hyper::StatusCode;
use hyper::{Body, Response};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
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

/*fn get_document(args: &Vec<String>, index_value: i32) -> Result<DocumentType, MyError> {
    //args[index_value] = document type, args[index_value]+ = value
    match args[index_value as usize].to_lowercase().as_str() {
        "skill" => {
            if args.len() >= (index_value + 3) as usize {
                let skill = SkillType {
                    _key: args[(index_value + 1) as usize].clone(),
                    name: args[(index_value + 2) as usize].clone(),
                };
                return Ok(crate::database_logic::database_logic::DocumentType::Skill(
                    skill,
                ));
            } else {
                print!("commande invalide")
            }
        }
        "user" => {
            if args.len() >= (index_value + 7) as usize {
                match args[(index_value + 6) as usize].clone().parse::<i32>() {
                    Ok(_) => {
                        if true {
                            let user = UserType {
                                _key: args[(index_value + 1) as usize].clone(),
                                name: args[(index_value + 2) as usize].clone(),
                                pseudo: args[(index_value + 3) as usize].clone(),
                                email: args[(index_value + 4) as usize].clone(),
                                birth_date: args[(index_value + 5) as usize].clone(),
                                level: args[(index_value + 6) as usize]
                                    .clone()
                                    .parse::<i32>()
                                    .unwrap(),
                            };
                            return Ok(crate::database_logic::database_logic::DocumentType::User(
                                user,
                            ));
                        }
                    }
                    Err(_) => {
                        return Err(MyError {
                            details: "erreur lors de conversion string to int".to_string(),
                        })
                    }
                }
            } else {
                print!("commande invalide")
            }
        }
        _other => print!("other document type, "),
    }
    return Err(MyError {
        details: "document invalide".to_string(),
    });
}*/

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
 * @brief This function is called when a add request is made to the server.
 * @param args -> A vector of string that contains the request to the server.
 * @return A String which indicate the state of the request.
 */
pub async fn add_user_function(
    params: HashMap<String, String>,
) -> Result<Response<Body>, warp::Rejection> {
    match convert_hash_to_user(params) {
        Ok(user) => {
            match add_document_to_collection(
                DocumentType::User(user),
                "Users".to_string(),
                "MainDB".to_string(),
            )
            .await
            {
                Ok(_) => Ok(Response::new(Body::from("Document Créée".to_string()))),
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
            Ok(json) => {Ok(Response::new(Body::from(json)))},
            Err(_) => Ok(warp::reply::with_status(
                "Serialization error",
                StatusCode::NOT_ACCEPTABLE,
            )
            .into_response()),
        },
        Err(e) => {;Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response())},
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
            Ok(json) => {Ok(Response::new(Body::from(json)))},
            Err(_) => Ok(warp::reply::with_status(
                "Serialization error",
                StatusCode::NOT_ACCEPTABLE,
            )
            .into_response()),
        },
        Err(e) => {;Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response())},
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
