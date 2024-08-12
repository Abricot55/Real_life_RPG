use crate::database_logic::database_logic::*;
use crate::util::json_to_hashmap;
use hyper::StatusCode;
use hyper::{Body, Response};
use std::collections::HashMap;
use warp::reply::Reply;
use warp::Filter;

pub fn photo_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let get_photo_route = warp::query::<HashMap<String, String>>().and_then(get_photo_list);

    let save_photo_route = warp::path("save")
        .and(warp::body::json())
        .and_then(add_photo_user);

    let put = warp::put().and(save_photo_route);
    let get = warp::get().and(get_photo_route);

    warp::path("photo").and(put.or(get))
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
 * @brief This function add a photo in the document with the key provided as a parameter. The map passed in argument must contains the photo and key field for this function to work.
 * @param params -> A map with all the informations needed. Must contains the key and photo fields.
 * @return A response containing a message indicating if the operation was successfull.
 */
async fn add_photo_user(
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
async fn get_photo_list(
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
