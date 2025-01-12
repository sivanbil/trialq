// models/task_model.rs
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: i32,
    pub task_no: String,
    pub related_id: i32,
    pub task_category: i32,
    pub state: i32,
    pub create_time: String,
    pub update_time: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub task_no: String,
    pub related_id: i32,
    pub task_category: i32,
    pub state: i32,
    pub create_time: String,
    pub update_time: String,
}