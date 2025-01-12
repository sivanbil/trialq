// project_report_extend_data_model.rs
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct ProjectReportExtendData {
    pub id: i32,
    pub project_report_data_id: i32,
    pub data_group: String,
    pub percent_pages_entered: String,
    pub percent_pages_solved: String,
    pub answered_query: String,
    pub opened_query: String,
    pub od_gt7: i32,
    pub od_gt14: i32,
    pub op_gt21: i32,
    pub op_gt30: i32,
}

// project_report_extend_data_model.rs
#[derive(Insertable,AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::models::projects::project_report::schema::project_report_extend_data)]
pub struct NewProjectReportExtendData {
    pub project_report_data_id: i32,
    pub data_group: String,
    pub percent_pages_entered: String,
    pub percent_pages_solved: String,
    pub answered_query: String,
    pub opened_query: String,
    pub od_gt7: i32,
    pub od_gt14: i32,
    pub op_gt21: i32,
    pub op_gt30: i32,
}