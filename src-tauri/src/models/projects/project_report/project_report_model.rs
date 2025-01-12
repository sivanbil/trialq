// project_report_model.rs
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct ProjectReport {
    pub id: i32,
    pub project_number: String,
    pub report_number: String,
    pub state: i32,
    pub create_time: String,
    pub update_time: String,
}

// project_report_model.rs
#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = project_reports)]
pub struct NewProjectReport {
    pub project_number: String,
    pub report_number: String,
    pub state: i32,
    pub create_time: String,
    pub update_time: String,
}