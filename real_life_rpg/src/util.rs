use image::{DynamicImage, ImageFormat};
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

pub fn compress_image_png(image: DynamicImage) -> Result<DynamicImage, String>{
    match image.save_with_format("output.png", ImageFormat::Png){
        Ok(_) => Ok(image),
        Err(_) => Err("Couldn't compress image to png format".to_string()),
    }
}