use crate::database_logic::database_logic::*;
use crate::util::{is_valid_email, json_to_hashmap, weird_json_normal_str};
use chrono::Local;
use hyper::body::to_bytes;
use hyper::StatusCode;
use hyper::{Body, Response};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::time::Duration;
use tokio::time::error::Elapsed;
use tokio::time::sleep;
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
 * @brief This function convert a map containing the relation information into a RelationUserUserType struture.
 * @param params -> A map with all necessary information. This map must contains the fields from and to.
 * @return It returns the resulting Relation.
 */
async fn convert_hash_to_UU_relation(
    params: HashMap<String, String>,
) -> Result<RelationUserUserType, String> {
    let _from = match params.get("from") {
        Some(value) => value.clone(),
        None => return Err("The relation needs to come from someone".to_string()),
    };

    let _to = match params.get("to") {
        Some(value) => value.clone(),
        None => return Err("The relation need to go to someone".to_string()),
    };

    let force = match params.get("force") {
        Some(value) => value
            .parse::<i32>()
            .map_err(|_| "The force need to be a number")?,
        None => 0,
    };

    let time = match params.get("time") {
        Some(value) => value
            .parse::<i32>()
            .map_err(|_| "The time need to be a number")?,
        None => 0,
    };

    let relation_type = "friends".to_string();

    return Ok(RelationUserUserType {
        _from,
        _to,
        force,
        time,
        relation_type,
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
                    map.insert(
                        "pseudo".to_string(),
                        serde_json::Value::String(pseudo.clone()),
                    );
                    sleep(Duration::from_millis(100)).await;

                    let mut retries = 5;
                    while retries > 0 {
                        match search_field(
                            map.clone(),
                            "users_view".to_string(),
                            "MainDB".to_string(),
                        )
                        .await
                        {
                            Ok(values) => {
                                if values.len() > 0 {
                                    print!("Received user data.");
                                    match values[0].get("_key") {
                                        Some(key) => {
                                            let real_key: String =
                                                weird_json_normal_str(key.to_string());
                                            let temp_photo: PhotoListType = PhotoListType {
                                                _key: Some(real_key),
                                                photos: vec![],
                                            };
                                            match add_document_to_collection(
                                                DocumentType::Photos(temp_photo),
                                                "Photos".to_string(),
                                                "MainDB".to_string(),
                                            )
                                            .await
                                            {
                                                Ok(_) => {
                                                    print!("Photo document added successfully.");
                                                    return Ok(warp::reply::with_status(
                                                        "All documents have been created!",
                                                        StatusCode::CREATED,
                                                    )
                                                    .into_response());
                                                }
                                                Err(err) => {
                                                    eprintln!(
                                                        "Failed to add photo document: {:?}",
                                                        err
                                                    );
                                                    return Ok(warp::reply::with_status("The photo document couldn't be added to the collection", StatusCode::INTERNAL_SERVER_ERROR).into_response());
                                                }
                                            }
                                        }
                                        None => {
                                            return Ok(warp::reply::with_status(
                                                "The document did not have a _key field",
                                                StatusCode::NOT_FOUND,
                                            )
                                            .into_response())
                                        }
                                    }
                                } else {
                                    print!("No user found, retrying...");
                                }
                            }
                            Err(err) => {
                                eprintln!("Failed to search user: {:?}", err);
                                return Ok(warp::reply::with_status(
                                    "User search failed",
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                )
                                .into_response());
                            }
                        }

                        retries -= 1;
                        sleep(Duration::from_millis(200)).await;
                    }

                    Ok(warp::reply::with_status(
                        "No user found after retries",
                        StatusCode::NOT_FOUND,
                    )
                    .into_response())
                }
                Err(err) => {
                    eprintln!("Failed to add user document: {:?}", err);
                    Ok(warp::reply::with_status(
                        "Failed to add user",
                        StatusCode::INTERNAL_SERVER_ERROR,
                    )
                    .into_response())
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to convert params to user: {:?}", err);
            Ok(
                warp::reply::with_status("Invalid user data", StatusCode::BAD_REQUEST)
                    .into_response(),
            )
        }
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
                Ok(document) => {
                    print!("{}", document.to_string());
                    Ok(Response::new(Body::from(document.to_string())))
                }
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
                            Some(value) => {
                                match serde_json::from_value::<Vec<PhotoType>>(value.clone()) {
                                    Ok(photos) => {
                                        let mut temp_photos = photos.clone();
                                        temp_photos.push(photo);
                                        let photo_list: PhotoListType = PhotoListType {
                                            _key: Some(key.clone()),
                                            photos: temp_photos,
                                        };
                                        update_document_in_collection(
                                            key.clone(),
                                            DocumentType::Photos(photo_list),
                                            "Photos".to_string(),
                                            "MainDB".to_string(),
                                        )
                                        .await;
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
                                }
                            }
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
 * @brief This function add a new edge document in the friend collection in the database.
 * @param params -> The informations concerning the relation, it needs the keys : from, to and the keys force and time are optional
 * @return The response into a Response<Body>.
 */
pub async fn add_friend_function(
    params: HashMap<String, String>,
) -> Result<Response<Body>, warp::Rejection> {
    match convert_hash_to_UU_relation(params).await {
        Ok(relation) => {
            match add_document_to_collection(
                DocumentType::Uu(relation),
                "Friends".to_string(),
                "MainDB".to_string(),
            )
            .await
            {
                Ok(_) => {
                    return Ok(
                        warp::reply::with_status("Relation Created", StatusCode::ACCEPTED)
                            .into_response(),
                    )
                }
                Err(e) => {
                    return Ok(
                        warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response()
                    )
                }
            }
        }
        Err(e) => {
            return Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response())
        }
    }
}
/**
 * @brief This function get all the friend of a user if the key or id is provided in the parameter.
 * @param params -> A hashmap that must contains the key or the id of the user. The Id will be chosen first.
 * @return A Response<Body> with the friends if friends have been found.
 */
pub async fn get_friends_function(
    params: HashMap<String, String>,
) -> Result<Response<Body>, warp::Rejection> {
    match params.get("id") {
        Some(value) => return sub_function(value.clone()).await,
        None => match params.get("key") {
            Some(key) => return sub_function(format!("Users/{}", key)).await,
            None => {
                return Ok(warp::reply::with_status(
                    "No key or id provided",
                    StatusCode::NOT_ACCEPTABLE,
                )
                .into_response())
            }
        },
    };
    /**
     * @brief This function is doing the real work, the big one around is just parsing the id of the user we want
     */
    async fn sub_function(_id: String) -> Result<Response<Body>, warp::Rejection> {
        match get_relation_from(_id.clone(), "Friends".to_string(), "MainDB".to_string()).await {
            Ok(friend_list) => {
                let mut users: Vec<String> = vec![];
                for i in friend_list {
                    let simple_user = match serde_json::to_string(&i) {
                        Ok(json) => match json_to_hashmap(&json) {
                            Ok(map_relation) => {
                                let mut key = "".to_string();
                                if map_relation["_from"] == _id {
                                    let s = map_relation["_to"].to_string();
                                    key = s[7..s.len() - 1].to_string();
                                } else {
                                    let s = map_relation["_from"].to_string();
                                    key = s.to_string()[7..s.len() - 1].to_string();
                                }
                                match get_document_in_collection(
                                    key.clone(),
                                    "Users".to_string(),
                                    "MainDB".to_string(),
                                )
                                .await
                                {
                                    Ok(value) => value.to_string(),
                                    Err(e) => {
                                        return Ok(warp::reply::with_status(
                                            e,
                                            StatusCode::NOT_ACCEPTABLE,
                                        )
                                        .into_response())
                                    }
                                }
                            }
                            Err(e) => {
                                return Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE)
                                    .into_response())
                            }
                        },
                        Err(e) => {
                            return Ok(warp::reply::with_status(
                                "Could not find the user",
                                StatusCode::NOT_ACCEPTABLE,
                            )
                            .into_response())
                        }
                    };
                    users.push(simple_user);
                }
                match serde_json::to_string(&users) {
                    Ok(final_json) => {
                        return Ok(warp::reply::with_status(final_json, StatusCode::ACCEPTED)
                            .into_response());
                    }
                    Err(e) => {
                        return Ok(warp::reply::with_status(
                            "Could not convert to String",
                            StatusCode::NOT_ACCEPTABLE,
                        )
                        .into_response())
                    }
                }
            }

            Err(e) => {
                return Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response())
            }
        }
    }
}

