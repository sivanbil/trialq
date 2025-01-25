// project_report_repository.rs
use crate::models::projects::project_report::project_report_model::{NewProjectReport, ProjectReport};
use crate::models::projects::project_report::schema::project_report::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::models::projects::Pagination;

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

    // 更新报告状态
    pub fn update_report_state(&self, report_id: i32, new_state: i32) -> Result<usize, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::update(project_report.find(report_id))
            .set(state.eq(new_state)) // Update only the `status` field
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

    // 分页查询项目（支持 keyword 检索）
    pub fn find_project_reports_paginated(&self, pagination: Pagination) -> Result<Vec<ProjectReport>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;

        // 计算 offset
        let offset = (pagination.current_page - 1) * pagination.page_size;

        // 构建查询
        let mut query = project_report.into_boxed();

        // 如果提供了 keyword，则添加模糊匹配条件
        if let Some(keyword) = pagination.keyword {
            let search_pattern = format!("{}%", keyword); // 使用 % 实现模糊匹配
            query = query.filter(project_number.like(search_pattern));
        }

        // 执行分页查询
        query
            .order(id.asc()) // 按 id 升序排序
            .offset(offset) // 跳过前面的记录
            .limit(pagination.page_size) // 限制每页的记录数
            .load::<ProjectReport>(&mut conn)
            .map_err(|e| e.to_string())
    }

    /// 根据项目编号统计报告总数
    /// 如果 project_no 为空，统计所有报告的总数
    pub fn count_reports_by_project_number(&self, project_no: Option<String>) -> Result<i64, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;

        // 构建查询
        let mut query = project_report.into_boxed();

        // 如果提供了 keyword，则添加模糊匹配条件
        if let Some(keyword) = project_no {
            let search_pattern = format!("{}", keyword); // 使用 % 实现模糊匹配
            query = query.filter(project_number.eq(search_pattern));
        }

        // 查询总记录数
        query
            .count()
            .get_result(&mut conn)
            .map_err(|e| e.to_string())
    }
}