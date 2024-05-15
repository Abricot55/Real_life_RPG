mod server_logic;
mod database_logic;
use crate::server_logic::server_logic::*;
use std::error::Error;
use std::net::TcpListener;

#[tokio::main]
async fn main()  -> Result<(), Box<dyn Error>> {
    let what_to_do: i32 = 1;
    if what_to_do == 0{
        //server
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            println!("Connection established!");
        }
    } else if what_to_do == 1{
        //parser
        run().await;
    }
    Ok(())
}


