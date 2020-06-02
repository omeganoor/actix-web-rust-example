
use actix_web::{App, HttpServer};

mod config;

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref USERMAP: Mutex<HashMap<u32, &'static str>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };    
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    
    HttpServer::new(|| App::new()
        .service(config::app::index)
        .service(config::app::index_path))
    .bind("127.0.0.1:8080")?
    .run()
    .await
}