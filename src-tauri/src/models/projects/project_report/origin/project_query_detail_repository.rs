// project_query_detail_repository.rs
use crate::models::projects::project_report::origin::project_query_detail::project_query_detail_model::{NewProjectQueryDetail, ProjectQueryDetail};
use crate::models::projects::project_report::origin::project_query_detail::schema::project_query_detail::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct ProjectQueryDetailRepository {
    pool: Pool<ConnectionManager<SqliteConnection>>, // 使用 SqliteConnection
}

impl ProjectQueryDetailRepository {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        ProjectQueryDetailRepository { pool }
    }

    // 创建查询详情
    pub fn create_query_detail(&self, new_query_detail: NewProjectQueryDetail) -> Result<ProjectQueryDetail, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::insert_into(project_query_detail)
            .values(&new_query_detail)
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;
        project_query_detail
            .order(id.desc())
            .first(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 根据报告编号查询查询详情
    pub fn find_query_details_by_report_number(&self, report_number: &str) -> Result<Vec<ProjectQueryDetail>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        project_query_detail
            .filter(report_number.eq(report_number))
            .load::<ProjectQueryDetail>(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 根据 ID 查询查询详情
    pub fn find_query_detail_by_id(&self, query_detail_id: i32) -> Result<Option<ProjectQueryDetail>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        project_query_detail
            .find(query_detail_id)
            .first::<ProjectQueryDetail>(&mut conn)
            .optional()
            .map_err(|e| e.to_string())
    }

    // 更新查询详情
    pub fn update_query_detail(&self, query_detail_id: i32, updated_query_detail: NewProjectQueryDetail) -> Result<usize, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::update(project_query_detail.find(query_detail_id))
            .set(&updated_query_detail)
            .execute(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 删除查询详情
    pub fn delete_query_detail(&self, query_detail_id: i32) -> Result<usize, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::delete(project_query_detail.find(query_detail_id))
            .execute(&mut conn)
            .map_err(|e| e.to_string())
    }
}