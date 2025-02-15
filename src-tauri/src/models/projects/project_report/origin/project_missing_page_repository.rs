// project_missing_page_repository.rs
use crate::models::projects::project_report::origin::missing_page_stats::MissingPageStats;
use crate::models::projects::project_report::origin::project_missing_page_model::{
    NewProjectMissingPage, ProjectMissingPage,
};
use crate::models::projects::project_report::origin::schema::project_missing_page::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sql_query;
use diesel::sql_types::Text;
use crate::models::projects::Pagination;

pub struct ProjectMissingPageRepository {
    pool: Pool<ConnectionManager<SqliteConnection>>, // 使用 SqliteConnection
}

impl ProjectMissingPageRepository {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        ProjectMissingPageRepository { pool }
    }

    // 创建缺失页面记录
    pub fn create_missing_page(
        &self,
        new_missing_page: NewProjectMissingPage,
    ) -> Result<ProjectMissingPage, String> {
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

    pub fn batch_create_missing_page(
        &self,
        new_missing_page_list: Vec<NewProjectMissingPage>,
    ) -> Result<ProjectMissingPage, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::insert_into(project_missing_page)
            .values(&new_missing_page_list)
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;
        project_missing_page
            .order(id.desc())
            .first(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 根据报告编号查询缺失页面记录
    pub fn find_missing_pages_by_report_number(
        &self,
        report_no: &str,
        pagination: Pagination
    ) -> Result<Vec<ProjectMissingPage>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        let offset = (pagination.current_page - 1) * pagination.page_size;

        project_missing_page
            .offset(offset) // 跳过前面的记录
            .limit(pagination.page_size) // 限制
            .filter(report_number.eq(report_no))
            .load::<ProjectMissingPage>(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 根据 ID 查询缺失页面记录
    pub fn find_missing_page_by_id(
        &self,
        missing_page_id: i32,
    ) -> Result<Option<ProjectMissingPage>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        project_missing_page
            .find(missing_page_id)
            .first::<ProjectMissingPage>(&mut conn)
            .optional()
            .map_err(|e| e.to_string())
    }

    // 更新缺失页面记录
    pub fn update_missing_page(
        &self,
        missing_page_id: i32,
        updated_missing_page: NewProjectMissingPage,
    ) -> Result<usize, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::update(project_missing_page.find(missing_page_id))
            .set(&updated_missing_page)
            .execute(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 删除缺失页面记录
    pub fn delete_missing_page(&self, report_no: String) -> Result<usize, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::delete(project_missing_page.filter(report_number.eq(report_no)))
            .execute(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 查询所有唯一的 site_number
    pub fn find_unique_site_numbers(&self) -> Result<Vec<String>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        project_missing_page
            .select(site_number)
            .group_by(site_number)
            .load::<String>(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 根据 report_number 和 site_number 统计 data_page_name 的数量，并生成 gt_7 和 gt_14 字段
    pub fn find_missing_page_stats(
        &self,
        report_no: &str,
        site_no: &str,
    ) -> Result<MissingPageStats, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;

        // 定义 SQL 查询
        let sql = r#"
            SELECT
                COUNT(data_page_name) AS data_page_count,
                SUM(CASE WHEN days_of_missing_pages > 7 THEN 1 ELSE 0 END) AS gt_7,
                SUM(CASE WHEN days_of_missing_pages > 14 THEN 1 ELSE 0 END) AS gt_14
            FROM
                project_missing_page
            WHERE
                report_number = ?
                AND site_number = ?;
        "#;

        // 执行查询并绑定参数
        let result = sql_query(sql)
            .bind::<Text, _>(report_no) // 绑定 report_number
            .bind::<Text, _>(site_no) // 绑定 site_number
            .get_result::<MissingPageStats>(&mut conn)
            .map_err(|e| e.to_string())?;

        Ok(result)
    }
}
