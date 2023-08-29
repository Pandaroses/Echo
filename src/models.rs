use bcrypt::DEFAULT_COST;
use diesel::prelude::*;
use crate::schemas::{users , sessions};

#[derive(Queryable , QueryableByName , Insertable , Debug , Clone , Selectable)]
#[diesel(table_name = users)]
pub struct User {
    pub id : String,
    pub first_name : String,
    pub last_name : String,
    pub email : String,
    pub password : String
}
impl User{
    pub fn new(email : String , password : String, first_name : String , last_name : String) -> std::io::Result<Self>{
        Ok(
            Self {
                id : uuid::Uuid::new_v4().to_string(),
                email,
                first_name,
                last_name,
                password : bcrypt::hash(password, DEFAULT_COST).unwrap(),
            })
    }
}

#[derive(Queryable , QueryableByName , Insertable , Debug , Clone , Associations , Selectable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = sessions)]
pub struct Session {
    pub id : String,
    pub user_id : String
}

impl Session {
    pub fn new(user_id : String) -> Self {
        Self {
            user_id,
            id : uuid::Uuid::new_v4().to_string()
        }
    }
}