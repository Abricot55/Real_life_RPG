
/**
 * @brief tests for the search_for_function function in the server_logic librairy.
 * Valid cases : 
 *      search_for_add : when the first element of the vector is add.
 *      search_for_other : when the first element of the vector is not relevant.
 */
#[cfg(test)]
mod tests_search_for_function{
    use super::*;

    #[test]
    fn search_for_add(){
        let vector_test : Vec<String> = vec![String::from("add")];
        search_for_function(&vector_test);

        let vector_test : Vec<String> = vec![String::from("aDd")];
        search_for_function(&vector_test);
    }

}