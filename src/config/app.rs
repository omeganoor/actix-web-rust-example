use actix_web::{get, web, Responder};

#[get("/{id}/{name}")]
pub async fn index_path(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[get("/")]
pub async fn index(var: web::Path<()>) -> impl Responder {
    format!("Hello Guys!!!!")
}