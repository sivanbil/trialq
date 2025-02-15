use crate::models::projects::project_report::project_report_source_model::{
    NewProjectReportSource, ProjectReportSource,
};
use crate::models::projects::project_report::schema::project_report_source::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct ProjectReportSourceRepository {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl ProjectReportSourceRepository {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        ProjectReportSourceRepository { pool }
    }

    // 创建新的 ProjectReportSource
    pub fn create(
        &self,
        new_source: NewProjectReportSource,
    ) -> Result<ProjectReportSource, diesel::result::Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(project_report_source)
            .values(&new_source)
            .execute(&mut conn)?;

        project_report_source.order(id.desc()).first(&mut conn)
    }

    // 根据 ID 查找 ProjectReportSource
    pub fn find_by_id(
        &self,
        source_id: i32,
    ) -> Result<Option<ProjectReportSource>, diesel::result::Error> {
        let mut conn = self.pool.get().unwrap();
        project_report_source
            .find(source_id)
            .first(&mut conn)
            .optional()
    }

    // 根据项目编号查找 ProjectReportSource
    pub fn find_by_report_number(
        &self,
        report_no: &str,
    ) -> Result<Vec<ProjectReportSource>, diesel::result::Error> {
        let mut conn = self.pool.get().unwrap();
        project_report_source
            .filter(report_number.eq(report_no))
            .load(&mut conn)
    }

    // 更新 ProjectReportSource
    pub fn update(
        &self,
        source_id: i32,
        updated_source: NewProjectReportSource,
    ) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::update(project_report_source.find(source_id))
            .set(&updated_source)
            .execute(&mut conn)
    }

    // 删除 ProjectReportSource
    pub fn delete(&self, report_no: String) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(project_report_source.filter(report_number.eq(report_no))).execute(&mut conn)
    }
}
