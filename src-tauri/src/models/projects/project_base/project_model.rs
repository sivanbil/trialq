// project_model.rs
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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
