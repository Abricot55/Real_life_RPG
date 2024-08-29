use crate::database_logic::database_logic::*;
use crate::util::json_to_hashmap;
use chrono::{DateTime, Duration, Utc};
use comment_photo::comment_routes;
use hyper::StatusCode;
use hyper::{Body, Response};
use other_photo_things::other_photo_routes;
use serde_json::Map;
use serde_json::Value;
use std::collections::HashMap;
use warp::reply::Reply;
use warp::Filter;

mod comment_photo;
mod other_photo_things;

use super::structs::{DocumentType, MessageType, PhotoListType, PhotoType};

pub fn photo_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let delete_photo_route = warp::path::end()
        .and(warp::query::<HashMap<String, String>>())
        .and_then(delete_photo);

    let update_photo_route = warp::path::end()
        .and(warp::query::<HashMap<String, String>>())
        .and_then(update_photo);

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

    let post = warp::post()
        .and(save_wall_route)
        .or(save_both_route)
        .or(save_storie_route);

    let get = warp::get()
        .and(get_photo_route)
        .or(get_storie_route)
        .or(get_wall_route);

    let put = warp::put().and(update_photo_route);

    let delete = warp::delete().and(delete_photo_route);

    warp::path("photo").and(post.or(get).or(delete).or(put).or(comment_routes()).or(other_photo_routes()))
}

/**
 * @brief This function convert a map containing the photo information into a PhotoType struture.
 * @param params -> A map with all necessary information. This map must contains the field image.
 * @return It return the resulting Photo.
 */
