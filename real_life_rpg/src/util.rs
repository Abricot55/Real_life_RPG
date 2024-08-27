use std::collections::HashMap;

use bcrypt::{hash, DEFAULT_COST};
use hyper::{body::to_bytes, Body};
use image::{DynamicImage, ImageFormat};
use regex::Regex;
use serde_json::{Map, Value};

pub fn is_valid_email(email: &str) -> bool {
    let pattern = r"^[\w.+-]+@\w+\.\w{2,}$";
    let regex = Regex::new(pattern).unwrap();
    regex.is_match(email)
}

pub fn hash_string(base_string: &str) -> String {
    return hash(base_string, DEFAULT_COST).unwrap();
}

pub fn compress_image_png(image: DynamicImage) -> Result<DynamicImage, String> {
    match image.save_with_format("output.png", ImageFormat::Png) {
        Ok(_) => Ok(image),
        Err(_) => Err("Couldn't compress image to png format".to_string()),
    }
}

pub fn json_to_hashmap(doc: &str) -> Result<Map<String, Value>,String>{
    match serde_json::from_str(doc) {
        Ok(val) => return Ok(val),
        Err(_) => return Err("The json couldn't be parsed".to_string())
    }
}

pub fn weird_json_normal_str(json_str : String)->String{
    json_str.replace(r#"""#, "")
}

/**
 * @brief This function take the string of a body in a request and tranform it in hashmap.
 * @param The body of the request.
 * @return an option which can contain the hashmap if the conversion worked.
 */
pub async fn get_body_map(body: Body) -> Option<HashMap<String, Value>> {
    match to_bytes(body).await {
        Ok(body_bytes) => match String::from_utf8(body_bytes.to_vec()) {
            Ok(body_str) => match serde_json::from_str(&body_str) {
                Ok(body_map) => return Some(body_map),
                Err(_) => return None,
            },
            Err(_) => return None,
        },
        Err(_) => return None,
    }
}