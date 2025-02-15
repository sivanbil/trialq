use crate::models::projects::project_report::project_report_data_model::{
    NewProjectReportData, ProjectReportData,
};
use crate::models::projects::project_report::schema::project_report_data::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct ProjectReportDataRepository {
    pool: Pool<ConnectionManager<SqliteConnection>>, // 使用 SqliteConnection
}

impl ProjectReportDataRepository {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        ProjectReportDataRepository { pool }
    }

    // 创建报告数据
    pub fn create_report_data(
        &self,
        new_report_data: NewProjectReportData,
    ) -> Result<ProjectReportData, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::insert_into(project_report_data)
            .values(&new_report_data)
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;

        // 返回最新插入的记录
        project_report_data
            .order(id.desc())
            .first(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 根据站点查询报告数据
    pub fn find_reports_by_site(&self, site_str: &str) -> Result<Vec<ProjectReportData>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        project_report_data
            .filter(site.eq(site_str))
            .load::<ProjectReportData>(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 根据报告 ID 查询报告数据
    pub fn find_report_data_by_id(
        &self,
        report_id: i32,
    ) -> Result<Option<ProjectReportData>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        project_report_data
            .find(report_id)
            .first::<ProjectReportData>(&mut conn)
            .optional()
            .map_err(|e| e.to_string())
    }

    // 根据报告编号查询报告数据
    pub fn find_report_data_by_no(
        &self,
        report_no: &str,
    ) -> Result<Vec<ProjectReportData>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        project_report_data
            .filter(report_number.eq(report_no))
            .order(site.asc()) // 根据 site 字段升序排列
            .load::<ProjectReportData>(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 更新报告数据
    pub fn update_report_data(
        &self,
        report_id: i32,
        updated_report_data: NewProjectReportData,
    ) -> Result<usize, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::update(project_report_data.find(report_id))
            .set(&updated_report_data)
            .execute(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 删除报告数据
    pub fn delete_report_data(&self, report_no: String) -> Result<usize, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::delete(project_report_data.filter(report_number.eq(report_no)))
            .execute(&mut conn)
            .map_err(|e| e.to_string())
    }
}
