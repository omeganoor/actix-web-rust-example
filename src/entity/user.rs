#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub address: String,
    pub email: String,
}

impl Person {
    pub fn find_all() -> Vec<User> {
        let mut map = USERMAP.lock().unwrap();
        for (id, user) in &map {
            println!("id = {:?}, user = {:?}", id, user);
        }
    }

    pub fn insert(new_user: User) -> {
        let mut map = USERMAP.lock().unwrap();
        map.insert(map.len(), User);
    }
}