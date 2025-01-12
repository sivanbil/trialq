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
use crate::models::projects::{
    project_report::{
        project_report_model::{ProjectReport, NewProjectReport},
        project_report_repository::ProjectReportRepository,
        project_report_source_repository::ProjectReportSourceRepository,
        project_report_data_repository::ProjectReportDataRepository,
        project_report_extend_data_repository::ProjectReportExtendDataRepository
    }
};

#[derive(Serialize)]
pub struct ResponseData {
    #[serde(rename = "projectName")] // 转换为驼峰命名
    project_name: String,
    #[serde(rename = "reportNumber")] // 转换为驼峰命名
    report_number: String,
    #[serde(rename = "sourceFiles")] // 转换为驼峰命名
    source_files: Vec<i32>,
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
use crate::core::excel_process_engine::{FileProcessor, ValidationResult};
pub struct ProjectReportService {
    repository: Arc<ProjectReportRepository>,
}

impl ProjectReportService {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        let repository = Arc::new(ProjectReportRepository::new(pool));
        ProjectReportService { repository }
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

    pub fn async_process_excel_files(&self, files: Vec<String>) -> Result<Response, String> {
        for (index, file_path) in files.clone().into_iter().enumerate() {
            let callback = |result: ValidationResult| {
                println!("{:?}", result);
                // // 获取数据库连接
                // let create_time = format_date(SystemTime::now(), "%Y-%m-%d");
                // // 1、生成report_number
                // let report_number = format!("REPORT_{}",generate_task_number().to_string());
                //
                // // 2、生成report
                // let new_report = NewProjectReport {
                //     project_number: "".to_string(),
                //     report_number: report_number.clone(),
                //     create_time: format_date(SystemTime::now(), "%Y-%m-%d"),
                //     state: 0,
                //     update_time: "".to_string(),
                // };
                // // 3、生成report_source
                //
                // // 4、存储原始的数据
            };
            FileProcessor::process_file(file_path, callback).expect("TODO: panic message");

        }


        // 5、汇总原始数据存储到报告数据里

        let source_files = vec![1, 2, 3];
        // 构建返回的 JSON 数据
        Ok(Response {
            valid: true,
            data: ResponseData {
                project_name: "".to_string(),
                report_number: "".to_string(),
                source_files,
                create_time: "".to_string(),
                state_name: "".to_string(),
            },
        })
    }

}