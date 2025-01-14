use serde::{Serialize, Deserialize};
use diesel::prelude::*;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct ProjectSite {
    pub id: i32,
    pub project_name: String,
    pub site_number: String,
    pub site_name: String,
    pub site_cra: String,
}

#[derive(Insertable,AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::models::projects::project_base::schema::project_site)]
pub struct NewProjectSite {
    #[serde(default)]
    pub project_name: String,
    pub site_number: String,
    #[serde(default)]
    pub site_name: String,
    #[serde(default)]
    pub site_cra: String,
}