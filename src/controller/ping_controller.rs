use actix_web::{get, HttpResponse};

#[get("/ping")]
fn ping() -> HttpResponse {
    HttpResponse::Ok()
        .body("ok ping!!!".to_string())
}

