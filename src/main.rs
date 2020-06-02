#[macro_use]
extern crate lazy_static;

use actix_web::{App, HttpServer};

mod config;
mod constant;
mod controller;
mod entity;
mod repository;
mod service;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new(|| App::new()
        // .service(config::app::index)
        // .service(config::app::index_path)
        .configure(config::user::user_config_services))
    .bind("127.0.0.1:8080")?
    .run()
    .await
}