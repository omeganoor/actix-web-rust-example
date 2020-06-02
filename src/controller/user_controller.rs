use crate::entity::*;
use crate::service::*;
use crate::constant::*;
use actix_web::{get, post, web, HttpResponse};

#[post("")]
pub async fn create_user(new_person: web::Json<user::User>) -> HttpResponse {
    user_services::insert(new_person.0);
    HttpResponse::Ok().body("created".to_string())
}

#[get("")]
pub async fn find_all() -> HttpResponse {
    let point = user::User{id:1, name:"jason".to_string(),  email:"jason@json.com".to_string()};

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: user::User = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);

    let result = user_services::find_all();
    println!("deserialized = {:?}", result);
    HttpResponse::Ok().json(response::ResponseBody::new(response_code::MESSAGE_OK, result))
}

#[get("/{id}")]
pub async fn find_by_id(info: web::Path<i32>) -> HttpResponse {
    let id = info;
    let result = user_services::find_by_id(*id);
    HttpResponse::Ok().json(response::ResponseBody::new(response_code::MESSAGE_OK, result))
}