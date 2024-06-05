mod database_logic;
mod server_logic;
use crate::server_logic::server_logic::*;

use hyper::body::to_bytes;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use hyper::{Method, StatusCode};
use serde_json::Value;
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use warp::Filter;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /*let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())*/
    // GET /your_url?command=some_command
    let get_database_route = warp::path("users") //donc url "http://127.0.0.1:3000" -> si juste warp::path("chemin") -> url "http://127.0.0.1:3000/chemin"
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>()).and_then(get_user_function);

    // POST /your_url with body { "command": "some_command" }
    let post_route = warp::path("your_url")
        .and(warp::post())
        .and(warp::body::json())
        .map(|mut p: HashMap<String, String>| {
            match p.remove("command") {
                Some(command) => {
                    // Traitez la commande POST ici
                    format!("Received POST command: {}", command)
                }
                None => String::from("No command received"),
            }
        });

    let routes = get_database_route.or(post_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
    Ok(())
}
/*
/**
 * @brief This function handle the http request sent to the server.
 * @param req -> The http request sent to the server.
 */*/
/*async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    match get_body_map(req.into_body()).await {
        Some(body_map) => match &method {
            &Method::GET => Ok(get_user_function(body_map).await),
            /*&Method::POST => Ok(run("add", path_array).await),
            &Method::DELETE => Ok(run("delete", path_array).await),
            &Method::PUT => Ok(run("update", path_array).await),*/
            _ => {
                let mut not_found: Response<Body> = Response::default();
                *not_found.status_mut() = StatusCode::NOT_FOUND;
                Ok(not_found)
            }
        },
        None => {
            let mut not_found: Response<Body> = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}*/
