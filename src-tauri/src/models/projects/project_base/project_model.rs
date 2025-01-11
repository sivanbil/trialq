// project_model.rs
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub project_name: String,
    pub description: String,
    pub create_time: String,
    pub update_time: String,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = crate::models::projects::project_base::schema::projects)]
pub struct NewProject {
    pub project_name: String,
    pub description: String,
    pub create_time: String,
    pub update_time: String,
}