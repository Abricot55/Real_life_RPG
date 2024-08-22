use crate::database_logic::database_logic::*;
use crate::util::json_to_hashmap;
use hyper::StatusCode;
use hyper::{Body, Response};
use serde_json::Map;
use serde_json::Value;
use std::collections::HashMap;
use warp::reply::Reply;
use warp::Filter;

use super::structs::{DocumentType, PhotoListType, PhotoType};

pub fn photo_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let get_photo_route = warp::path::end()
        .and(warp::query::<HashMap<String, String>>())
        .and_then(get_everything);

    let get_wall_route = warp::path("wall")
        .and(warp::query::<HashMap<String, String>>())
        .and_then(get_wall);

    let get_storie_route = warp::path("storie")
        .and(warp::query::<HashMap<String, String>>())
        .and_then(get_storie);

    let save_wall_route = warp::path("save")
        .and(warp::path("wall"))
        .and(warp::body::json())
        .and_then(add_wall_photo);

    let save_storie_route = warp::path("save")
        .and(warp::path("storie"))
        .and(warp::body::json())
        .and_then(add_storie_photo);

    let save_both_route = warp::path("save")
        .and(warp::path("both"))
        .and(warp::body::json())
        .and_then(add_both_photo);

    let put = warp::put()
        .and(save_wall_route)
        .or(save_both_route)
        .or(save_storie_route);
    let get = warp::get().and(get_photo_route).or(get_storie_route).or(get_wall_route);

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
 * @brief This function call the add_photo_user function but with the field wall set to true.
 * @param params -> A map with all the informations needed. Must contains the key, wall and storie fields.
 * @return A response containing a message indicating if the operation was successfull.
 */
async fn add_wall_photo(
    params: HashMap<String, String>,
) -> Result<Response<Body>, warp::Rejection> {
    return add_photo_user(params, true, false).await;
}
/**
 * @brief This function call the add_photo_user function but with the field storie set to true.
 * @param params -> A map with all the informations needed. Must contains the key, wall and storie fields.
 * @return A response containing a message indicating if the operation was successfull.
 */
async fn add_storie_photo(
    params: HashMap<String, String>,
) -> Result<Response<Body>, warp::Rejection> {
    return add_photo_user(params, false, true).await;
}
/**
 * @brief This function call the add_photo_user function but with the field wall and storie set to true.
 * @param params -> A map with all the informations needed. Must contains the key, wall and storie fields.
 * @return A response containing a message indicating if the operation was successfull.
 */
async fn add_both_photo(
    params: HashMap<String, String>,
) -> Result<Response<Body>, warp::Rejection> {
    return add_photo_user(params, true, true).await;
}

/**
 * @brief This function add a photo in the document with the key provided as a parameter. The map passed in argument must contains the photo and key field for this function to work.
 * @param params -> A map with all the informations needed. Must contains the key, wall and storie fields.
 * @param wall -> A boolean which inform if the photo must be save on the wall or not.
 * @param storie -> A boolean which inform if the photo must be save as a storie or not.
 * @return A response containing a message indicating if the operation was successfull.
 */
async fn add_photo_user(
    params: HashMap<String, String>,
    is_wall: bool,
    is_storie: bool,
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
                        Ok(map) => match get_two_type_from_photolisttype_json(map) {
                            Ok(values) => {
                                let wall_value = &values[0];
                                let storie_value = &values[1];
                                let mut temp_wall = wall_value.clone();
                                let mut temp_storie = storie_value.clone();
                                if is_wall {
                                    temp_wall.push(photo.clone());
                                }
                                if is_storie {
                                    temp_storie.push(photo);
                                }
                                let photo_list: PhotoListType = PhotoListType {
                                    _key: Some(key.clone()),
                                    wall: temp_wall,
                                    storie: temp_storie,
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
                            Err(e) => {
                                return Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE)
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

async fn get_wall(params: HashMap<String, String>) -> Result<Response<Body>, warp::Rejection> {
    get_photo_list(params, true, false).await
}

async fn get_storie(params: HashMap<String, String>) -> Result<Response<Body>, warp::Rejection> {
    get_photo_list(params, false, true).await
}

async fn get_everything(
    params: HashMap<String, String>,
) -> Result<Response<Body>, warp::Rejection> {
    get_photo_list(params, true, true).await
}

/**
 * @brief This function get a list of photo in the database depending of the key passed as an argument.
 * @param params -> a Map that contains the key of the list of the document in the photo collection.
 * @return A response containing the document or a string if it didn't worked.
 */
async fn get_photo_list(
    params: HashMap<String, String>,
    is_wall: bool,
    is_storie: bool,
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
                Ok(document) => {
                    if is_wall == is_storie {
                        match serde_json::to_string(&document) {
                            Ok(json) =>Ok(Response::new(Body::from(json))),
                            Err(_) => Ok(warp::reply::with_status(
                                "Serialization error",
                                StatusCode::NOT_ACCEPTABLE,
                            )
                            .into_response()),
                        }
                    } else {
                        match json_to_hashmap(document.as_str().unwrap()) {
                            Ok(value) => match get_two_type_from_photolisttype_json(value) {
                                Ok(list) => {
                                    fn convert_str_sub(
                                        list: Vec<Vec<PhotoType>>,
                                        which: usize,
                                    ) -> Result<Response<Body>, warp::Rejection>
                                    {
                                        match serde_json::to_string(&list[which]) {
                                            Ok(wall) => Ok(Response::new(Body::from(wall))),
                                            Err(_) => Ok(warp::reply::with_status(
                                                "Serialization error",
                                                StatusCode::NOT_ACCEPTABLE,
                                            )
                                            .into_response()),
                                        }
                                    }

                                    if is_wall {
                                        convert_str_sub(list, 0)
                                    } else {
                                        convert_str_sub(list, 1)
                                    }
                                }
                                Err(_) => todo!(),
                            },
                            Err(_) => todo!(),
                        }
                    }
                }
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
 * @brief This function retrieve the two different lists in the photos of a user. The two lists are the wall and the storie list.
 * @param photolist -> A map that must contains the wall and the storie field. They must contain Vec<PhotoType>.
 * @return A result with a vector with the two lists if it worked and a string if it didn't work.
 */
fn get_two_type_from_photolisttype_json(
    photolist: Map<String, Value>,
) -> Result<Vec<Vec<PhotoType>>, String> {
    let wall = match photolist.get("wall") {
        Some(value) => match serde_json::from_value::<Vec<PhotoType>>(value.clone()) {
            Ok(good) => good.clone(),
            Err(_) => return Err("could not convert wall to photo vector".to_string()),
        },
        None => return Err("There is no wall attribute".to_string()),
    };
    let storie = match photolist.get("storie") {
        Some(value) => match serde_json::from_value::<Vec<PhotoType>>(value.clone()) {
            Ok(good) => good.clone(),
            Err(_) => return Err("could not convert stories to photo vector".to_string()),
        },
        None => return Err("There is no storie attribute".to_string()),
    };
    return Ok(vec![wall, storie]);
}
