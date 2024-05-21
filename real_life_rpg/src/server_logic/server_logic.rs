use std::env;
use crate::database_logic::database_logic::{*};
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
pub async fn run() {
    let command: Vec<String> = env::args().collect();
    let args: Vec<String> = command[1..].to_vec();
    search_for_function(&args).await;
}

fn get_document(args: &Vec<String>, index_value: i32) -> Result<DocumentType, MyError>{
    //args[index_value] = document type, args[index_value]+ = value
    match args[index_value as usize].to_lowercase().as_str(){
        "skill" => if args.len() >= (index_value + 3) as usize{
            let skill = SkillType {
                _key: args[(index_value+1) as usize].clone(),
                name: args[(index_value+2) as usize].clone(),
            };
            return Ok(crate::database_logic::database_logic::DocumentType::Skill(skill));
        } else {
            print!("commande invalide")
        },
        "user" => if args.len() >= (index_value + 7) as usize{
            match args[(index_value+6) as usize].clone().parse::<i32>(){
                Ok(_) => if true{
                    let user = UserType {
                        _key: args[(index_value+1) as usize].clone(),
                        name: args[(index_value+2) as usize].clone(),
                        pseudo: args[(index_value+3) as usize].clone(),
                        email: args[(index_value+4) as usize].clone(),
                        birth_date: args[(index_value+5) as usize].clone(),
                        level: args[(index_value+6) as usize].clone().parse::<i32>().unwrap()
                    };
                    return Ok(crate::database_logic::database_logic::DocumentType::User(user))
                },
                Err(_) => return Err(MyError {details: "erreur lors de conversion string to int".to_string()})
            }
        } else {
            print!("commande invalide")
        },
        _other => print!("other document type, ")
    }
    return Err(MyError {details: "document invalide".to_string()});
}

/**
 * @brief This function research what to do depending on the first element of the vector passed in argument.
 * @param args -> a vector of string
 */
async fn search_for_function(args: &Vec<String>){
    if !args.is_empty() {
        match args[0].to_lowercase().as_str() {
            "add" => if args.len() >= 3{
                match args[1].to_lowercase().as_str(){
                    "database" => create_new_db(args[2].clone()).await, //database name
                    "collection" => if args.len() >= 4{
                        create_new_collection(args[2].clone(), args[3].clone()).await//collection name, database name
                    }else {
                        print!("commande invalide")
                    },
                    "document" =>  if args.len() >= 5{ //collection name, database name, document type, value...
                        match get_document(&args, 4){
                            Ok(doc) => add_document_to_collection(doc, args[2].clone(), args[3].clone()).await,
                            Err(e) => print!("{}", e)
                        }
                    }else {
                        print!("commande invalide")
                    },
                    "relation" => if args.len() >= 4{
                        create_new_relation(args[2].clone(),args[3].clone()).await // relation name, database name
                    }else {
                        print!("commande invalide")
                    },
                    _other => print!("add other")
                }
            }else {
                print!("commande invalide")
            },
            "get" => if args.len() >= 2{
                match args[1].to_lowercase().as_str(){
                    //"database" => print!("get database"),
                    "collection" => if args.len() >= 4{
                        match get_collection(args[2].clone(), args[3].clone()).await{ //collection name, database name
                            Ok(_collection) => {
                                print!("collection trouvée")
                                //if you do something with collection, remove '_', return it
                            },
                            Err(_) => print!("collection non trouvée")
                        }
                    }else {
                        print!("commande invalide")
                    },
                    "document" => if args.len() >= 5{ //document key, collection name, database name
                        match get_document_in_collection(args[2].clone(), args[3].clone(), args[4].clone()).await{ //document key, collection name, database name
                            Ok(_document) => {
                                print!("{}", _document.to_string())
                                //if you do something with document, remove '_', return it
                            },
                            Err(_) => print!("document non trouvé")
                        }
                    } else {
                        print!("commande invalide")
                    }, 
                    //"relation" => print!("get relation"),
                    _other => print!("get other")
                }
            }else {
                print!("commande invalide")
            },
            "update" => if args.len() >= 2{
                match args[1].to_lowercase().as_str(){
                    //"database" => print!("update database"),
                    //"collection" => print!("update collection"),
                    "document" => if args.len() >= 6{ //document key, collection name, database name, docutment type, value...
                        match get_document(&args, 5){
                            Ok(doc) => update_document_in_collection(args[2].clone(), doc, args[3].clone(), args[4].clone()).await,
                            Err(e) => print!("{}", e)
                        }
                    } else{
                        print!("commande invalie")
                    },
                    //"relation" => print!("update relation"),
                    _other => print!("update other")
                }
            }else {
                print!("commande invalide")
            },
            "delete" => if args.len() >= 2{
                match args[1].to_lowercase().as_str(){
                    "document" => if args.len() >= 5{ //document key, collection name, database name
                        delete_document_in_collection(args[2].clone(), args[3].clone(), args[4].clone()).await
                    } else {
                        print!("commande invalide")
                    },
                    _other => print!("delete other")
                }
            }else {
                print!("commande invalide")
            },
            _other => print!("nothing")
        }
    }else {
        print!("commande invalide")
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