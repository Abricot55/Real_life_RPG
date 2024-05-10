use std::collections::{hash_map, HashMap};

use arangors::{document::options::{InsertOptions, InsertOptionsBuilder}, uclient::reqwest::ReqwestClient, ArangoError, ClientError, Collection, Connection, Database};
use serde::{ Deserialize, Serialize};
use serde_json:: Result as JResult;

#[derive(Serialize, Deserialize)]
enum DocumentType{
    User(UserType),
    Skill(SkillType)
}
#[derive(Serialize, Deserialize)]
pub struct UserType{
    name : String,
    pseudo : String,
    email : String,
    birth_date : String,
    level : i32
}
#[derive(Serialize, Deserialize)]
pub struct SkillType{
    name : String
}

/**
 * @brief This function etablish connection with the port 8529.
 * @return A result with the connection if it worked and a error if it didn't.
 */
#[tokio::main]
async fn connect_to_connection() -> Result<Connection, ClientError>{
    let url = "http://localhost:8529";
    let username = "root";
    let password = "t53ee&&v9vt67";
    return Connection::establish_jwt(url, &username, &password).await;
}

/**
 * @brief This function create a new database on the connexion on port 8529.
 * @param name -> The name of the database we are creating.
 * @return A result wich is either a error or another result containing the value of the database if the creation worked.
 */
#[tokio::main]
pub async fn create_new_db( name : String){
    match connect_to_connection(){
        Ok(connection) => 
            match connection.create_database(name.as_str()).await{
                Ok(db) => print!("Database {} succesfully created", name),
                Err(e) => print!("Impossible to create this database")
            },
        Err(e) => print!("Impossible to connect")
    }
}

/**
 * @brief This function etablish connection with the database on port 8529.
 * @param name -> The name of the database we want to access.
 * @return A result wich is either a error or another result containing the value if it worked.
 */
#[tokio::main]
async fn connect_to_db( name : String) -> Result<Database<ReqwestClient>, ClientError>{
    match connect_to_connection(){
        Ok(connection) => return connection.db(name.as_str()).await,
        Err(e) => return Err(e)
    }
}


/**
 * @brief This function create a new collection in the current database.
 * @param name -> the name of the collection we want to create.
 * @return A result which is either an error or the new collection.
 */
#[tokio::main]
pub async fn create_new_collection( name : String, database_name : String){
    match connect_to_db(database_name){
        Ok(db) => 
            match db.create_collection(name.as_str()).await{
                Ok(collec) => print!("Collection {} succesfully created in database", name),
                Err(e) => print!("Impossible to create the collection")
            }
        Err(e) => print!("Impossible to connect to the database")
    }
}

/**
 * @brief This function access a collection or relation in the current database.
 * @param name -> the name of the collection or relation we want to access.
 * @return A result which is either an error or the collection/relation.
 */
#[tokio::main]
async fn get_collection( name : String, database_name : String) -> Result<arangors::Collection<ReqwestClient>, ClientError>{
    match connect_to_db(database_name){
        Ok(db) => return db.collection(name.as_str()).await,
        Err(e) => return Err(e)
    }
}

/**
 * @brief This function create a new relation collection in the database.
 * @param name -> The name of the new relation.
 * @return A result which is either an error or the new relation
 */
#[tokio::main]
async fn create_new_relation(name : String, database_name : String){
    match connect_to_db(database_name){
        Ok(db) => 
            match db.create_edge_collection(name.as_str()).await{
                Ok(collec) => print!("Collection {} succesfully created in database", name),
                Err(e) => print!("Impossible to create the collection")
            }
        Err(e) => print!("Impossible to connect to the database")
    }
}


/*#[tokio::main]
pub async fn add_document_to_collection(document : DocumentType){
    let insert : InsertOptions = InsertOptions::default();
    match convert_doc_json(document){
        Ok(json_doc) => 
            match self.collec.create_document(json_doc,insert).await{
                Ok(doc) => return Ok(doc),
                Err(e) => return Err("Impossible to add the document")
            },
        Err(e) => return Err("The document is invalid")
    }
}*/

    #[tokio::main]
    async fn get_document_in_collection(id : i32){

    }

    #[tokio::main]
    async fn delete_document_in_collection(id : i32){

    }

    #[tokio::main]
    async fn update_document_in_collection(id : i32){

    }

    #[tokio::main]
    async fn add_content_to_realtion(id : i32){

    }

    #[tokio::main]
    async fn del_content_to_relation(id : i32){

    }


fn convert_doc_json(document : DocumentType) -> JResult<String>{
    return Ok(serde_json::to_string(&document)?);
}




/**
 * @brief module use to link tests to this librairy
 */
#[cfg(test)]
mod tests {
    use super::*;
    include!("tests_database_logic.rs");
}