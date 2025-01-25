// project_report_data_model.rs
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct ProjectReportData {
    pub id: i32,
    pub site: String,
    pub site_name: String,
    pub cra: String,
    pub pages: i32,
    pub pages_entered: i32,
    pub missing_pages: i32,
    pub md_gt7: i32,
    pub md_gt14: i32,
    pub sdv_backlog: i32,
    pub edc_status_comment: String,
    pub percent_pages_entered: String, // 改为 f64
    pub percent_pages_sdved: String,   // 改为 f64
    pub answered_query: i32,        // 改为 i32
    pub opened_query: i32,          // 改为 i32
    pub op_gt7: i32,
    pub op_gt14: i32,
    pub op_gt21: i32,
    pub op_gt30: i32,
    pub report_number: String
}

// project_report_data_model.rs
#[derive(Insertable,AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::models::projects::project_report::schema::project_report_data)]
pub struct NewProjectReportData {
    pub site: String,
    pub site_name: String,
    pub cra: String,
    pub pages: i32,
    pub pages_entered: i32,
    pub missing_pages: i32,
    pub md_gt7: i32,
    pub md_gt14: i32,
    pub sdv_backlog: i32,
    pub edc_status_comment: String,
    pub percent_pages_entered: String,
    pub percent_pages_sdved: String,
    pub answered_query: i32,
    pub opened_query: i32,
    pub op_gt7: i32,
    pub op_gt14: i32,
    pub op_gt21: i32,
    pub op_gt30: i32,
    pub report_number: String
}