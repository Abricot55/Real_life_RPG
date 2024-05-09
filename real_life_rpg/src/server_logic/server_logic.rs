use std::env;
use crate::database_logic::database_logic::{connect_to_connection,*};
/**
 * @brief This function collect information from the terminal and call serach_for_function with these information
*/
pub fn run() {
    let command: Vec<String> = env::args().collect();
    let args: Vec<String> = command[1..].to_vec();
    search_for_function(&args);
}

/**
 * @brief This function research what to do depending on the first element of the vector passed in argument.
 * @param args -> a vector of string
 */
fn search_for_function(args: &Vec<String>) {
    if !args.is_empty() {


        ///as/da/sda/sd//asd/
        match args[0].to_lowercase().as_str() {
            "add" => ptit_truc_test(),
            _other => print!("nothing"),
        }
    }
}

fn ptit_truc_test(){
    let mut conn_object: Option<ConnectionEstablished> = None;
    match connect_to_connection(){
        Ok(connection) => conn_object = Some(ConnectionEstablished{conn : connection}),
        Err(e) => print!("oups!")
    }
    match conn_object {
        Some(value) => 
            match value.create_new_db("new_db".to_string()) {
                Ok(db) => print!("succesfully created"),
                Err(e) => print!("impossible")
            }
        None => print!("oups!")
    }
}

/**
 * @brief module use to link tests to this librairy
 */
#[cfg(test)]
mod tests {
    use super::*;
    include!("tests_server_logic.rs");
}