pub async fn add_message_function(
    params: HashMap<String, String>,
) -> Result<Response<Body>, warp::Rejection> {
    let from: String = match params.get("from") {
        Some(value) => value.clone(),
        None => {
            return Ok(
                warp::reply::with_status("No from id specified", StatusCode::NOT_ACCEPTABLE)
                    .into_response(),
            )
        }
    };
    let to: String = match params.get("to") {
        Some(value) => value.clone(),
        None => {
            return Ok(
                warp::reply::with_status("No to id specified", StatusCode::NOT_ACCEPTABLE)
                    .into_response(),
            )
        }
    };
    let message: String = match params.get("message") {
        Some(value) => value.clone(),
        None => {
            return Ok(
                warp::reply::with_status("No message!!", StatusCode::NOT_ACCEPTABLE)
                    .into_response(),
            )
        }
    };
    let date: String = match params.get("date") {
        Some(value) => value.clone(),
        None => Local::now().format("%Y-%m-%d").to_string(),
    };

    let message: MessageType = MessageType {
        message,
        state: MessageState::SENT,
        date: date.clone(),
        from : from.clone(),
    };
    match get_relation_from_two(
        from.clone(),
        to.clone(),
        "Messages".to_string(),
        "MainDB".to_string(),
    )
    .await
    {
        Ok(all_relations) => {
            if all_relations.is_empty() {
                let new: MessageListType = MessageListType {
                    _from: from,
                    _to: to,
                    messages: vec![message],
                    date: date,
                };
                match add_document_to_collection(
                    DocumentType::Messages(new),
                    "Messages".to_string(),
                    "MainDB".to_string(),
                )
                .await
                {
                    Ok(v) => {
                        return Ok(
                            warp::reply::with_status(v, StatusCode::NOT_ACCEPTABLE).into_response()
                        )
                    }
                    Err(e) => {
                        return Ok(
                            warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response()
                        )
                    }
                }
            } else {
                match json_to_hashmap(all_relations[0].to_string().as_str()) {
                    Ok(map_value) => {
                        let messages_value = match map_value.get("messages") {
                            Some(value) => value.clone(),
                            None => {
                                return Ok(warp::reply::with_status(
                                    "Weird its suppose to work",
                                    StatusCode::NOT_ACCEPTABLE,
                                )
                                .into_response())
                            }
                        };
                        let mut list_mess: Vec<MessageType> =
                            match serde_json::from_value::<Vec<MessageType>>(messages_value) {
                                Ok(json) => json.clone(),
                                Err(_) => {
                                    return Ok(warp::reply::with_status(
                                        "deserialisation error",
                                        StatusCode::NOT_ACCEPTABLE,
                                    )
                                    .into_response())
                                }
                            };
                        list_mess.push(message);
                        let new: MessageListType = MessageListType {
                            _from: from.clone(),
                            _to: to.clone(),
                            messages: list_mess,
                            date: date.clone(),
                        };
                        let key: String = match map_value.get("_key") {
                            Some(value) => {
                                let s = value.to_string();
                                s[1..s.len() - 1].to_string()
                            }
                            None => {
                                return Ok(warp::reply::with_status(
                                    "suppose to work",
                                    StatusCode::NOT_ACCEPTABLE,
                                )
                                .into_response())
                            }
                        };
                        update_document_in_collection(
                            key,
                            DocumentType::Messages(new),
                            "Messages".to_string(),
                            "MainDB".to_string(),
                        )
                        .await;
                        return Ok(warp::reply::with_status(
                            "message added",
                            StatusCode::NOT_ACCEPTABLE,
                        )
                        .into_response());
                    }
                    Err(e) => {
                        return Ok(
                            warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response()
                        )
                    }
                }
            }
        }
        Err(e) => {
            return Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response())
        }
    }
}

