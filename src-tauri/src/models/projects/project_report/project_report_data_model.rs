// project_report_data_model.rs
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct ProjectReportData {
    pub id: i32,
    pub site: String,
    pub site_name: String,
    pub cra: String,
    pub enrollment: String,
    pub pages: String,
    pub pages_entered: String,
    pub missing_pages: String,
    pub sdv_backlog: String,
    pub edc_status_comment: String,
    pub plan_rmv: String,
    pub latest_rmv: String,
}

// project_report_data_model.rs
#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = project_report_data)]
pub struct NewProjectReportData {
    pub site: String,
    pub site_name: String,
    pub cra: String,
    pub enrollment: String,
    pub pages: String,
    pub pages_entered: String,
    pub missing_pages: String,
    pub sdv_backlog: String,
    pub edc_status_comment: String,
    pub plan_rmv: String,
    pub latest_rmv: String,
}