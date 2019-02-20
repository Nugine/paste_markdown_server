use crate::env::short_cryptor as sc;
use crate::post::*;
use crate::store::PostStore;
use actix_web::error::{ErrorBadRequest, ErrorNotFound};
use actix_web::{Error, HttpRequest, HttpResponse, Json, Result};
use failure::Fail;
use log::info;
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub store: Arc<Mutex<PostStore>>,
}

#[derive(Fail, Debug)]
enum HandlerError {
    #[fail(display = "Not Found")]
    NotFound,
    #[fail(display = "Bad Request")]
    BadRequest,
}
impl HandlerError {
    fn to_not_found<T>(_err: T) -> HttpResponse {
        HttpResponse::from_error(Error::from(ErrorNotFound(HandlerError::NotFound)))
    }

    fn to_bad_request<T>(_err: T) -> HttpResponse {
        HttpResponse::from_error(Error::from(ErrorBadRequest(HandlerError::BadRequest)))
    }
}

pub fn get_post(req: &HttpRequest<AppState>) -> HttpResponse {
    let time: i64 = match req
        .match_info()
        .query::<String>("key")
        .ok()
        .and_then(|k| sc.decrypt_url_component(&k).ok())
        .and_then(|u| String::from_utf8(u).ok())
        .and_then(|s| s.parse().ok())
    {
        None => return HandlerError::to_bad_request(()),
        Some(t) => t,
    };

    let store_clone = req.state().store.clone();
    let mut store = store_clone.lock().unwrap();
    store.clean();

    let post_str = match store.find_by_time(time) {
        None => return HandlerError::to_not_found(()),
        Some(s) => s,
    };

    info!("GET  time = {}, store_size = {}", time, store.size());
    HttpResponse::Ok().body(post_str)
}

pub fn save_post((item, req): (Json<Post>, HttpRequest<AppState>)) -> Result<Json<UploadResp>> {
    let store_clone = req.state().store.clone();
    let mut store = store_clone.lock().unwrap();
    store.clean();

    match store.save(item.0) {
        None => Err(Error::from(ErrorNotFound(HandlerError::NotFound))),
        Some(t) => {
            info!("SAVE time = {}, store_size = {}", t, store.size());
            Ok(Json(UploadResp {
                location: sc.encrypt_to_url_component(&format!("{}", t)),
            }))
        }
    }
}