pub async fn get_message_function(
    params: HashMap<String, String>,
) -> Result<Response<Body>, warp::Rejection> {
    let from: String = match params.get("from") {
        Some(value) => value.clone(),
        None => {
            return Ok(
                warp::reply::with_status("No from id specified", StatusCode::NOT_ACCEPTABLE)
                    .into_response(),
            )
        }
    };
    match params.get("to") {
        Some(value) => return get_between_message(from, value.clone()).await,
        None => return get_all_messages(from).await,
    }

    async fn get_all_messages(from: String) -> Result<Response<Body>, warp::Rejection> {
        match get_relation_from(from, "Messages".to_string(), "MainDB".to_string()).await {
            Ok(all_relations) => match serde_json::to_string(&all_relations) {
                Ok(value) => {
                    return Ok(warp::reply::with_status(value, StatusCode::ACCEPTED).into_response())
                }
                Err(_) => {
                    return Ok(warp::reply::with_status(
                        "Deserialisation error",
                        StatusCode::NOT_ACCEPTABLE,
                    )
                    .into_response())
                }
            },
            Err(e) => {
                return Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response())
            }
        }
    }
    async fn get_between_message(
        from: String,
        to: String,
    ) -> Result<Response<Body>, warp::Rejection> {
        match get_relation_from_two(from, to, "Messages".to_string(), "MainDB".to_string()).await {
            Ok(all_relations) => match serde_json::to_string(&all_relations) {
                Ok(value) => {
                    return Ok(warp::reply::with_status(value, StatusCode::ACCEPTED).into_response())
                }
                Err(_) => {
                    return Ok(warp::reply::with_status(
                        "Deserialisation error",
                        StatusCode::NOT_ACCEPTABLE,
                    )
                    .into_response())
                }
            },
            Err(e) => {
                return Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response())
            }
        }
        return Ok(warp::reply::with_status("M", StatusCode::NOT_ACCEPTABLE).into_response());
    }
}

