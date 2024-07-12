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
    var.insert("id".to_string(), "Users/1807".to_string());
    match get_friends_function(var).await{
        Ok(response) => print!("o"),
        Err(_) => print!("AS")
    }*/

    let get_friend_route = warp::path("friend")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and_then(get_friends_function);
    
    let get_photo_route = warp::path("photo")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and_then(get_photo_list);

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

    let post_friend_route = warp::path("friend")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(add_friend_function);

    let update_user_route = warp::path("users")
        .and(warp::path("update"))
        .and(warp::put())
        .and(warp::body::json())
        .and_then(update_user_function);

    let routes = relevant_search_user_route
        .or(get_friend_route)
        .or(get_photo_route)
        .or(update_user_route)
        .or(search_user_route)
        .or(get_user_route)
        .or(post_user_route)
        .or(save_photo_route)
        .or(post_friend_route);
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
    Ok(())
}
