use crate::post::*;

use actix_web::{HttpRequest, Json};

use actix_web::Result;

pub fn get_post(req: HttpRequest) -> Result<Json<Post>> {
    let id: String = req.match_info().query("id")?;
    Ok(Json(Post::empty().title(&id)))
}

pub fn save_post(_item: Json<Post>) -> Json<UploadResp> {
    Json(UploadResp {
        location: String::from("abcdef"),
    })
}
