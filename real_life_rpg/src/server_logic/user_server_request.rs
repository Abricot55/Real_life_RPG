use crate::database_logic::database_logic::*;
use crate::util::{is_valid_email, json_to_hashmap, weird_json_normal_str};
use hyper::StatusCode;
use hyper::{Body, Response};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;
use warp::reply::Reply;
use warp::Filter;

use super::structs::*;

/**
 * @brief This function return a filter with all the different routes (request that concern the users)
 */
pub fn user_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let update_user_route = warp::path("update")
        .and(warp::body::json())
        .and_then(update_user_function);

    let relevant_search_user_route = warp::path("relevant")
        .and(warp::query::<HashMap<String, Value>>())
        .and_then(relevant_search_user_function);

    let get_user_route = warp::path::end()
        .and(warp::query::<HashMap<String, String>>())
        .and_then(get_user_function);

    let search_user_route = warp::path("search")
        .and(warp::query::<HashMap<String, Value>>())
        .and_then(search_user_function);

    let post_user_route = warp::path::end()
        .and(warp::body::json())
        .and_then(add_user_function);

    let get = warp::get().and(
        search_user_route
            .or(relevant_search_user_route)
            .or(get_user_route),
    );
    let post = warp::post().and(post_user_route);
    let put = warp::put().and(update_user_route);
    warp::path("users").and(get.or(post).or(put))
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
    let title = match params.get("title") {
        Some(value) => value.clone(),
        None => "".to_string(),
    };

    let title_list = match params.get("title_list"){
        Some(value)=> {
            let split_strings: Vec<&str> = value[1..value.len() - 1].split(',').collect();
            split_strings
                .iter()
                .map(|s| s.trim_matches('"').to_string())
                .collect()
        },
        None => Vec::new()
    };
    return Ok(UserType {
        name,
        pseudo,
        email,
        birth_date: birth,
        level,
        password,
        title,
        title_list
    });
}

/**
 * @brief This function is called when a add user request is made to the server. It also create a photo collection with the same key as the user.
 * @param params -> A map containing all the user information. The map must have the name, email, birthday and password field.
 * @return A String which indicate the state of the request.
 */
async fn add_user_function(
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
                                                wall: vec![],
                                                storie: vec![],
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
async fn get_user_function(
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
async fn search_user_function(
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
async fn relevant_search_user_function(
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

/*RÉCENT MAIS A VÉRIFIER SI CA MARCHE' JE NAI PAS ENCORE TESTÉ */
async fn update_user_function(
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
