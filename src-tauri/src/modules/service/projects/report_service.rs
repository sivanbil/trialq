use std::path::PathBuf;
// 主要是报告的数据，已经与项目关联的逻辑
// 创建项目报告
// project_report_service.rs
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use std::sync::Arc;
use std::time::SystemTime;
use date_formatter::utils::format_date;
use regitry_code::generate_task_number;
use serde::Serialize;
use crate::models::projects::{project_report::{
    project_report_model::{ProjectReport, NewProjectReport},
    project_report_repository::ProjectReportRepository,
    project_report_source_repository::ProjectReportSourceRepository,
    project_report_data_repository::ProjectReportDataRepository,
    project_report_extend_data_repository::ProjectReportExtendDataRepository,
    origin::{
        project_query_detail_repository::ProjectQueryDetailRepository,
        project_query_detail_model::{
            NewProjectQueryDetail
        },
        project_missing_page_repository::ProjectMissingPageRepository,
        project_missing_page_model::{
            NewProjectMissingPage
        },
        project_data_clean_progress_repository::ProjectDataCleanProgressRepository,
        project_data_clean_progress_model::{
            NewProjectDataCleanProgress,
        }
    }
}, Pagination};
#[derive(Serialize)]
pub struct ReportListResponse {
    valid: bool,
    data: Vec<ResponseData>, // 项目列表
    total: i64,              // 总记录数
    current_page: i64,       // 当前页码
    page_size: i64,          // 每页大小
}
#[derive(Serialize)]
pub struct ResponseData {
    #[serde(rename = "projectName")] // 转换为驼峰命名
    project_name: String,
    #[serde(rename = "reportNumber")] // 转换为驼峰命名
    report_number: String,
    #[serde(rename = "sourceFiles")] // 转换为驼峰命名
    source_files: Vec<String>,
    #[serde(rename = "createTime")] // 转换为驼峰命名
    create_time: String,
    #[serde(rename = "stateName")]
    state_name: String,
}

// 定义返回的 JSON 结构体
#[derive(Serialize)]
pub struct Response {
    valid: bool,
    data: ResponseData,
}

#[derive(Serialize)]
pub struct SummaryResponse {
    valid: bool,
    sate: i32,
}
use crate::core::excel_process_engine::{FileProcessor, ValidationResult};
use crate::models::projects::project_report::project_report_source_model::NewProjectReportSource;

pub struct ProjectReportService {
    repository: Arc<ProjectReportRepository>,
    source_repository: Arc<ProjectReportSourceRepository>,
    data_repository: Arc<ProjectReportDataRepository>,
    extend_data_repository: Arc<ProjectReportExtendDataRepository>,
    missing_page_repository: Arc<ProjectMissingPageRepository>,
    query_detail_repository: Arc<ProjectQueryDetailRepository>,
    clean_data_repository: Arc<ProjectDataCleanProgressRepository>,

}

