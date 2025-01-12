// project_report_repository.rs
use crate::models::projects::project_report::project_report_model::{NewProjectReport, ProjectReport};
use crate::models::projects::project_report::schema::project_report::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct ProjectReportRepository {
    pool: Pool<ConnectionManager<SqliteConnection>>, // 使用 SqliteConnection
}

impl ProjectReportRepository {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        ProjectReportRepository { pool }
    }

    // 创建报告
    pub fn create_report(&self, new_report: NewProjectReport) -> Result<ProjectReport, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::insert_into(project_report)
            .values(&new_report)
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;
        project_report
            .order(id.desc())
            .first(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 根据项目编号查询报告
    pub fn find_reports_by_project_number(&self, project_no: &str) -> Result<Vec<ProjectReport>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        project_report
            .filter(project_number.eq(project_no))
            .load::<ProjectReport>(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 根据报告编号查询报告
    pub fn find_report_by_report_number(&self, report_no: &str) -> Result<Option<ProjectReport>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        project_report
            .filter(report_number.eq(report_no))
            .first::<ProjectReport>(&mut conn)
            .optional()
            .map_err(|e| e.to_string())
    }

    // 根据报告 ID 查询报告
    pub fn find_report_by_id(&self, report_id: i32) -> Result<Option<ProjectReport>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        project_report
            .find(report_id)
            .first::<ProjectReport>(&mut conn)
            .optional()
            .map_err(|e| e.to_string())
    }

    // 更新报告
    pub fn update_report(&self, report_id: i32, updated_report: NewProjectReport) -> Result<usize, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::update(project_report.find(report_id))
            .set(&updated_report)
            .execute(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 删除报告
    pub fn delete_report(&self, report_id: i32) -> Result<usize, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::delete(project_report.find(report_id))
            .execute(&mut conn)
            .map_err(|e| e.to_string())
    }
}