// project_query_detail_repository.rs
use crate::models::projects::project_report::origin::project_query_detail_model::{NewProjectQueryDetail, ProjectQueryDetail};
use crate::models::projects::project_report::origin::schema::project_query_detail::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::models::projects::project_report::origin::project_data_clean_progress_repository::ProjectDataCleanProgressRepository;


#[derive(Debug)]
pub struct QueryStatistics {
    pub answered_query_count: i64,
    pub opened_query_count: i64,
    pub query_age_distribution: QueryAgeDistribution,
}

#[derive(Debug)]
pub struct QueryAgeDistribution {
    pub less_than_7_days: i32,
    pub between_7_and_14_days: i32,
    pub between_14_and_21_days: i32,
    pub between_21_and_30_days: i32,
    pub more_than_30_days: i32,
}

use chrono::{NaiveDate, Local};

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
    pub fn find_query_details_by_report_number(&self, report_no: &str) -> Result<Vec<ProjectQueryDetail>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        project_query_detail
            .filter(report_number.eq(report_no))
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

    // 查询所有唯一的 study_environment_site
    pub fn find_unique_study_environment_sites(&self) -> Result<Vec<String>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        project_query_detail
            .select(study_environment_site)
            .group_by(study_environment_site)
            .load::<String>(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 获取查询统计信息
    // 获取查询统计信息
    pub fn get_query_statistics(&self) -> Result<QueryStatistics, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;

        // 获取 Answered Query 的数量
        let answered_query_count: i64 = project_query_detail
            .filter(qry_status.eq("Answered"))
            .count()
            .get_result(&mut conn)
            .map_err(|e| e.to_string())?;

        // 获取 Opened Query 的数量
        let opened_query_count: i64 = project_query_detail
            .filter(qry_status.eq("Open"))
            .count()
            .get_result(&mut conn)
            .map_err(|e| e.to_string())?;

        // 获取所有 Open 状态的 Query 的 qry_open_date
        let open_queries: Vec<String> = project_query_detail
            .filter(qry_status.eq("Open"))
            .select(qry_open_date)
            .load(&mut conn)
            .map_err(|e| e.to_string())?;

        // 计算 Query 的年龄分布
        let mut less_than_7_days = 0;
        let mut between_7_and_14_days = 0;
        let mut between_14_and_21_days = 0;
        let mut between_21_and_30_days = 0;
        let mut more_than_30_days = 0;

        let current_date = Local::now().naive_local().date();

        for qry_open_date_str in open_queries {
            // 解析日期字符串，并将结果绑定到一个新的变量名
            let parsed_qry_open_date = NaiveDate::parse_from_str(&qry_open_date_str, "%Y/%m/%d")
                .map_err(|e| format!("Failed to parse date: {}", e))?;

            // 计算日期差值
            let days_open = (current_date - parsed_qry_open_date).num_days();

            // 根据差值分类
            match days_open {
                0..=6 => less_than_7_days += 1,
                7..=13 => between_7_and_14_days += 1,
                14..=20 => between_14_and_21_days += 1,
                21..=30 => between_21_and_30_days += 1,
                _ => more_than_30_days += 1,
            }
        }

        // 返回结果
        Ok(QueryStatistics {
            answered_query_count,
            opened_query_count,
            query_age_distribution: QueryAgeDistribution {
                less_than_7_days,
                between_7_and_14_days,
                between_14_and_21_days,
                between_21_and_30_days,
                more_than_30_days,
            },
        })
    }
}
