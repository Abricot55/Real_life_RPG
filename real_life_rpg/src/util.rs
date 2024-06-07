use regex::Regex;


pub fn is_valid_email(email: &str) -> bool {
    let pattern = r"^[\w.+-]+@\w+\.\w{2,}$";
    let regex = Regex::new(pattern).unwrap();
    regex.is_match(email)
}