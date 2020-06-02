use actix_web::{web, HttpResponse, Result};

mod constant;
mod entity;
mod service;

pub async fn insert(new_person: web::Json<entity::user>) -> Result<HttpResponse> {
    match service::user_services::insert(new_person.0) {
        Ok(()) => Ok(HttpResponse::Created().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

pub async fn find_all() -> Result<HttpResponse> {
    match service::user_services::find_all() {
        Ok(user) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, user))),
        Err(err) => Ok(err.response()),
    }
}