use super::*;


pub(super) fn other_photo_routes(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let add_like_route = warp::body::json().and_then(add_a_like);
    let sub_like_route = warp::body::json().and_then(sub_a_like);
    let add_share_route = warp::body::json().and_then(add_a_share);
    let update_title_route = warp::body::json().and_then(update_title);
    let update_desc_route = warp::body::json().and_then(update_description);
    let delete_title_route = warp::body::json().and_then(delete_title);
    let delete_description_route = warp::body::json().and_then(delete_decription);

    let post = warp::post().and(add_like_route).or(sub_like_route).or(add_share_route);
    let put = warp::put().and(update_title_route).or(update_desc_route);
    let delete = warp::delete().and(delete_description_route).or(delete_title_route);
    warp::path("comment").and(post.or(delete).or(put))
}

async fn update_title(params: HashMap<String, String>) -> Result<Response<Body>, warp::Rejection> {
    let photoinfo = match get_specific_photo_only_hashmap(params.clone()).await {
        Ok(value) => value,
        Err(e) => {
            return Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response())
        }
    };
    let mut photo = photoinfo.photo;
    let title_new = match params.get("title") {
        Some(value) => value.clone(),
        None => {
            return Ok(
                warp::reply::with_status("Need a new title", StatusCode::NOT_ACCEPTABLE)
                    .into_response(),
            )
        }
    };
    photo.title = title_new;
    update_photo_posseded(photoinfo.key, photo).await
}

async fn delete_title(params: HashMap<String, String>) -> Result<Response<Body>, warp::Rejection> {
    let photoinfo = match get_specific_photo_only_hashmap(params.clone()).await {
        Ok(value) => value,
        Err(e) => {
            return Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response())
        }
    };
    let mut photo = photoinfo.photo;
    photo.title = "".to_string();
    update_photo_posseded(photoinfo.key, photo).await
}

async fn update_description(params: HashMap<String, String>) -> Result<Response<Body>, warp::Rejection> {
    let photoinfo = match get_specific_photo_only_hashmap(params.clone()).await {
        Ok(value) => value,
        Err(e) => {
            return Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response())
        }
    };
    let mut photo = photoinfo.photo;
    let desc_new = match params.get("description") {
        Some(value) => value.clone(),
        None => {
            return Ok(
                warp::reply::with_status("Need a new description", StatusCode::NOT_ACCEPTABLE)
                    .into_response(),
            )
        }
    };
    photo.description = desc_new;
    update_photo_posseded(photoinfo.key, photo).await
}

async fn delete_decription(params: HashMap<String, String>) -> Result<Response<Body>, warp::Rejection> {
    let photoinfo = match get_specific_photo_only_hashmap(params.clone()).await {
        Ok(value) => value,
        Err(e) => {
            return Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response())
        }
    };
    let mut photo = photoinfo.photo;
    photo.description = "".to_string();
    update_photo_posseded(photoinfo.key, photo).await
}

async fn add_a_like(params: HashMap<String, String>)-> Result<Response<Body>, warp::Rejection>{
    let photoinfo = match get_specific_photo_only_hashmap(params.clone()).await {
        Ok(value) => value,
        Err(e) => {
            return Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response())
        }
    };
    let mut photo = photoinfo.photo;
    photo.likes += 1;
    update_photo_posseded(photoinfo.key, photo).await
}

async fn sub_a_like(params: HashMap<String, String>)-> Result<Response<Body>, warp::Rejection>{
    let photoinfo = match get_specific_photo_only_hashmap(params.clone()).await {
        Ok(value) => value,
        Err(e) => {
            return Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response())
        }
    };
    let mut photo = photoinfo.photo;
    photo.likes -= 1;
    update_photo_posseded(photoinfo.key, photo).await
}

async fn add_a_share(params: HashMap<String, String>)-> Result<Response<Body>, warp::Rejection>{
    let photoinfo = match get_specific_photo_only_hashmap(params.clone()).await {
        Ok(value) => value,
        Err(e) => {
            return Ok(warp::reply::with_status(e, StatusCode::NOT_ACCEPTABLE).into_response())
        }
    };
    let mut photo = photoinfo.photo;
    photo.shared += 1;
    update_photo_posseded(photoinfo.key, photo).await
}