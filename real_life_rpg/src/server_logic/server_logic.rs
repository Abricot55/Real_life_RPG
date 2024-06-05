use crate::database_logic::database_logic::*;
use hyper::body::to_bytes;
use hyper::{Body, Request, Response, Server};
use hyper::{Method, StatusCode};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
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

fn get_document(args: &Vec<String>, index_value: i32) -> Result<DocumentType, MyError> {
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
}

/**
 * @brief This function is called when a add request is made to the server.
 * @param args -> A vector of string that contains the request to the server.
 * @return A String which indicate the state of the request.
 */
async fn add_function(args: &Vec<String>) -> Response<Body> {
    if args.len() >= 3 {
        match args[1].to_lowercase().as_str() {
            "database" => {
                create_new_db(args[2].clone()).await;
                return Response::new(Body::from("Database Créée".to_string()));
            } //database name
            "collection" => {
                if args.len() >= 4 {
                    create_new_collection(args[2].clone(), args[3].clone()).await;
                    return Response::new(Body::from("Collection Créée".to_string()));
                }
                //collection name, database name
                else {
                    return status(StatusCode::NOT_ACCEPTABLE);
                }
            }
            "document" => {
                if args.len() >= 5 {
                    //collection name, database name, document type, value...
                    match get_document(&args, 4) {
                        Ok(doc) => {
                            add_document_to_collection(doc, args[2].clone(), args[3].clone()).await;
                            return Response::new(Body::from("Document Créée".to_string()));
                        }
                        Err(e) => return status(StatusCode::NOT_ACCEPTABLE),
                    }
                } else {
                    return status(StatusCode::NOT_ACCEPTABLE);
                }
            }
            "relation" => {
                if args.len() >= 4 {
                    create_new_relation(args[2].clone(), args[3].clone()).await;
                    return Response::new(Body::from("Relation Créée".to_string()));
                // relation name, database name
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
                Err(_) => Ok(status(StatusCode::NOT_FOUND)),
            }
        }
        None => Ok(status(StatusCode::NOT_ACCEPTABLE)),
    }
}

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