/*RÉCENT MAIS A VÉRIFIER SI CA MARCHE' JE NAI PAS ENCORE TESTÉ */
pub async fn update_user_function(
    params: HashMap<String, String>,
) -> Result<Response<Body>, warp::Rejection> {
    match params.get("_key") {
        Some(key) => {
            match get_document_in_collection(key.clone(), "Users".to_string(), "MainDB".to_string())
                .await
            {
                Ok(document) => match json_to_hashmap(document.as_str().unwrap()) {
                    Ok(map_value) => {
                        let mut temp: HashMap<String, String> = HashMap::new();
                        for (k, v) in map_value {
                            let k_temp = k.clone();
                            match params.get(&k) {
                                Some(_) => temp.insert(k, params[&k_temp].clone()),
                                None => temp.insert(k, v.to_string()),
                            };
                        }
                        match convert_hash_to_user(temp) {
                            Ok(user) => {
                                update_document_in_collection(
                                    key.clone(),
                                    DocumentType::User(user),
                                    "Users".to_string(),
                                    "MainDB".to_string(),
                                )
                                .await;
                                return Ok(warp::reply::with_status(
                                    "User updated",
                                    StatusCode::ACCEPTED,
                                )
                                .into_response());
                            }
                            Err(e) => {
                                return Ok(warp::reply::with_status(e, StatusCode::ACCEPTED)
                                    .into_response())
                            }
                        }
                    }
                    Err(e) => {
                        return Ok(warp::reply::with_status(e, StatusCode::ACCEPTED).into_response())
                    }
                },
                Err(e) => {
                    return Ok(warp::reply::with_status(e, StatusCode::ACCEPTED).into_response())
                }
            }
        }
        None => {
            return Ok(
                warp::reply::with_status("No key provided", StatusCode::ACCEPTED).into_response(),
            )
        }
    }
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
