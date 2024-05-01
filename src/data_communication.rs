use arangors::Connection;

#[tokio::main]
pub async fn get_database(){
    let url = "http://localhost:8529";
    let conn = Connection::establish_jwt(url, "root", "openSesame").await.unwrap();
    let db = conn.db("test_db").await.unwrap();
    let collection = db.collection("test_collection").await.unwrap();
    println!("{:?}", collection);
}