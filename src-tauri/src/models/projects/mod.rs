use serde::Serialize;

pub mod project_base;

pub mod dashboard;

pub mod project_report;

#[derive(Debug,Clone,Serialize)]
pub struct Pagination {
    pub current_page: i64,       // 当前页码
    pub page_size: i64,          // 每页大小
    pub keyword: Option<String>, // 可选的 keyword 参数，用于模糊检索
}




