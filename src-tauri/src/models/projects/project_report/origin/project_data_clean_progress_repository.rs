// project_data_clean_progress_repository.rs
use crate::models::projects::project_report::origin::project_data_clean_progress::project_data_clean_progress_model::{NewProjectDataCleanProgress, ProjectDataCleanProgress};
use crate::models::projects::project_report::origin::project_data_clean_progress::schema::project_data_clean_progress::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct ProjectDataCleanProgressRepository {
    pool: Pool<ConnectionManager<SqliteConnection>>, // 使用 SqliteConnection
}

impl ProjectDataCleanProgressRepository {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        ProjectDataCleanProgressRepository { pool }
    }

    // 创建数据清理进度
    pub fn create_data_clean_progress(&self, new_data_clean_progress: NewProjectDataCleanProgress) -> Result<ProjectDataCleanProgress, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::insert_into(project_data_clean_progress)
            .values(&new_data_clean_progress)
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;
        project_data_clean_progress
            .order(id.desc())
            .first(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 根据报告编号查询数据清理进度
    pub fn find_data_clean_progress_by_report_number(&self, report_number: &str) -> Result<Vec<ProjectDataCleanProgress>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        project_data_clean_progress
            .filter(report_number.eq(report_number))
            .load::<ProjectDataCleanProgress>(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 根据 ID 查询数据清理进度
    pub fn find_data_clean_progress_by_id(&self, data_clean_progress_id: i32) -> Result<Option<ProjectDataCleanProgress>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        project_data_clean_progress
            .find(data_clean_progress_id)
            .first::<ProjectDataCleanProgress>(&mut conn)
            .optional()
            .map_err(|e| e.to_string())
    }

    // 更新数据清理进度
    pub fn update_data_clean_progress(&self, data_clean_progress_id: i32, updated_data_clean_progress: NewProjectDataCleanProgress) -> Result<usize, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::update(project_data_clean_progress.find(data_clean_progress_id))
            .set(&updated_data_clean_progress)
            .execute(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 删除数据清理进度
    pub fn delete_data_clean_progress(&self, data_clean_progress_id: i32) -> Result<usize, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::delete(project_data_clean_progress.find(data_clean_progress_id))
            .execute(&mut conn)
            .map_err(|e| e.to_string())
    }
}