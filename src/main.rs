mod handler;
mod post;
use crate::handler::*;
use actix_web::{http, server, App};

const ADDR: &'static str = "127.0.0.1:8080";

fn main() {
    println!("server running at {}", ADDR);

    server::new(|| {
        App::new()
            .route("/post/{id}", http::Method::GET, get_post)
            .route("/post", http::Method::POST, save_post)
    })
    .bind(ADDR)
    .unwrap()
    .run()
}
