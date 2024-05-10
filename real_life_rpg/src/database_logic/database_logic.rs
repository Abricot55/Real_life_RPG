use std::collections::{hash_map, HashMap};

use arangors::{document::options::{InsertOptions, InsertOptionsBuilder, RemoveOptions}, uclient::reqwest::ReqwestClient, ArangoError, ClientError, Collection, Connection, Database};
use serde::{ Deserialize, Serialize};
use serde_json:: Result as JResult;

#[derive(Serialize, Deserialize)]
enum DocumentType{
    User(UserType),
    Skill(SkillType)
}
#[derive(Serialize, Deserialize)]
pub struct UserType{
    pub _key : String,
    pub name : String,
    pub pseudo : String,
    pub email : String,
    pub birth_date : String,
    pub level : i32
}
#[derive(Serialize, Deserialize)]
pub struct SkillType{
    pub _key : String,
    pub name : String
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

/**
 * @brief This function add a new document to a specific collection.
 * @param document -> The document that will be added to the collection.
 * @param collection_name -> The name of the collection the document will be added to.
 * @param database_name -> The name of the database the collection is a part of.
 */
#[tokio::main]
pub async fn add_document_to_collection(document : DocumentType, collection_name : String, database_name : String){
    let insert : InsertOptions = InsertOptions::default();
    match get_collection(collection_name, database_name){
        Ok(collec) => 
            match convert_doc_json(document){
                Ok(json_doc) => 
                    match collec.create_document(json_doc, insert).await{
                        Ok(doc) => print!("Document succesfully created"),
                        Err(e) => print!("Impossible to create the document")
                    }
                Err(e) => print!("The document is invalid")
            }
        Err(e) => print!("impossible to connect to the collection")
    }
}

/**
 * @brief This functionis used to retrieve a specific document in a collection.
 * @param key -> The unique key of the document.
 * @param collection_name -> The name of the collection in which the document exist.
 * @param database_name -> The name of the database in which the collection exist.
 * @return The document with the specified id.
 */
#[tokio::main]
async fn get_document_in_collection(key : String, collection_name : String, database_name : String) -> Result<arangors::Document<String>, ClientError>{
    match get_collection(collection_name, database_name){
        Ok(collec) => return collec.document::<String>(key.as_str()).await,
        Err(e) => return Err(e)
    }
}

/**
 * @brief This function is used to delete a document in a given collection.
 * @param key -> The unique key of the document that need to be deleted.
 * @param collection_name -> The name of the collection in which the document exist.
 * @param database_name -> The name of the database in which the collection exist.
 */
#[tokio::main]
async fn delete_document_in_collection(key : String, collection_name : String, database_name : String){
    let remove : RemoveOptions = RemoveOptions::default();
    match get_collection(collection_name, database_name){
        Ok(collec) => 
            match collec.remove_document::<String>(key.as_str(), remove, None).await{
                Ok(doc) => print!("Document succesfully deleted"),
                Err(e) => print!("Unable to delete the document")
            },
        Err(e) => print!("unable to connect to the collection")
    }
}

/**
 * @brief This function is used to update an already present document in a specified collection.
 * @detail this function will search for a document with the same key and delete it and add the new_document in the collection.
 * @param key -> the key of the document.
 * @param new_document -> The updated version of the document.
 * @param collection_name -> the name of the collection in which the document exist.
 * @param database_name -> the name of the database in which the collection exist.
 * 
 */
#[tokio::main]
async fn update_document_in_collection(key : String, new_document : DocumentType, collection_name : String, database_name : String){
    delete_document_in_collection(key, collection_name.clone(), database_name.clone())
    add_document_to_collection(new_document, collection_name, database_name)
}

/**
 * @brief This function convert a DocumentType to a String which is formatted like a jSon document.
 * @param document -> The document that will be converted.
 * return A result that contain either the String or an error.
 */
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