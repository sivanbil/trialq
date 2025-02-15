// project_missing_page_model.rs
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct ProjectMissingPage {
    pub id: i32,
    pub project_name: String,
    pub site_name: String,
    pub site_number: String,
    pub subject_id: String,
    pub instance_name: String,
    pub data_page_name: String,
    pub days_of_missing_pages: i32,
    pub create_time: String,
    pub update_time: String,
    pub report_number: String,
}

// project_missing_page_model.rs
use crate::models::projects::project_report::model_convert_utils::deserialize_string_to_i32;
#[derive(Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::models::projects::project_report::origin::schema::project_missing_page)]
pub struct NewProjectMissingPage {
    pub project_name: String,
    pub site_name: String,
    pub site_number: String,
    pub subject_id: String,
    pub instance_name: String,
    pub data_page_name: String,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub days_of_missing_pages: i32,
    #[serde(default)]
    pub create_time: String,
    #[serde(default)]
    pub update_time: String,
    #[serde(default)]
    pub report_number: String,
}
