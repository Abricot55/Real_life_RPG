mod database_logic;
mod server_logic;
mod util;
use crate::server_logic::server_logic::*;
use std::collections::HashMap;
use warp::Filter;


#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
 
    let get_user_route = warp::path("users") 
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>()).and_then(get_user_function);
//////surment ici le probleme
    let search_user_route = warp::path("users").and(warp::path("search")) 
    .and(warp::get())
    .and(warp::query::<HashMap<String, String>>()).and_then(search_user_function);

    let post_user_route = warp::path("users")
        .and(warp::post())
        .and(warp::body::json()).and_then(add_user_function);

    let routes = get_user_route.or(post_user_route).or(search_user_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
    Ok(())
}
