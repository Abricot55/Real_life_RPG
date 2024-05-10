use std::env;
use crate::database_logic::database_logic::{*};
use std::error::Error;
use std::fmt;

/*
*@brief Custom error
*/
#[derive(Debug)]
struct MyError {
    details: String
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

/**
 * @brief This function collect information from the terminal and call serach_for_function with these information
*/
pub fn run() {
    let command: Vec<String> = env::args().collect();
    let args: Vec<String> = command[1..].to_vec();
    search_for_function(&args);
}

fn get_document(args: &Vec<String>) -> Result<DocumentType, MyError>{
    //args[4] = document type, args[5]+ = value
    match args[4].to_lowercase().as_str(){
        "skill" => if args.len() >= 7{
            let mut skill = SkillType {
                _key: args[5].clone(),
                name: args[6].clone(),
            };
            return Ok(crate::database_logic::database_logic::DocumentType::Skill(skill));
        },
        "user" => if args.len() >= 11{
            match args[10].clone().parse::<i32>(){
                Ok(_) => if true{
                    let mut user = UserType {
                        _key: args[5].clone(),
                        name: args[6].clone(),
                        pseudo: args[7].clone(),
                        email: args[8].clone(),
                        birth_date: args[9].clone(),
                        level: args[10].clone().parse::<i32>().unwrap()
                    };
                    return Ok(crate::database_logic::database_logic::DocumentType::User(user))
                },
                Err(_) => return Err(MyError {details: "erreur lors de conversion string to int".to_string()}),
                _other => print!("what the hell")
            }
        },
        _other => print!("other document type, ")
    }
    return Err(MyError {details: "document invalide".to_string()});
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
                        create_new_collection(args[2].clone(), args[3].clone())//collection name, database name
                    },
                    "document" =>  if args.len() >= 5{ //collection name, database name, document type, value...
                        match get_document(&args){
                            Ok(doc) => add_document_to_collection(doc, args[2].clone(), args[3].clone()),
                            Err(e) => print!("{}", e)
                        }
                    },
                    "relation" => if args.len() >= 4{
                        create_new_relation(args[2].clone(),args[3].clone()) // relation name, database name
                    },
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