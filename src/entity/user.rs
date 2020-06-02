use serde::{Serialize, Deserialize};

use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref USERMAP: Mutex<HashMap<usize, User>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn find_all() -> Vec<User> {
        println!("get all user");
        let mut vec = Vec::new();
        let map = USERMAP.lock().unwrap();
        println!("size = {}", map.len().to_string());
        for (_,val) in map.iter() {
            let res = User{id:val.id, name:val.name.to_string(),  email:val.email.to_string()};
            vec.push(res);
        }
        println!("user_list : {:?}", vec);
        vec
    }

    pub fn find_by_id(id : i32) -> User {
        println!("get user by id");
        let map = USERMAP.lock().unwrap();
        println!("id = {}", id.to_string());
        let index = id as usize;
        let val = map.get(&index).unwrap();
        println!("value = {:?}", val);
        User{id:val.id, name:val.name.to_string(),  email:val.email.to_string()}
    }

    pub fn insert(new_user: User){
        println!("insert new user");
        let mut map = USERMAP.lock().unwrap();
        let id = map.len();
        let mut user_name = new_user;
        user_name.id = id as i32;
        map.insert(id, user_name);
        println!("size = {}, user_map = {:?}", map.len().to_string(), map);

    }
}