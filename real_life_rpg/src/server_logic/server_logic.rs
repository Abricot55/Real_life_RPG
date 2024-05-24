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
 * @brief This function collect information concerning the request and call the good function accordingly.
*/
pub async fn run(method_name : &str, args : &Vec<String>) -> String{
    match method_name.to_lowercase().as_str(){
        "add" => return add_function(&args).await,
        "delete" => return delete_function(&args).await,
        "get" => return get_function(&args).await,
        "update" => return update_function(&args).await,
        _other => return "Echec".to_string()
    }
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
 * @brief This function is called when a add request is made to the server.
 * @param args -> A vector of string that contains the request to the server.
 * @return A String which indicate the state of the request.
 */
async fn add_function(args: &Vec<String>) -> String{
    if args.len() >= 3{
        match args[1].to_lowercase().as_str(){
            "database" => {
                create_new_db(args[2].clone()).await;
                return "Database Créée".to_string()}, //database name
            "collection" => {if args.len() >= 4{
                create_new_collection(args[2].clone(), args[3].clone()).await;
                return "Colledction Créée".to_string()}//collection name, database name
            else {
                return "commande invalide".to_string();
            }},
            "document" =>  { if args.len() >= 5{ //collection name, database name, document type, value...
                match get_document(&args, 4){
                    Ok(doc) => {
                        add_document_to_collection(doc, args[2].clone(), args[3].clone()).await;
                        return "Document Créé".to_string()},
                    Err(e) => return e.to_string()}
            }else {
                return "commande invalide".to_string();
            }},
            "relation" => { if args.len() >= 4{
                create_new_relation(args[2].clone(),args[3].clone()).await;
                return "Relation Créée".to_string() // relation name, database name
            }else {
                return "commande invalide".to_string();
            }},
            _other => return "add other".to_string()
        }
    }
    return "commande invalide".to_string();
}

/**
 * @brief Function called when the get request is sent to the server.
 * @param args -> A vector of string that contains the request to the server
 * @return A String which indicate the state of the request.
 */
async fn get_function(args: &Vec<String>) -> String{
    if args.len() >= 2{
        match args[1].to_lowercase().as_str(){
            //"database" => print!("get database"),
            "collection" => if args.len() >= 4{
                match get_collection(args[2].clone(), args[3].clone()).await{ //collection name, database name
                    Ok(_collection) => {
                        return "collection trouvée".to_string()
                        //if you do something with collection, remove '_', return it
                    },
                    Err(_) => return "collection non trouvée".to_string()
                }
            }else {
                return "commande invalide".to_string();
            },
            "document" => if args.len() >= 5{ //document key, collection name, database name
                match get_document_in_collection(args[2].clone(), args[3].clone(), args[4].clone()).await{ //document key, collection name, database name
                    Ok(_document) => {
                        return _document.to_string()
                        //if you do something with document, remove '_', return it
                    },
                    Err(_) => return "document non trouvé".to_string()
                }
            } else {
                return "commande invalide".to_string();
            }, 
            //"relation" => print!("get relation"),
            _other => return "get other".to_string()
        }
    }
    return "commande invalide".to_string();
}

/**
 * @brief Function called when the update request is sent to the server.
 * @param args -> A vector of string that contains the request to the server
 * @return A String which indicate the state of the request.
 */
async fn update_function(args: &Vec<String>) -> String{
    if args.len() >= 2{
        match args[1].to_lowercase().as_str(){
            //"database" => print!("update database"),
            //"collection" => print!("update collection"),
            "document" => if args.len() >= 6{ //document key, collection name, database name, docutment type, value...
                match get_document(&args, 5){
                    Ok(doc) => {
                        update_document_in_collection(args[2].clone(), doc, args[3].clone(), args[4].clone()).await;
                        return "Le document à été mis à jour!".to_string()},
                    Err(e) => return e.to_string()
                }
            } else{
                return "commande invalie".to_string();
            },
            //"relation" => print!("update relation"),
            _other => return "update other".to_string()
        }
    }
    return "commande invalide".to_string();

}

/**
 * @brief Function called when the delete request is sent to the server.
 * @param args -> A vector of string that contains the request to the server
 * @return A String which indicate the state of the request.
 */
async fn delete_function(args: &Vec<String>) -> String{
    if args.len() >= 2{
        match args[1].to_lowercase().as_str(){
            "document" => {if args.len() >= 5{ //document key, collection name, database name
                delete_document_in_collection(args[2].clone(), args[3].clone(), args[4].clone()).await;
                return "Document Supprimé".to_string()
            } else {
                return "commande invalide".to_string();
            }},
            _other => return "delete other".to_string()
        }
    }
    return "commande invalide".to_string();
}

/**
 * @brief module use to link tests to this librairy
 */
#[cfg(test)]
mod tests {
    use super::*;
    include!("tests_server_logic.rs");
}