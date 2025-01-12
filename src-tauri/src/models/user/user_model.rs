use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub random: String,
    pub active_code: String,
    pub validate_date: String,
    pub expire_date: String,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = crate::models::user::schema::users)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub random: String,
    pub active_code: String,
    pub validate_date: String,
    pub expire_date: String,
}
