use std::{any::Any, collections::HashMap};

use bcrypt::{hash, DEFAULT_COST};
use image::{DynamicImage, ImageFormat};
use regex::Regex;
use serde::de::value;
use serde_json::{Map, Value};
use warp::filters::body::json;

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
        Err(e) => return Err("The json couldn't be parsed".to_string())
    }
}

pub fn weird_json_normal_str(json_str : String)->String{
    json_str.replace(r#"""#, "")
}