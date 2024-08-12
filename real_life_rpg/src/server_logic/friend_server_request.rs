use crate::database_logic::database_logic::*;
use crate::util::json_to_hashmap;
use hyper::StatusCode;
use hyper::{Body, Response};
use std::collections::HashMap;
use warp::reply::Reply;
use warp::Filter;

pub fn friend_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let get_friend_route = warp::query::<HashMap<String, String>>().and_then(get_friends_function);

    let post_friend_route = warp::body::json().and_then(add_friend_function);

    let get = warp::get().and(get_friend_route);
    let post = warp::post().and(post_friend_route);

    warp::path("friend").and(get.or(post))
}

/**
 * @brief This function convert a map containing the relation information into a RelationUserUserType struture.
 * @param params -> A map with all necessary information. This map must contains the fields from and to.
 * @return It returns the resulting Relation.
 */
async fn convert_hash_to_uu_relation(
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
 * @brief This function add a new edge document in the friend collection in the database.
 * @param params -> The informations concerning the relation, it needs the keys : from, to and the keys force and time are optional
 * @return The response into a Response<Body>.
 */
async fn add_friend_function(
    params: HashMap<String, String>,
) -> Result<Response<Body>, warp::Rejection> {
    match convert_hash_to_uu_relation(params).await {
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
async fn get_friends_function(
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
                                let key: String;
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
                        Err(_) => {
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
                    Err(_) => {
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
