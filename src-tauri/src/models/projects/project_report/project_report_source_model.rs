use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct ProjectReportSource {
    pub id: i32,
    pub project_number: String,
    pub report_number: String,
    pub source_file_name: String,
    pub source_file_type: String,
    pub create_time: String,
    pub update_time: String,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::models::projects::project_report::schema::project_report_source)]
pub struct NewProjectReportSource {
    pub project_number: String,
    pub report_number: String,
    pub source_file_name: String,
    pub source_file_type: String,
    pub create_time: String,
    pub update_time: String,
}
