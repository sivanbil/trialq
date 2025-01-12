// project_missing_page_repository.rs
use crate::models::projects::project_report::origin::project_missing_page::project_missing_page_model::{NewProjectMissingPage, ProjectMissingPage};
use crate::models::projects::project_report::origin::project_missing_page::schema::project_missing_page::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct ProjectMissingPageRepository {
    pool: Pool<ConnectionManager<SqliteConnection>>, // 使用 SqliteConnection
}

impl ProjectMissingPageRepository {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        ProjectMissingPageRepository { pool }
    }

    // 创建缺失页面记录
    pub fn create_missing_page(&self, new_missing_page: NewProjectMissingPage) -> Result<ProjectMissingPage, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::insert_into(project_missing_page)
            .values(&new_missing_page)
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;
        project_missing_page
            .order(id.desc())
            .first(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 根据报告编号查询缺失页面记录
    pub fn find_missing_pages_by_report_number(&self, report_number: &str) -> Result<Vec<ProjectMissingPage>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        project_missing_page
            .filter(report_number.eq(report_number))
            .load::<ProjectMissingPage>(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 根据 ID 查询缺失页面记录
    pub fn find_missing_page_by_id(&self, missing_page_id: i32) -> Result<Option<ProjectMissingPage>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        project_missing_page
            .find(missing_page_id)
            .first::<ProjectMissingPage>(&mut conn)
            .optional()
            .map_err(|e| e.to_string())
    }

    // 更新缺失页面记录
    pub fn update_missing_page(&self, missing_page_id: i32, updated_missing_page: NewProjectMissingPage) -> Result<usize, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::update(project_missing_page.find(missing_page_id))
            .set(&updated_missing_page)
            .execute(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 删除缺失页面记录
    pub fn delete_missing_page(&self, missing_page_id: i32) -> Result<usize, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::delete(project_missing_page.find(missing_page_id))
            .execute(&mut conn)
            .map_err(|e| e.to_string())
    }
}