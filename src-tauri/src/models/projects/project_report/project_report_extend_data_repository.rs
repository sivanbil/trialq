// project_report_extend_data_repository.rs
use crate::models::projects::project_report_extend_data::project_report_extend_data_model::{NewProjectReportExtendData, ProjectReportExtendData};
use crate::models::projects::project_report_extend_data::schema::project_report_extend_data::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct ProjectReportExtendDataRepository {
    pool: Pool<ConnectionManager<SqliteConnection>>, // 使用 SqliteConnection
}

impl ProjectReportExtendDataRepository {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        ProjectReportExtendDataRepository { pool }
    }

    // 创建扩展数据
    pub fn create_extend_data(&self, new_extend_data: NewProjectReportExtendData) -> Result<ProjectReportExtendData, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::insert_into(project_report_extend_data)
            .values(&new_extend_data)
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;
        project_report_extend_data
            .order(id.desc())
            .first(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 根据 project_report_data_id 查询扩展数据
    pub fn find_extend_data_by_report_data_id(&self, report_data_id: i32) -> Result<Vec<ProjectReportExtendData>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        project_report_extend_data
            .filter(project_report_data_id.eq(report_data_id))
            .load::<ProjectReportExtendData>(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 根据 ID 查询扩展数据
    pub fn find_extend_data_by_id(&self, extend_data_id: i32) -> Result<Option<ProjectReportExtendData>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        project_report_extend_data
            .find(extend_data_id)
            .first::<ProjectReportExtendData>(&mut conn)
            .optional()
            .map_err(|e| e.to_string())
    }

    // 更新扩展数据
    pub fn update_extend_data(&self, extend_data_id: i32, updated_extend_data: NewProjectReportExtendData) -> Result<usize, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::update(project_report_extend_data.find(extend_data_id))
            .set(&updated_extend_data)
            .execute(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 删除扩展数据
    pub fn delete_extend_data(&self, extend_data_id: i32) -> Result<usize, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::delete(project_report_extend_data.find(extend_data_id))
            .execute(&mut conn)
            .map_err(|e| e.to_string())
    }
}