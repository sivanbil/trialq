
// project_query_detail_model.rs
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct ProjectQueryDetail {
    pub id: i32,
    pub study: String,
    pub site_name: String,
    pub study_environment_site: String,
    pub subject_name: String,
    pub folder: String,
    pub form: String,
    pub log_id: i32,
    pub qry_open_date: String,
    pub qry_open_date_localized: String,
    pub qry_open_by: String,
    pub query_text: String,
    pub marking_group_name: String,
    pub qry_response_date: String,
    pub qry_response_date_localized: String,
    pub qry_response_user: String,
    pub qry_answer: String,
    pub qry_status: String,
    pub report_number: String,
}

// project_query_detail_model.rs
#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = project_query_detail)]
pub struct NewProjectQueryDetail {
    pub study: String,
    pub site_name: String,
    pub study_environment_site: String,
    pub subject_name: String,
    pub folder: String,
    pub form: String,
    pub log_id: i32,
    pub qry_open_date: String,
    pub qry_open_date_localized: String,
    pub qry_open_by: String,
    pub query_text: String,
    pub marking_group_name: String,
    pub qry_response_date: String,
    pub qry_response_date_localized: String,
    pub qry_response_user: String,
    pub qry_answer: String,
    pub qry_status: String,
    pub report_number: String,
}