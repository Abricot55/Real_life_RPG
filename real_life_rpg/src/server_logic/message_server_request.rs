use crate::database_logic::database_logic::*;
use crate::util::json_to_hashmap;
use chrono::Local;
use hyper::StatusCode;
use hyper::{Body, Response};
use std::collections::HashMap;
use warp::reply::Reply;
use warp::Filter;

pub fn message_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    let get_message_route = warp::query::<HashMap<String, String>>().and_then(get_message_function);

    let add_message_route = warp::body::json().and_then(add_message_function);

    let get = warp::get().and(get_message_route);
    let post = warp::post().and(add_message_route);

    warp::path("message").and(get.or(post))
}

fn hash_to_message(params: HashMap<String, String>) -> Result<MessageType, String> {
    let from: String = match params.get("from") {
        Some(value) => value.clone(),
        None => return Err("No from id specified".to_string()),
    };

    let message: String = match params.get("message") {
        Some(value) => value.clone(),
        None => return Err("No message!!".to_string()),
    };
    let date: String = match params.get("date") {
        Some(value) => value.clone(),
        None => Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };

    let message_obj: MessageType = MessageType {
        message,
        state: MessageState::SENT,
        date: date.clone(),
        from: from.clone(),
    };

    return Ok(message_obj);
}
/**
 * @brief This function add a message between to users.
 * @param params -> This hashmap contains the important informations. It must contain the fields : from, to and message
 * @return A response body containing a string thaht indicate if the processus worked.
 */
pub async fn add_message_function(
    params: HashMap<String, String>,
) -> Result<Response<Body>, warp::Rejection> {
    let message: MessageType = match hash_to_message(params.clone()) {
        Ok(mes) => mes,
        Err(e) => {
            return Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response())
        }
    };
    let from = message.clone().from;
    let date = message.clone().date;

    let to: String = match params.get("to") {
        Some(value) => value.clone(),
        None => return Ok(warp::reply::with_status("No to id specified", StatusCode::NOT_ACCEPTABLE).into_response()),
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
                    messages: vec![message.clone()],
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
                            warp::reply::with_status(v, StatusCode::ACCEPTED).into_response()
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
                        list_mess.push(message.clone());
                        let new: MessageListType = MessageListType {
                            _from: from.clone(),
                            _to: to.clone(),
                            messages: list_mess,
                            date: date,
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
                        return Ok(
                            warp::reply::with_status("message added", StatusCode::ACCEPTED)
                                .into_response(),
                        );
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

/**
 * @brief This function retrive the messages concerning one ore two users.
 *          If a from field AND a to field are passed in argument, it retrive the messages between thos two,
 *          if only a from is specified it retrieve all the message concerning this user.
 * @param params -> The Hashmap must contain the from field.
 * @return A response body with the list of message that are relevant
 */
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
    }
}
