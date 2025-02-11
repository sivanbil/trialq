// project_data_clean_progress_repository.rs
use crate::models::projects::project_report::origin::project_data_clean_progress_model::{NewProjectDataCleanProgress, ProjectDataCleanProgress};
use crate::models::projects::project_report::origin::schema::project_data_clean_progress::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sql_query;
use diesel::sql_types::Text;

#[derive(QueryableByName, Debug)]
struct SiteNumber {
    #[sql_type = "Text"]
    site_number: String,
}

#[derive(Debug)]
pub struct DataCleanProgressStats {
    pub total_pages: i64,          // 总页数
    pub total_entered: i64,        // 已录入的页数
    pub total_verify_required: i64, // 需要验证的页数
}
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

    // 创建数据清理进度
    pub fn batch_create_data_clean_progress(&self, new_data_clean_progress_list: Vec<NewProjectDataCleanProgress>) -> Result<ProjectDataCleanProgress, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::insert_into(project_data_clean_progress)
            .values(&new_data_clean_progress_list)
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;
        project_data_clean_progress
            .order(id.desc())
            .first(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 根据报告编号查询数据清理进度
    pub fn find_data_clean_progress_by_report_number(&self, report_no: &str) -> Result<Vec<ProjectDataCleanProgress>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        project_data_clean_progress
            .filter(report_number.eq(report_no))
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

    // 查询 site_number（通过 SUBSTR 和 INSTR 函数处理 site 字段）
    pub fn find_site_numbers(&self) -> Result<Vec<String>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;

        // 定义 SQL 查询
        let query = r#"
            SELECT SUBSTR(site, 1, INSTR(site, '-') - 1) AS site_number
            FROM project_data_clean_progress
            GROUP BY site;
        "#;

        // 执行原始 SQL 查询
        sql_query(query)
            .load::<SiteNumber>(&mut conn)
            .map_err(|e| e.to_string())
            .map(|results| results.into_iter().map(|r| r.site_number).collect())
    }

    // 获取汇总信息
    // 获取数据清理进度统计信息
    pub fn find_data_clean_progress_stats(&self, report_number_str: &str, site_number: &str) -> Result<DataCleanProgressStats, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        let result = project_data_clean_progress
            .filter(report_number.eq(report_number_str))
            .filter(site.like(format!("{}%", site_number))) // 假设 site 字段以 site_number 开头
            .select((
                diesel::dsl::count(id).nullable(),
                diesel::dsl::sum(entered).nullable(),
                diesel::dsl::sum(verify_required).nullable(),
            ))
            .first::<(Option<i64>, Option<i64>, Option<i64>)>(&mut conn)
            .map_err(|e| e.to_string())?;
        let total_pages = result.0.unwrap_or(0);
        let total_entered = result.1.unwrap_or(0);
        let total_verify_required = result.2.unwrap_or(0);
        Ok(DataCleanProgressStats {
            total_pages,
            total_entered,
            total_verify_required,
        })
    }
}
