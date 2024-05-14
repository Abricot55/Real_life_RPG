mod server_logic;
mod database_logic;
use crate::server_logic::server_logic::*;
use std::error::Error;

#[tokio::main]
async fn main()  -> Result<(), Box<dyn Error>> {
    run().await;
    Ok(())
}


