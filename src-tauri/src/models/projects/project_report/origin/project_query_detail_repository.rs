// project_query_detail_repository.rs
use crate::models::projects::project_report::origin::project_query_detail_model::{
    NewProjectQueryDetail, ProjectQueryDetail,
};
use crate::models::projects::project_report::origin::schema::project_query_detail::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

#[derive(Debug)]
pub struct QueryStatistics {
    pub answered_query_count: i64,
    pub opened_query_count: i64,
    pub query_age_distribution: QueryAgeDistribution,
}

#[derive(Debug)]
pub struct QueryAgeDistribution {
    pub less_than_7_days: i32,
    pub more_than_7_days: i32,
    pub more_than_14_days: i32,
    pub more_than_21_days: i32,
    pub more_than_30_days: i32,
}

use chrono::{Local, NaiveDate};
use crate::models::projects::Pagination;

pub struct ProjectQueryDetailRepository {
    pool: Pool<ConnectionManager<SqliteConnection>>, // 使用 SqliteConnection
}

impl ProjectQueryDetailRepository {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        ProjectQueryDetailRepository { pool }
    }

    // 创建查询详情
    pub fn create_query_detail(
        &self,
        new_query_detail: NewProjectQueryDetail,
    ) -> Result<ProjectQueryDetail, String> {
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

    // 创建查询详情
    pub fn batch_create_query_detail(
        &self,
        new_query_detail: Vec<NewProjectQueryDetail>,
    ) -> Result<ProjectQueryDetail, String> {
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
    pub fn find_query_details_by_report_number(
        &self,
        report_no: &str,
        pagination: Pagination
    ) -> Result<Vec<ProjectQueryDetail>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;

        let offset = (pagination.current_page - 1) * pagination.page_size;

        project_query_detail
            .offset(offset) // 跳过前面的记录
            .limit(pagination.page_size) // 限制每页的记录数
            .filter(report_number.eq(report_no))
            .load::<ProjectQueryDetail>(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 根据 ID 查询查询详情
    pub fn find_query_detail_by_id(
        &self,
        query_detail_id: i32,
    ) -> Result<Option<ProjectQueryDetail>, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        project_query_detail
            .find(query_detail_id)
            .first::<ProjectQueryDetail>(&mut conn)
            .optional()
            .map_err(|e| e.to_string())
    }

    // 更新查询详情
    pub fn update_query_detail(
        &self,
        query_detail_id: i32,
        updated_query_detail: NewProjectQueryDetail,
    ) -> Result<usize, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::update(project_query_detail.find(query_detail_id))
            .set(&updated_query_detail)
            .execute(&mut conn)
            .map_err(|e| e.to_string())
    }

    // 删除查询详情
    pub fn delete_query_detail(&self, report_no: String) -> Result<usize, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;
        diesel::delete(project_query_detail.filter(report_number.eq(report_no)))
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

    fn classify_days_open(
        &self,
        days_open: i64,
        less_than_7_days: &mut i32,
        more_than_7_days: &mut i32,
        more_than_14_days: &mut i32,
        more_than_21_days: &mut i32,
        more_than_30_days: &mut i32,
    ) {
        if days_open <= 6 {
            *less_than_7_days += 1;
        }
        if days_open > 7 {
            *more_than_7_days += 1;
        }
        if days_open > 14 {
            *more_than_14_days += 1;
        }
        if days_open > 21 {
            *more_than_21_days += 1;
        }

        if days_open >=30 {
            *more_than_30_days += 1;
        }
    }

    // 获取查询统计信息
    // 获取查询统计信息
    pub fn get_query_statistics(
        &self,
        report_number_str: &str,
        site_number: &str,
    ) -> Result<QueryStatistics, String> {
        let mut conn = self.pool.get().map_err(|e| e.to_string())?;

        // 获取 Answered Query 的数量
        let answered_query_count: i64 = project_query_detail
            .filter(qry_status.eq("Answered"))
            .filter(report_number.eq(report_number_str))
            .filter(study_environment_site.eq(site_number))
            .count()
            .get_result(&mut conn)
            .map_err(|e| e.to_string())?;
        // 获取所有 Open 状态的 Query 的 qry_open_date
        let open_queries: Vec<String> = project_query_detail
            .filter(qry_status.eq("Open"))
            .filter(report_number.eq(report_number_str))
            .filter(study_environment_site.eq(site_number))
            .select(qry_open_date)
            .load(&mut conn)
            .map_err(|e| e.to_string())?;

        let opened_query_count: i64 = open_queries.len() as i64;
        println!(
            "site_number:{}, {:?},Opened query count: {}",
            site_number,
            report_number_str,
            open_queries.len()
        );
        // 计算 Query 的日期分布
        let mut less_than_7_days = 0;
        let mut more_than_7_days = 0;
        let mut more_than_14_days = 0;
        let mut more_than_21_days = 0;
        let mut more_than_30_days = 0;
        let current_date = Local::now().naive_local().date();
        for qry_open_date_str in open_queries {
            if let Some(pos) = qry_open_date_str.find(' ') {
                let output = &qry_open_date_str[0..pos];
                // 解析日期字符串，并将结果绑定到一个新的变量名
                let parsed_qry_open_date = NaiveDate::parse_from_str(output, "%m/%d/%Y")
                    .map_err(|e| format!("Failed to parse date: {}", e))?;

                // 计算日期差值
                println!(
                    "current_date:{}, parsed_qry_open_date: {:?}",
                    current_date, parsed_qry_open_date
                );
                let days_open = (current_date - parsed_qry_open_date).num_days();
                println!("days_open: {:?}", days_open);

                self.classify_days_open(days_open, &mut less_than_7_days, &mut more_than_7_days,&mut more_than_14_days, &mut more_than_21_days, &mut more_than_30_days);

            }
        }
        // 返回结果
        Ok(QueryStatistics {
            answered_query_count,
            opened_query_count,
            query_age_distribution: QueryAgeDistribution {
                less_than_7_days,
                more_than_7_days,
                more_than_14_days,
                more_than_21_days,
                more_than_30_days,
            },
        })
    }
}
