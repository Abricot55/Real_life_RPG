use std::env;
use crate::database_logic::database_logic::*;
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
            "add" => print!("ADD"),
            _other => print!("nothing"),
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