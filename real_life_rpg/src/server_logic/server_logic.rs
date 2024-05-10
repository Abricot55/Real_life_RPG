use std::env;
use crate::database_logic::database_logic::{*};
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
        match args[0].to_lowercase().as_str() {
            "add" => if args.len() >= 3{
                match args[1].to_lowercase().as_str(){
                    "database" => create_new_db(args[2].clone()),
                    "collection" => if args.len() >= 4{
                        create_new_collection(args[2].clone(), args[3].clone())
                    },
                    "document" =>  print!("add document {}", args[2].as_str()),
                    "relation" => print!("add relation {}", args[2].as_str()),
                    _other => print!("add other")
                }
            },
            "get" => if args.len() >= 2{
                match args[1].to_lowercase().as_str(){
                    "database" => print!("get database"),
                    "collection" => print!("get collection"),
                    "document" => print!("get document"),
                    "relation" => print!("get relation"),
                    _other => print!("get other")
                }
            },
            "update" => if args.len() >= 2{
                match args[1].to_lowercase().as_str(){
                    "database" => print!("update database"),
                    "collection" => print!("update collection"),
                    "document" => print!("update document"),
                    "relation" => print!("update relation"),
                    _other => print!("update other")
                }
            },
            "delete" => if args.len() >= 2{
                match args[1].to_lowercase().as_str(){
                    "document" => print!("delete document"),
                    _other => print!("delete other")
                }
            },
            _other => print!("nothing")
        }
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