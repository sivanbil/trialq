// project_data_clean_progress_model.rs
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct ProjectDataCleanProgress {
    pub id: i32,
    pub study: String,
    pub site: String,
    pub subject: String,
    pub folder_name: String,
    pub page: String,
    pub entered: i32,
    pub verify_required: i32,
    pub create_time: String,
    pub update_time: String,
    pub report_number: String,
}

use crate::models::projects::project_report::model_convert_utils::{deserialize_string_to_i32};
// project_data_clean_progress_model.rs
#[derive(Insertable,AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::models::projects::project_report::origin::schema::project_data_clean_progress)]
pub struct NewProjectDataCleanProgress {
    pub study: String,
    pub site: String,
    pub subject: String,
    pub folder_name: String,
    pub page: String,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub entered: i32,
    #[serde(deserialize_with = "deserialize_string_to_i32")]
    pub verify_required: i32,
    #[serde(default)]
    pub create_time: String,
    #[serde(default)]
    pub update_time: String,
    #[serde(default)]
    pub report_number: String,
}




