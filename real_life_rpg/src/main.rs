mod database_logic;
mod server_logic;
mod util;
use crate::server_logic::server_logic::*;
use serde_json::Value;
use std::collections::HashMap;
use warp::Filter;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /*let mut var: HashMap<String, String> = HashMap::new();
    var.insert("image".to_string(), "s".to_string());
    var.insert("key".to_string(), "1234".to_string());
    match add_photo_user(var).await{
        Ok(response) => print!("o"),
        Err(_) => print!("AS")
    }*/
    let save_photo_route = warp::path("save")
        .and(warp::path("photo"))
        .and(warp::put())
        .and(warp::body::json())
        .and_then(add_photo_user);
    
    let relevant_search_user_route = warp::path("users")
        .and(warp::path("relevant"))
        .and(warp::get())
        .and(warp::query::<HashMap<String, Value>>())
        .and_then(relevant_search_user_function);

    let search_user_route = warp::path("users")
        .and(warp::path("search"))
        .and(warp::get())
        .and(warp::query::<HashMap<String, Value>>())
        .and_then(search_user_function);

    let get_user_route = warp::path("users")
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and_then(get_user_function);

    let post_user_route = warp::path("users")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(add_user_function);

    let routes = relevant_search_user_route
        .or(search_user_route)
        .or(get_user_route)
        .or(post_user_route).or(save_photo_route);
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
    Ok(())
}
