use crate::hash_to_message;

use super::*;

/**
 * @brief This function add a new comment to the specified photo.
 * @param params -> A HashMap that must contain the key of the document and the id of the photo.
 * @return a Response with in its body the info concerning this request.
 */
pub async fn add_comment(
    params: HashMap<String, String>,
) -> Result<Response<Body>, warp::Rejection> {
    let photoinfo = match get_specific_photo_only_hashmap(params.clone()).await {
        Ok(value) => value,
        Err(e) => {
            return Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response())
        }
    };
    let mut photo = photoinfo.photo;
    let mut comment_new = match hash_to_message(params) {
        Ok(message) => message,
        Err(e) => {
            print!("{}", e);
            return Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response());
        }
    };
    comment_new.message_id = (photo.comments.len()) as i32;
    photo.comments.push(comment_new);
    update_photo_posseded(photoinfo.key, photo).await
}
