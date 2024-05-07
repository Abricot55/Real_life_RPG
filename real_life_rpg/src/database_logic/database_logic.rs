use arangors::Connection;


fn create_new_collection(name : String){

}

fn get_collection(){

}

fn add_document_to_collection(){

}

fn get_document_in_collection(){

}

fn delete_document_in_collection(){
    
}

fn update_document_in_collection(){

}

fn create_new_relation(){

}

fn add_content_to_realtion(){

}

/**
 * @brief module use to link tests to this librairy
 */
#[cfg(test)]
mod tests {
    use super::*;
    include!("tests_database_logic.rs");
}