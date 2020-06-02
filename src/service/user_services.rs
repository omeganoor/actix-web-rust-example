use crate::entity::user::*;
// use actix_web::{Result};


pub fn insert(new_user: User){
    User::insert(new_user);
}

pub fn find_all() -> Vec<User> {
    User::find_all()
}

pub fn find_by_id(id : i32) -> User {
    User::find_by_id(id)
}