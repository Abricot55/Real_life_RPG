mod database_logic;
mod server_logic;
mod util;

use std::collections::HashMap;

use crate::server_logic::friend_server_request::*;
use crate::server_logic::message_server_request::*;
use crate::server_logic::photo_server_request::*;
use crate::server_logic::user_server_request::*;

use crate::server_logic::structs::SkillType;
use database_logic::database_logic::add_document_to_collection;
use hyper::body::HttpBody;
use warp::Filter;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /*let mut var: HashMap<String, String> = HashMap::new();
    var.insert("key".to_string(), "16350".to_string());
    var.insert("title".to_string(), "Super Duper Cool Photo".to_string());
    var.insert("description".to_string(),"I took this photo being super duper drunk".to_string());
    var.insert("from".to_string(), "Mr Worldwide".to_string());
    var.insert("message_id".to_string(), "1".to_string());
    var.insert("photo_id".to_string(),"1".to_string());
    match hehe(var).await{
        Ok(res) => print!("os"),
        Err(_) => print!("AS")
    }*/
    /*let skill : SkillType = SkillType { name: "Chapeau Melon".to_string(), challenges: Vec::new() };

        add_document_to_collection(database_logic::database_logic::DocumentType::Skill(skill), "Skills".to_string(), "MainDB".to_string()).await.unwrap();
    */ 
    let routes = friend_routes()
        .or(photo_routes())
        .or(user_routes())
        .or(message_routes());

    warp::serve(routes.clone())
        .run(([127, 0, 0, 1], 3000))
        .await;

    Ok(())
}