impl ProjectReportService {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        ProjectReportService {
            repository: Arc::new(ProjectReportRepository::new(pool.clone())),
            source_repository: Arc::new(ProjectReportSourceRepository::new(pool.clone())),
            data_repository: Arc::new(ProjectReportDataRepository::new(pool.clone())),
            extend_data_repository: Arc::new(ProjectReportExtendDataRepository::new(pool.clone())),
            missing_page_repository: Arc::new(ProjectMissingPageRepository::new(pool.clone())),
            clean_data_repository: Arc::new(ProjectDataCleanProgressRepository::new(pool.clone())),
            query_detail_repository: Arc::new(ProjectQueryDetailRepository::new(pool.clone())),
        }
    }

    // 创建报告
    pub fn create_report(&self, new_report: NewProjectReport) -> Result<ProjectReport, String> {
        self.repository.create_report(new_report)
    }

    // 根据项目编号查询报告
    pub fn find_reports_by_project_number(&self, project_number: &str) -> Result<Vec<ProjectReport>, String> {
        self.repository.find_reports_by_project_number(project_number)
    }

    // 根据报告编号查询报告
    pub fn find_report_by_report_number(&self, report_number: &str) -> Result<Option<ProjectReport>, String> {
        self.repository.find_report_by_report_number(report_number)
    }

    // 根据报告 ID 查询报告
    pub fn find_report_by_id(&self, report_id: i32) -> Result<Option<ProjectReport>, String> {
        self.repository.find_report_by_id(report_id)
    }

    // 更新报告
    pub fn update_report(&self, report_id: i32, updated_report: NewProjectReport) -> Result<usize, String> {
        self.repository.update_report(report_id, updated_report)
    }

    // 删除报告
    pub fn delete_report(&self, report_id: i32) -> Result<usize, String> {
        self.repository.delete_report(report_id)
    }

    pub fn async_process_excel_files(&self, files: Vec<String>, project_number: String) -> Result<Response, String> {
        // 1. 生成报告编号和创建时间
        let create_time = format_date(SystemTime::now(), "%Y-%m-%d");
        let report_number = format!("REPORT_{}", generate_task_number());

        // 2. 创建项目报告
        self.create_project_report(&project_number, &report_number, &create_time)?;

        // 3. 处理每个文件
        for file_path in files {
            self.process_file(&file_path, &project_number, &report_number, &create_time)?;
        }

        // 4. 构建返回的 JSON 数据
        let source_files = vec!["query_detail".to_string(), "data_clean_progress".to_string(), "missing_pages".to_string()]; // 示例数据
        Ok(self.build_response(&report_number, &create_time, source_files))

    }

    /// 创建项目报告
    fn create_project_report(&self, project_number: &str, report_number: &str, create_time: &str) -> Result<ProjectReport, String> {
        let report = NewProjectReport {
            project_number: project_number.to_string(),
            report_number: report_number.to_string(),
            state: 1,// 1 导入基础数据 2 做完数据分析
            create_time: create_time.to_string(),
            update_time: "".to_string(),
        };
        self.repository.create_report(report).map_err(|e| e.to_string())
    }

    /// 处理单个文件
    fn process_file(&self, file_path: &str, project_number: &str, report_number: &str, create_time: &str) -> Result<(), String> {
        let file_path_buf = PathBuf::from(file_path);
        let file_name = file_path_buf
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or_default();

        // 根据文件名判断文件类型
        let source_file_type = match file_name.to_lowercase().as_str() {
            s if s.contains("query") => "query_detail",
            s if s.contains("clean") => "data_clean_progress",
            s if s.contains("missing") => "missing_pages",
            _ => "",
        };

        // 创建报告源
        let source = NewProjectReportSource {
            project_number: project_number.to_string(),
            report_number: report_number.to_string(),
            source_file_name: file_name.to_string(),
            source_file_type: source_file_type.to_string(),
            create_time: create_time.to_string(),
            update_time: "".to_string(),
        };
        self.source_repository.create(source).map_err(|e| e.to_string())?;

        // 处理文件内容
        let callback = |results: Vec<ValidationResult>, file_name: &str| {
            self.process_results(results, file_name, report_number, create_time);
        };
        FileProcessor::process_file(file_path.to_string(), callback).map_err(|e| e.to_string())
    }

    /// 处理文件解析结果
    fn process_results(&self, results: Vec<ValidationResult>, file_name: &str, report_number: &str, create_time: &str) {
        for result in results {
            let json_value = serde_json::to_value(&result.data).unwrap();
            match file_name.to_lowercase().as_str() {
                s if s.contains("clean") => self.process_clean_data(json_value, report_number, create_time),
                s if s.contains("query") => self.process_query_detail(json_value, report_number, create_time),
                s if s.contains("missing") => self.process_missing_page(json_value, report_number, create_time),
                _ => println!("未知文件类型: {}", file_name),
            }
        }
    }

    /// 处理清理数据
    fn process_clean_data(&self, json_value: serde_json::Value, report_number: &str, create_time: &str) {
        if let Ok(mut data) = serde_json::from_value::<NewProjectDataCleanProgress>(json_value) {
            data.report_number = report_number.to_string();
            data.create_time = create_time.to_string();
            self.clean_data_repository.create_data_clean_progress(data).unwrap();
        } else {
            println!("解析清理数据失败");
        }
    }

    /// 处理查询详情
    fn process_query_detail(&self, json_value: serde_json::Value, report_number: &str, create_time: &str) {
        if let Ok(mut data) = serde_json::from_value::<NewProjectQueryDetail>(json_value) {
            data.report_number = report_number.to_string();
            data.create_time = create_time.to_string();
            self.query_detail_repository.create_query_detail(data).unwrap();
        } else {
            println!("解析查询详情失败");
        }
    }

    /// 处理缺失页面
    fn process_missing_page(&self, json_value: serde_json::Value, report_number: &str, create_time: &str) {
        if let Ok(mut data) = serde_json::from_value::<NewProjectMissingPage>(json_value) {
            data.report_number = report_number.to_string();
            data.create_time = create_time.to_string();
            self.missing_page_repository.create_missing_page(data).unwrap();
        } else {
            println!("解析缺失页面失败");
        }
    }

    /// 构建返回的 JSON 数据
    fn build_response(&self, report_number: &str, create_time: &str, source_files: Vec<String>) -> Response {
        Response {
            valid: true,
            data: ResponseData {
                project_name: "".to_string(),
                report_number: report_number.to_string(),
                source_files,
                create_time: create_time.to_string(),
                state_name: "".to_string(),
            },
        }
    }

    /// 根据项目编号查询报告总数
    pub fn count_reports_by_project_number(&self, project_number: Option<String>) -> Result<i64, String> {

        let count_result = self.repository.count_reports_by_project_number(project_number);

        count_result
    }



    /// 根据项目编号分页查询报告
    pub fn find_reports_paginated(
        &self,
        current_page: i64,
        page_size: i64,
        keyword: Option<String>,
    ) -> Result<ReportListResponse , String> {

        let pagination = Pagination {
            current_page,
            page_size,
            keyword,
        };
        let report_list = self.repository.find_project_reports_paginated(pagination.clone())?;

        // 2. 查询总记录数
        let total = self.repository.count_reports_by_project_number(pagination.keyword)?;
        let data = report_list
            .into_iter()
            .map(|report| {
                // 查询 source_files
                let source_files_list = self.source_repository
                    .find_by_report_number(&report.report_number)
                    .unwrap_or_default(); // 如果查询失败，返回空列表

                println!("{:?}", source_files_list);
                let source_files = source_files_list.into_iter()
                    .map(|source_file| source_file.source_file_name.to_string())
                    .collect::<Vec<String>>();
                ResponseData {
                    project_name: report.project_number.clone(),
                    report_number: report.report_number.clone(),
                    source_files, // 使用查询结果
                    create_time: report.create_time.clone(),
                    state_name: match report.state {
                        0 | 1 => "已导入数据".to_string(),
                        2 => "数据分析完成".to_string(),
                        _ => "未知状态".to_string(),
                    },
                }
            })
            .collect();



        let reports = ReportListResponse {
            valid: true,
            data,
            total,
            current_page,
            page_size,
        };
        Ok(reports)
    }

    // 汇总数据
    pub fn summary_report_data(&self, report_number: &str, project_no: &str) -> Result<SummaryResponse, String> {
        // 单个项目以中心为维度，
        // missing_page -> site_number
        // query_detail -> study_environment_site
        // clean_data_progress -> site (切割)

        // 汇总所有的中心-- 可能存在不全？
        // 汇总所有的数据

        Ok(SummaryResponse {
            valid: false,
            sate: 2,
        })
    }

    pub fn summary_report_detail(&self, report_number: &str) {
        // 获取报告的详细的excel数据

    }

    // 获取每一个详细的数据样子
    pub fn origin_excel_data(&self, report_number: &str, source_file_type: &str) {

    }
}