mod env;
mod handler;
mod post;
mod store;

use crate::env::ADDR;
use crate::handler::*;
use crate::store::PostStore;
use actix_web::{http, server, App};
use dotenv::dotenv;
use std::cell::RefCell;

fn main() {
    dotenv().ok();
    env_logger::init();

    server::new(|| {
        App::with_state(AppState {
            store_cell: RefCell::new(PostStore::new()),
        })
        .resource("/post/{key}", |r| r.method(http::Method::GET).f(get_post))
        .resource("/post", |r| {
            r.method(http::Method::POST)
                .with_config(save_post, |((cfg, _),)| {
                    cfg.limit(20 * 1024);
                })
        })
    })
    .bind(&ADDR.clone())
    .unwrap()
    .run()
}
