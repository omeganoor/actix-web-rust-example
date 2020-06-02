
pub fn insert(new_person: PersonDTO, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Person::insert(new_person, &pool.get().unwrap()) {
        Ok(_) => Ok(())
   }
}

pub fn find_all() -> Result<Vec<Person>, ServiceError> {
    match Person::find_all() {
        Ok(person) => Ok(person),
    }
}