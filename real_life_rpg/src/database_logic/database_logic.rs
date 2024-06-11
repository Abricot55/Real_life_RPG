use std::collections::HashMap;

use arangors::{
    document::options::RemoveOptions, uclient::reqwest::ReqwestClient, ClientError, Connection,
    Database,
};
use arangors::{AqlQuery, Document};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub enum DocumentType {
    User(UserType),
    Skill(SkillType),
}

#[derive(Serialize, Deserialize)]
pub enum RelationType {
    Uu(RelationUserUserType),
    Us(RelationUserSkillType),
}

#[derive(Serialize, Deserialize)]
pub struct UserType {
    pub name: String,
    pub pseudo: String,
    pub email: String,
    pub birth_date: String,
    pub level: i32,
}
#[derive(Serialize, Deserialize)]
pub struct SkillType {
    pub _key: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RelationUserUserType {
    pub _key: String,
    pub from: String,
    pub to: String,
    pub force: Option<i32>,
    pub time: Option<i32>,
    pub relation_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct RelationUserSkillType {
    pub _key: String,
    pub from: String,
    pub to: String,
    pub level: i32,
    pub challenge_completed: i32,
    pub title: String,
}

/**
 * @brief This function etablish connection with the port 8529.
 * @return A result with the connection if it worked and a error if it didn't.
 */
pub async fn connect_to_connection() -> Result<Connection, ClientError> {
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
pub async fn create_new_db(name: String) {
    match connect_to_connection().await {
        Ok(connection) => match connection.create_database(name.as_str()).await {
            Ok(_) => print!("Database {} succesfully created", name),
            Err(_) => print!("Impossible to create this database"),
        },
        Err(_) => print!("Impossible to connect"),
    }
}

/**
 * @brief This function etablish connection with the database on port 8529.
 * @param name -> The name of the database we want to access.
 * @return A result wich is either a error or another result containing the value if it worked.
 */
pub async fn connect_to_db(name: String) -> Result<Database<ReqwestClient>, ClientError> {
    match connect_to_connection().await {
        Ok(connection) => return connection.db(name.as_str()).await,
        Err(e) => return Err(e),
    }
}

/**
 * @brief This function create a new collection in the current database.
 * @param name -> the name of the collection we want to create.
 * @return A result which is either an error or the new collection.
 */

pub async fn create_new_collection(name: String, database_name: String) {
    match connect_to_db(database_name).await {
        Ok(db) => match db.create_collection(name.as_str()).await {
            Ok(_) => print!("Collection {} succesfully created in database", name),
            Err(_) => print!("Impossible to create the collection"),
        },
        Err(_) => print!("Impossible to connect to the database"),
    }
}

/**
 * @brief This function access a collection or relation in the current database.
 * @param name -> the name of the collection or relation we want to access.
 * @return A result which is either an error or the collection/relation.
 */
pub async fn get_collection(
    name: String,
    database_name: String,
) -> Result<arangors::Collection<ReqwestClient>, ClientError> {
    match connect_to_db(database_name).await {
        Ok(db) => return db.collection(name.as_str()).await,
        Err(e) => return Err(e),
    }
}

/**
 * @brief This function create a new relation collection in the database.
 * @param name -> The name of the new relation.
 * @return A result which is either an error or the new relation
 */

pub async fn create_new_relation(name: String, database_name: String) {
    match connect_to_db(database_name).await {
        Ok(db) => match db.create_edge_collection(name.as_str()).await {
            Ok(_) => print!("Collection {} succesfully created in database", name),
            Err(_) => print!("Impossible to create the collection"),
        },
        Err(_) => print!("Impossible to connect to the database"),
    }
}

/**
 * @brief This function add a new document to a specific collection.
 * @param document -> The document that will be added to the collection.
 * @param collection_name -> The name of the collection the document will be added to.
 * @param database_name -> The name of the database the collection is a part of.
 */

pub async fn add_document_to_collection(
    document: DocumentType,
    collection_name: String,
    database_name: String,
) -> Result<String, String> {
    match convert_doc_json(document) {
        Ok(json_doc) => {
            let aql = format!("INSERT {} INTO {}", json_doc.to_string(), collection_name);
            let query: AqlQuery = AqlQuery::builder().query(&aql).build();
            match connect_to_db(database_name).await {
                Ok(db) => match db.aql_query::<Value>(query).await {
                    Ok(_) => Ok("Document successfully added to the collection".to_string()),
                    Err(_) => Ok("The document couldn't be added to the collection".to_string()),
                },
                Err(_) => Ok("Couldn't connect to the database".to_string()),
            }
        }
        Err(_) => Ok("The document is invalid".to_string()),
    }
}

/**
 * @brief This functionis used to retrieve a specific document in a collection.
 * @param key -> The unique key of the document.
 * @param collection_name -> The name of the collection in which the document exist.
 * @param database_name -> The name of the database in which the collection exist.
 * @return The document with the specified id.
 */

pub async fn get_document_in_collection(
    key: String,
    collection_name: String,
    database_name: String,
) -> Result<Value, String> {
    match get_collection(collection_name, database_name).await {
        Ok(collec) => match collec.document::<Value>(key.as_str()).await {
            Ok(doc) => match serde_json::to_value(doc.to_string().as_str()) {
                Ok(json_doc) => return Ok(json_doc),
                Err(_) => return Err("Conversion en Json impossible".to_string()),
            },
            Err(e) => return Err("Immposssible de trouver le document".to_string()),
        },
        Err(e) => return Err("Impossible de trouver la collection".to_string()),
    }
}

/**
 * @brief This function is used to delete a document in a given collection.
 * @param key -> The unique key of the document that need to be deleted.
 * @param collection_name -> The name of the collection in which the document exist.
 * @param database_name -> The name of the database in which the collection exist.
 */

pub async fn delete_document_in_collection(
    key: String,
    collection_name: String,
    database_name: String,
) {
    let remove: RemoveOptions = RemoveOptions::default();
    match get_collection(collection_name, database_name).await {
        Ok(collec) => match collec
            .remove_document::<String>(key.as_str(), remove, None)
            .await
        {
            Ok(_) => print!("Document succesfully deleted"),
            Err(_) => print!("Unable to delete the document"),
        },
        Err(_) => print!("unable to connect to the collection"),
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

pub async fn update_document_in_collection(
    key: String,
    new_document: DocumentType,
    collection_name: String,
    database_name: String,
) {
    delete_document_in_collection(key, collection_name.clone(), database_name.clone()).await;
    add_document_to_collection(new_document, collection_name, database_name).await;
}

/**
 * @brief This function convert a DocumentType to a String which is formatted like a jSon document.
 * @param document -> The document that will be converted.
 * return A result that contain either the String or an error.
 */
fn convert_doc_json(document: DocumentType) -> Result<String, String> {
    match serde_json::to_value(&document) {
        Ok(document_value) => match document_value.as_object().unwrap().iter().next() {
            Some((_key, value)) => return Ok(serde_json::to_string(&value).unwrap()),
            None => return Err("The document is empty".to_string()),
        },
        Err(_) => return Err("The document is invalid".to_string()),
    }
}

/**
 * @brief This function search in the database a document which correspond to the search criteria passed as parameter.
 * @param fields -> A hashMap<String, Value> that contains the different fields that the search will be based on.
 * @param view -> The view that contains the documents in the database.
 * @param database_name -> The name of the database to which the aql request will be done.
 * @return A Result that contain either the vector of found document or a String if something went wrong.
 */
pub async fn search_field(
    fields: HashMap<String, Value>,
    view: String,
    database_name: String,
) -> Result<Vec<Document<Value>>, String> {
    if !fields.is_empty() {
        let mut base_query = format!(
            r#"
        FOR doc IN {}
        SEARCH "#,
            view
        );

        let mut first = true;
        for (key, value) in fields {
            if !first {
                base_query.push_str(" AND ");
            }
            first = false;
            match value {
                Value::String(s) => base_query.push_str(&format!("doc.{} == '{}' ", key, s)),
                _ => base_query.push_str(&format!("doc.{} == {} ", key, value)),
            };
        }

        base_query.push_str(" RETURN doc");

        let query = AqlQuery::builder().query(&base_query).build();

        match connect_to_db(database_name).await {
            Ok(db) => match db.aql_query::<Document<Value>>(query).await {
                Ok(documents) => Ok(documents),
                Err(e) => Err(format!("The query didn't work: {:?}", e)),
            },
            Err(e) => Err(format!("Couldn't connect to db: {:?}", e)),
        }
    } else {
        Err("There is no search field!".to_string())
    }
}


/**
 * @brief This function search the fields and check for the substring. It then shows the 3 most relevant documents.
 * @param fields -> The fields on which the search is based.
 * @param view -> The view that contains the documents in the database.
 * @param database_name -> The name of the database to which the aql request will be sent.
 * @return A Result that contain either the vector of found document or a String if something went wrong.
 */
pub async fn relevant_search_field(
    fields: HashMap<String, Value>,
    view: String,
    database_name: String,
) -> Result<Vec<Document<Value>>, String> {
    if !fields.is_empty() {
        let mut base_query = format!(
            r#"
        FOR doc IN {}
        SEARCH "#,
            view
        );

        let mut first = true;
        for (key, value) in fields {
            if !first {
                base_query.push_str(" AND ");
            }
            first = false;

            
            match value {
                Value::String(s) => base_query.push_str(&format!("LIKE(doc.{}, '%{}%') ", key, s)),
                _ => base_query.push_str(&format!("LIKE(doc.{}, {}) ", key, value)),
            };
        }

        base_query.push_str(
            "  SORT BM25(doc) DESC LIMIT 3 RETURN doc",
        );

        let query = AqlQuery::builder().query(&base_query).build();

        match connect_to_db(database_name).await {
            Ok(db) => match db.aql_query::<Document<Value>>(query).await {
                Ok(documents) => Ok(documents),
                Err(e) => Err(format!("The query didn't work: {:?}", e)),
            },
            Err(e) => Err(format!("Couldn't connect to db: {:?}", e)),
        }
    } else {
        Err("There is no search field!".to_string())
    }
}

/**
 * @brief module use to link tests to this librairy
 */
#[cfg(test)]
mod tests {
    include!("tests_database_logic.rs");
}
