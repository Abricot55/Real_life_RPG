use arangors::{uclient::reqwest::ReqwestClient, ArangoError, ClientError, Connection, Database};

/**
 * @brief struct used to simulate an object connection.
 */
pub struct ConnectionEstablished{
    pub(crate) conn : Connection
}

/**
 * @brief struct used to simulate an object database
 */
pub struct DatabaseConnected{
    datab : Database<ReqwestClient>
}

/**
 * @brief This function etablish connection with the port 8529.
 * @return A result with the connection if it worked and a error if it didn't.
 */
#[tokio::main]
pub async fn connect_to_connection() -> Result<Connection, ClientError>{
    let url = "http://localhost:8529";
    let username = "root";
    let password = "t53ee&&v9vt67";
    return Connection::establish_jwt(url, &username, &password).await;
}

/**
 * @brief the following functions can only be used if a struct ConnectionEstablished is created.
 */
impl ConnectionEstablished{

    /**
     * @brief This function create a new database on the connexion on port 8529.
     * @param name -> The name of the database we are creating.
     * @return A result wich is either a error or another result containing the value of the database if the creation worked.
     */
    #[tokio::main]
    pub async fn create_new_db(&self, name : String) -> Result<Database<ReqwestClient>, ClientError>{
        self.conn.create_database(name.as_str()).await
    }

    /**
     * @brief This function etablish connection with the database on port 8529.
     * @param name -> The name of the database we want to access.
     * @return A result wich is either a error or another result containing the value if it worked.
     */
    #[tokio::main]
    async fn connect_to_db(&self, name : String) -> Result<Database<ReqwestClient>, ClientError>{
        self.conn.db(name.as_str()).await
    }
}

/**
 * @brief The following functions can only be used if a struct DatabaseConnected is created.
 */
impl DatabaseConnected{

    #[tokio::main]
    async fn create_new_collection(&self, name : String){

    }

    #[tokio::main]
    async fn get_collection(&self, name : String){

    }

    #[tokio::main]
    async fn add_document_to_collection(&self){

    }

    #[tokio::main]
    async fn get_document_in_collection(&self, id : i32){

    }

    #[tokio::main]
    async fn delete_document_in_collection(&self, id : i32){

    }

    #[tokio::main]
    async fn update_document_in_collection(&self, id : i32){

    }

    #[tokio::main]
    async fn create_new_relation(&self){

    }

    #[tokio::main]
    async fn add_content_to_realtion(&self, id : i32){

    }
}
/**
 * @brief module use to link tests to this librairy
 */
#[cfg(test)]
mod tests {
    use super::*;
    include!("tests_database_logic.rs");
}