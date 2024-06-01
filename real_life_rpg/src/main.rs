mod server_logic;
mod database_logic;
use crate::server_logic::server_logic::*;

use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Method, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(handle)) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}


async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path_array : &Vec<String> = &path_to_array(req.uri().path());
    match req.method() {
        &Method::GET => Ok(run("get",path_array).await),
        &Method::POST => Ok(run("add",path_array).await),
        &Method::DELETE => Ok(run("delete",path_array).await),
        &Method::PUT => Ok(run("update",path_array).await),
        _ => {
            let mut not_found: Response<Body> = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

/**
 * @brief This function convert a path separate with "/" into a vector of String.
 * @param path -> the path.
 * @return the vector of string.
 */
fn path_to_array(path : &str) -> Vec<String>{
    let mut temp_str :String = "".to_string();
    let mut req_array : Vec<String> = vec![];
    path.chars().for_each(|letter| {
        if letter == '/'{
        req_array.push(temp_str.clone());
        temp_str.clear();
        }
        else{temp_str.push(letter);}

});
    return req_array;
}



