// project_missing_page_model.rs
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

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
#[derive(Insertable,AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::models::projects::project_report::origin::schema::project_missing_page)]
pub struct NewProjectMissingPage {
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