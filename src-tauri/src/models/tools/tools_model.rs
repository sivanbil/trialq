use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Tool {
    pub id: i32,
    pub name: String,
    pub link_url: String,
    pub sort_value: i32,
    pub is_valid: bool,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = crate::models::tools::schema::tools)]
pub struct NewTool {
    pub name: String,
    pub link_url: String,
    pub sort_value: i32,
    pub is_valid: bool,
}