fn convert_hash_photo(params: HashMap<String, String>) -> Result<PhotoType, String> {
    let image = match params.get("image") {
        Some(value) => {
            let split_strings: Vec<&str> = value[1..value.len() - 1].split(',').collect();
            split_strings
                .iter()
                .map(|s| s.trim_matches('"').to_string())
                .collect()
        }
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
    let comments: Vec<MessageType> = match params.get("comments") {
        Some(value) => match serde_json::to_value(value) {
            Ok(real_value) => match serde_json::from_value::<Vec<MessageType>>(real_value) {
                Ok(messages) => messages,
                Err(_) => return Err("Problem encountered when parsing comments".to_string()),
            },
            Err(_) => return Err("Problem while parsing comments".to_string()),
        },
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
    let date = Utc::now().to_rfc3339();

    let photo_id: i32 = match params.get("photo_id") {
        Some(value) => value
            .parse::<i32>()
            .map_err(|_| "The number of like need to be a number")?,
        None => return Err("The photo needs an id".to_string()),
    };
    return Ok(PhotoType {
        photo_id,
        image,
        title,
        likes,
        comments,
        shared,
        description,
        date,
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
                                    let mut ph = photo.clone();
                                    ph.photo_id = (wall_value.len() * 2 + 1) as i32;
                                    temp_wall.push(ph);
                                }
                                if is_storie {
                                    let mut ph = photo.clone();
                                    ph.photo_id = (storie_value.len() * 2) as i32;
                                    temp_storie.push(ph);
                                }
                                let photo_list: PhotoListType = PhotoListType {
                                    _key: Some(key.clone()),
                                    wall: temp_wall,
                                    storie: temp_storie,
                                };
                                match update_document_in_collection(
                                    key.clone(),
                                    DocumentType::Photos(photo_list),
                                    "Photos".to_string(),
                                    "MainDB".to_string(),
                                )
                                .await
                                {
                                    Ok(_) => {
                                        return Ok(warp::reply::with_status(
                                            "Photo Succesfully added",
                                            StatusCode::ACCEPTED,
                                        )
                                        .into_response())
                                    }
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
 * @brief This function return the list of photo on the wall of a user.
 * @param params -> A map that must contain the key field which represent the document.
 * @return A Response with the list of photos in it body.
 */
async fn get_wall(params: HashMap<String, String>) -> Result<Response<Body>, warp::Rejection> {
    get_photo_list(params, true, false).await
}

/**
 * @brief This function return the list of photo in the stories of a user.
 * @param params -> A map that must contain the key field which represent the document.
 * @return A Response with the list of photos in it body.
 */
async fn get_storie(
    params: HashMap<String, String>,
) -> Result<Response<Body>, warp::Rejection> {
    get_photo_list(params, false, true).await
}

/**
 * @brief This function return the list of photo in the stories and on the wall of a user.
 * @param params -> A map that must contain the key field which represent the document.
 * @return A Response with the two lists of photos in it body.
 */
async fn get_everything(
    params: HashMap<String, String>,
) -> Result<Response<Body>, warp::Rejection> {
    get_photo_list(params, true, true).await
}

/**
 * @brief This function get a list of photo in the database depending of the key passed as an argument.
 * @param params -> a Map that contains the key of the list of the document in the photo collection.
 * @param is_wall -> A boolean wihch inform if the wall needs to be retrieve.
 * @param is_storie -> A boolean which inform if the storie needs to be retrieve.
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
                        match get_two_type_from_photolisttype_doc(document.clone()) {
                            Ok(lists) => {
                                match delete_old_stories(key.clone(), &lists[0], lists[1].clone())
                                    .await
                                {
                                    Ok(_) => match serde_json::to_string(&document) {
                                        Ok(json) => Ok(Response::new(Body::from(json))),
                                        Err(_) => Ok(warp::reply::with_status(
                                            "Serialization error",
                                            StatusCode::NOT_ACCEPTABLE,
                                        )
                                        .into_response()),
                                    },
                                    Err(e) => {
                                        Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE)
                                            .into_response())
                                    }
                                }
                            }
                            Err(e) => Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE)
                                .into_response()),
                        }
                    } else {
                        match json_to_hashmap(document.as_str().unwrap()) {
                            Ok(value) => match get_two_type_from_photolisttype_json(value) {
                                Ok(list) => {
                                    fn convert_str_sub(
                                        list: &Vec<PhotoType>,
                                    ) -> Result<Response<Body>, warp::Rejection>
                                    {
                                        match serde_json::to_string(list) {
                                            Ok(wall) => {
                                                print!("{}", wall);
                                                Ok(Response::new(Body::from(wall)))
                                            }
                                            Err(_) => Ok(warp::reply::with_status(
                                                "Serialization error",
                                                StatusCode::NOT_ACCEPTABLE,
                                            )
                                            .into_response()),
                                        }
                                    }

                                    if is_wall {
                                        convert_str_sub(&list[0])
                                    } else {
                                        let stories = list[1].clone();
                                        match delete_old_stories(key.to_string(), &list[0], stories)
                                            .await
                                        {
                                            Ok(new_storie) => convert_str_sub(&new_storie),
                                            Err(e) => Ok(warp::reply::with_status(
                                                e,
                                                StatusCode::NOT_ACCEPTABLE,
                                            )
                                            .into_response()),
                                        }
                                    }
                                }
                                Err(e) => {
                                    Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE)
                                        .into_response())
                                }
                            },
                            Err(e) => Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE)
                                .into_response()),
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
 * @brief This function update the stories of a user if they are there for more than one day.
 * @param key -> The key of the document with the photos.
 * @param wall -> the list of photo on the wall of the user.
 * @param stories ->The list of stories the user posted.
 * @return A result that contains the list of stories but updated.
 */
async fn delete_old_stories(
    key: String,
    wall: &Vec<PhotoType>,
    mut stories: Vec<PhotoType>,
) -> Result<Vec<PhotoType>, String> {
    let mut n = 0;
    for each in stories.as_slice() {
        match DateTime::parse_from_rfc3339(each.date.as_str()) {
            Ok(date) => {
                if Utc::now() - date.to_utc() > Duration::hours(24) {
                    n += 1;
                } else {
                    break;
                }
            }
            Err(_) => return Err("Impossible to convert into date".to_string()),
        }
    }
    if n != 0 {
        for i in (0..n).rev() {
            stories.remove(i);
        }
        for i in 0..stories.len() {
            stories[i].photo_id -= (n * 2) as i32
        }
        delete_document_in_collection(key.clone(), "Photos".to_string(), "MainDB".to_string())
            .await;
        let doc = DocumentType::Photos(PhotoListType {
            _key: Some(key),
            wall: wall.to_vec(),
            storie: stories.clone(),
        });
        match add_document_to_collection(doc, "Photos".to_string(), "MainDB".to_string()).await {
            Ok(_) => return Ok(stories),
            Err(e) => return Err(e),
        }
    } else {
        return Ok(stories);
    }
}

/**
 * @brief This function update a photo with the new informations passed in the params parameter.
 * @param params -> A map that contain the information concerning the updated photo.
 * @return An Option with something in it if the operations was succesfull.
 */
async fn update_photo(params: HashMap<String, String>) -> Result<Response<Body>, warp::Rejection> {
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
                    Ok(doc) => match get_two_type_from_photolisttype_doc(doc) {
                        Ok(lists) => {
                            let mut temp_wall = lists[0].clone();
                            let mut temp_storie = lists[1].clone();
                            if photo.photo_id % 2 == 0 {
                                let place: i32 = photo.photo_id;
                                temp_storie[(place / 2) as usize] = photo;
                            } else {
                                let place: i32 = photo.photo_id;
                                temp_wall[((place - 1) / 2) as usize] = photo;
                            }
                            delete_document_in_collection(
                                key.clone(),
                                "Photos".to_string(),
                                "MainDB".to_string(),
                            )
                            .await;
                            let doc = DocumentType::Photos(PhotoListType {
                                _key: Some(key.clone()),
                                wall: temp_wall.to_vec(),
                                storie: temp_storie.clone(),
                            });
                            match add_document_to_collection(
                                doc,
                                "Photos".to_string(),
                                "MainDB".to_string(),
                            )
                            .await
                            {
                                Ok(_) => {
                                    return Ok(warp::reply::with_status(
                                        "worked fine",
                                        StatusCode::ACCEPTED,
                                    )
                                    .into_response())
                                }
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
                        return Ok(
                            warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response()
                        )
                    }
                }
            }
            Err(e) => {
                return Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response())
            }
        },
        None => {
            return Ok(
                warp::reply::with_status("update didn't worked", StatusCode::NOT_ACCEPTABLE)
                    .into_response(),
            )
        }
    }
}
/**
 * @brief This function update a photo with the new informations passed in the parameter.
 * @param key -> They key of the document in which the photo is.
 * @param photo -> the new photo that need to be put in the document.
 * @return A response body with the informations in it
 */
async fn update_photo_posseded(
    key: String,
    photo: PhotoType,
) -> Result<Response<Body>, warp::Rejection> {
    match get_document_in_collection(key.clone(), "Photos".to_string(), "MainDB".to_string()).await
    {
        Ok(doc) => match get_two_type_from_photolisttype_doc(doc) {
            Ok(lists) => {
                let mut temp_wall = lists[0].clone();
                let mut temp_storie = lists[1].clone();
                if photo.photo_id % 2 == 0 {
                    let place: i32 = photo.photo_id;
                    temp_storie[(place / 2) as usize] = photo;
                } else {
                    let place: i32 = photo.photo_id;
                    temp_wall[((place - 1) / 2) as usize] = photo;
                }
                delete_document_in_collection(
                    key.clone(),
                    "Photos".to_string(),
                    "MainDB".to_string(),
                )
                .await;
                let doc = DocumentType::Photos(PhotoListType {
                    _key: Some(key.clone()),
                    wall: temp_wall.to_vec(),
                    storie: temp_storie.clone(),
                });
                match add_document_to_collection(doc, "Photos".to_string(), "MainDB".to_string())
                    .await
                {
                    Ok(_) => {
                        return Ok(
                            warp::reply::with_status("worked fine", StatusCode::ACCEPTED)
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
        },
        Err(e) => {
            return Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response())
        }
    }
}
/**
 * @brief This function completely change the storie list and the wall list of a user. It can change only one or the two, depending of the parameters.
 * @param key -> The key that represent the document.
 * @param new_storie -> The new storie list. If empty, it keeps the old stories.
 * @param new_wall -> The new wall list. if empty, it keeps the old wall.
 * @param wall -> The old wall.
 * @param storie -> The old stories.
 * @return A result so it is possible to know if it worked or not.
 */
async fn update_photolisttype(
    key: String,
    new_storie: Option<Vec<PhotoType>>,
    new_wall: Option<Vec<PhotoType>>,
    wall: Vec<PhotoType>,
    storie: Vec<PhotoType>,
) -> Result<String, String> {
    let real_wall: Vec<PhotoType> = match new_wall {
        Some(value) => value,
        None => wall,
    };
    let real_storie: Vec<PhotoType> = match new_storie {
        Some(value) => value,
        None => storie,
    };
    let doc = DocumentType::Photos(PhotoListType {
        _key: Some(key.clone()),
        wall: real_wall,
        storie: real_storie,
    });
    update_document_in_collection(key, doc, "Photos".to_string(), "MainDB".to_string()).await
}
/**
 * @brief This struct can make the transfert of single photos without losing the information in which document they are.
 */
struct PhotoInfo {
    key: String,
    photo: PhotoType,
}
/**
 * @brief This function get a specific photo. It returns it inside a photoInfo struct.
 * @param key -> The key of the document in which the photo is.
 * @param photo_id -> The id of the photo which need to be found.
 * @return A result with the photoInfo in it or a string if it dodn't worked.
 */
async fn get_specific_photo(key: String, photo_id: i32) -> Result<PhotoInfo, String> {
    let photolist =
        match get_document_in_collection(key.clone(), "Photos".to_string(), "MainDB".to_string())
            .await
        {
            Ok(document) => match get_two_type_from_photolisttype_doc(document) {
                Ok(photolist_real) => photolist_real,
                Err(_) => return Err("Photolist problem while parsing".to_string()),
            },
            Err(_) => return Err("No document with specified key".to_string()),
        };
    let photo: PhotoType;
    if photo_id % 2 == 0 {
        let place: i32 = photo_id / 2;
        photo = match photolist[1].get(place as usize) {
            Some(value) => value.clone(),
            None => return Err("Cannot access photo with specified id".to_string()),
        };
    } else {
        let place: i32 = (photo_id - 1) / 2;
        photo = match photolist[0].get(place as usize) {
            Some(value) => value.clone(),
            None => return Err("Cannot access photo with specified id".to_string()),
        };
    }
    return Ok(PhotoInfo { key, photo });
}

/**
 * @brief This function take a hashmap and get from it a single photo. The photo is put in a photoInfo struct so the key of the document is accessible to the function that called this one.
 * @param params -> A hashmaps that must contain the key and the photo_id of the concerned photo.
 * @return A result with the photoInfo in it or a string if it dodn't worked.
 */
async fn get_specific_photo_only_hashmap(
    params: HashMap<String, String>,
) -> Result<PhotoInfo, String> {
    let key = match params.get("key") {
        Some(value) => value.clone(),
        None => return Err("No Key provided".to_string()),
    };
    let photo_id = match params.get("photo_id") {
        Some(value) => value
            .parse::<i32>()
            .map_err(|_| "The number of the photo_id need to be a number")?,
        None => return Err("No Photo_id provided".to_string()),
    };
    get_specific_photo(key, photo_id).await
}

/**
 * @brief This function can delete the photo at the specified photo_id in the params parameter.
 * @param params -> A map that must contains the key and the photo_id field.
 * @return A result so it is possible to know if the deletion has been succesfull.
 */
async fn delete_photo(params: HashMap<String, String>) -> Result<Response<Body>, warp::Rejection> {
    let key = match params.get("key") {
        Some(value) => value.clone(),
        None => {
            return Ok(
                warp::reply::with_status("No Key provided", StatusCode::NOT_ACCEPTABLE)
                    .into_response(),
            )
        }
    };
    let photo = match params.get("photo_id") {
        Some(value) => match value.parse::<i32>() {
            Ok(v) => v,
            Err(_) => {
                return Ok(warp::reply::with_status(
                    "Need to be a number",
                    StatusCode::NOT_ACCEPTABLE,
                )
                .into_response())
            }
        },
        None => {
            return Ok(warp::reply::with_status(
                "Impossible to find the photo id",
                StatusCode::NOT_ACCEPTABLE,
            )
            .into_response())
        }
    };
    let document =
        match get_document_in_collection(key.clone(), "Photos".to_string(), "MainDB".to_string())
            .await
        {
            Ok(value) => value,
            Err(e) => {
                return Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response())
            }
        };
    let mut lists = match get_two_type_from_photolisttype_doc(document) {
        Ok(lis) => lis,
        Err(e) => {
            return Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response())
        }
    };
    if photo % 2 == 0 {
        let place = photo / 2;
        lists[1].remove(place as usize);
        for i in place..(lists[1].len()) as i32 {
            lists[1][i as usize].photo_id -= 2;
        }
    } else {
        let place = (photo - 1) / 2;
        lists[0].remove(place as usize);
        for i in place..(lists[0].len()) as i32 {
            lists[0][i as usize].photo_id -= 2;
        }
    }
    match update_photolisttype(key, None, None, lists[0].clone(), lists[1].clone()).await {
        Ok(v) => Ok(warp::reply::with_status(v, StatusCode::ACCEPTED).into_response()),
        Err(e) => Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response()),
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

/**
 * @brief This function retrieve the two different lists in the photos of a user. The two lists are the wall and the storie list.
 * @param photolist -> A document that represent a PhotolistType.
 * @return A result with a vector with the two lists if it worked and a string if it didn't work.
 */
fn get_two_type_from_photolisttype_doc(doc: Value) -> Result<Vec<Vec<PhotoType>>, String> {
    let photolist = match json_to_hashmap(doc.as_str().unwrap()) {
        Ok(value) => value,
        Err(_) => return Err("could not convert the doc into map".to_string()),
    };
    return get_two_type_from_photolisttype_json(photolist);
}


