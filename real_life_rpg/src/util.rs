use regex::Regex;
use bcrypt::{hash, DEFAULT_COST};

pub fn is_valid_email(email: &str) -> bool {
    let pattern = r"^[\w.+-]+@\w+\.\w{2,}$";
    let regex = Regex::new(pattern).unwrap();
    regex.is_match(email)
}

pub fn hash_string(base_string : &str) -> String{
    return hash(base_string, DEFAULT_COST).unwrap();
}