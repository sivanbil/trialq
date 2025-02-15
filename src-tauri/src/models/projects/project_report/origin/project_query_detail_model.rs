// project_query_detail_model.rs
use diesel::prelude::*;
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
    pub create_time: String,
    pub update_time: String,
    pub report_number: String,
}
use crate::models::projects::project_report::model_convert_utils::deserialize_string_to_i32;
// project_query_detail_model.rs
#[derive(Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::models::projects::project_report::origin::schema::project_query_detail)]
pub struct NewProjectQueryDetail {
    pub study: String,
    pub site_name: String,
    pub study_environment_site: String,
    pub subject_name: String,
    pub folder: String,
    pub form: String,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub log_id: i32,
    #[serde(default)]
    pub qry_open_date: String,
    #[serde(default)]
    pub qry_open_date_localized: String,
    pub qry_open_by: String,
    #[serde(default)]
    pub query_text: String,
    pub marking_group_name: String,
    #[serde(default)]
    pub qry_response_date: String,
    #[serde(default)]
    pub qry_response_date_localized: String,
    #[serde(default)]
    pub qry_response_user: String,
    #[serde(default)]
    pub qry_answer: String,
    #[serde(default)]
    pub qry_status: String,
    #[serde(default)]
    pub create_time: String,
    #[serde(default)]
    pub update_time: String,
    #[serde(default)]
    pub report_number: String,
}
