mod database_logic;
mod server_logic;
mod util;

use crate::server_logic::friend_server_request::*;
use crate::server_logic::message_server_request::*;
use crate::server_logic::photo_server_request::*;
use crate::server_logic::user_server_request::*;


use warp::Filter;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /*let mut var: HashMap<String, String> = HashMap::new();
    var.insert("to".to_string(), "Users/16350".to_string());
    var.insert("from".to_string(), "Users/16381".to_string());
    var.insert("message".to_string(), "allo".to_string());
    match add_message_function(var).await{
        Ok(response) => print!("o"),
        Err(_) => print!("AS")
    }*/


    let routes = friend_routes()
    .or(photo_routes())
    .or(user_routes()).or(message_routes());

        warp::serve(routes.clone()).run(([127, 0, 0, 1], 3000)).await; 
  
    Ok(())
}
