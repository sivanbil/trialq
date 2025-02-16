use std::path::PathBuf;
// 主要是报告的数据，已经与项目关联的逻辑
// 创建项目报告
// project_report_service.rs
use crate::models::projects::{
    project_report::{
        origin::{
            project_data_clean_progress_model::NewProjectDataCleanProgress,
            project_data_clean_progress_repository::ProjectDataCleanProgressRepository,
            project_missing_page_model::NewProjectMissingPage,
            project_missing_page_repository::ProjectMissingPageRepository,
            project_query_detail_model::NewProjectQueryDetail,
            project_query_detail_repository::ProjectQueryDetailRepository,
        },
        project_report_data_repository::ProjectReportDataRepository,
        project_report_model::{NewProjectReport, ProjectReport},
        project_report_repository::ProjectReportRepository,
        project_report_source_repository::ProjectReportSourceRepository,
    },
    Pagination,
};
use chrono::{Local, NaiveDate};
use date_formatter::utils::{format_timestamp, DateFormat};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use log::info;
use regitry_code::generate_task_number;
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;
use tauri::AppHandle;
use crate::models::projects::project_report::project_report_data_model::ProjectReportData;
use crate::modules::service::projects::site_service::SiteService;
use crate::utils::excel_date_to_naive_date;

#[derive(Serialize)]
pub struct DeleteResponse {
    valid: bool,
    message: String, // 返回的消息
}

#[derive(Serialize)]
pub struct SuccessResponse {
    pub(crate) valid: bool,
    pub(crate) message: String, // 返回的消息
}

#[derive(Serialize)]
pub struct OriginExcelDataResponse {
    valid: bool,
    pub(crate) data: Vec<serde_json::Value>,
}
#[derive(Serialize)]
pub struct ReportListResponse {
    valid: bool,
    data: Vec<ResponseData>, // 项目列表
    total: i64,              // 总记录数
    current_page: i64,       // 当前页码
    page_size: i64,          // 每页大小
}

#[derive(Serialize)]
pub struct ReportDetailResponse {
    valid: bool,
    #[serde(rename = "projectName")]
    project_name: String,
    #[serde(rename = "reportNumber")]
    report_number: String,
    #[serde(rename = "sourceFiles")]
    source_files: Vec<String>,
    #[serde(rename = "createTime")]
    create_time: String,
    state: i32,
    #[serde(rename = "stateName")]
    state_name: String,
    #[serde(rename = "reportData")]
    report_data: Vec<ProjectReportData>, // 返回完整的报告数据
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
    state: i32,
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
use crate::models::projects::project_base::project_site_model::ProjectSite;
use crate::models::projects::project_report::project_report_data_model::NewProjectReportData;
use crate::models::projects::project_report::project_report_source_model::NewProjectReportSource;

pub struct ProjectReportService {
    repository: Arc<ProjectReportRepository>,
    source_repository: Arc<ProjectReportSourceRepository>,
    data_repository: Arc<ProjectReportDataRepository>,
    missing_page_repository: Arc<ProjectMissingPageRepository>,
    query_detail_repository: Arc<ProjectQueryDetailRepository>,
    clean_data_repository: Arc<ProjectDataCleanProgressRepository>,
    site_service: Arc<SiteService>,
}

impl ProjectReportService {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        ProjectReportService {
            site_service: Arc::new(SiteService::new(pool.clone())),
            repository: Arc::new(ProjectReportRepository::new(pool.clone())),
            source_repository: Arc::new(ProjectReportSourceRepository::new(pool.clone())),
            data_repository: Arc::new(ProjectReportDataRepository::new(pool.clone())),
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
    pub fn find_reports_by_project_number(
        &self,
        project_number: &str,
    ) -> Result<Vec<ProjectReport>, String> {
        self.repository
            .find_reports_by_project_number(project_number)
    }

    // 根据报告编号查询报告
    pub fn find_report_by_report_number(
        &self,
        report_number: &str,
    ) -> Result<Option<ProjectReport>, String> {
        self.repository.find_report_by_report_number(report_number)
    }

    // 根据报告 ID 查询报告
    pub fn find_report_by_id(&self, report_id: i32) -> Result<Option<ProjectReport>, String> {
        self.repository.find_report_by_id(report_id)
    }

    // 更新报告
    pub fn update_report(
        &self,
        report_id: i32,
        updated_report: NewProjectReport,
    ) -> Result<usize, String> {
        self.repository.update_report(report_id, updated_report)
    }

    // 删除报告
    pub fn delete_report(&self, report_number: String) -> Result<DeleteResponse, String> {
        if report_number.clone() != "" {
            self.repository
                .delete_report(report_number.clone())
                .expect("TODO: panic message");
            self.source_repository
                .delete(report_number.clone())
                .expect("TODO: panic message");
            self.data_repository
                .delete_report_data(report_number.clone())
                .expect("TODO: panic message");
            self.query_detail_repository
                .delete_query_detail(report_number.clone())
                .expect("TODO: panic message");
            self.missing_page_repository
                .delete_missing_page(report_number.clone())
                .expect("TODO: panic message");
            self.clean_data_repository
                .delete_data_clean_progress(report_number.clone())
                .expect("TODO: panic message");
            Ok(DeleteResponse {
                valid: true,
                message: "success".to_string(),
            })
        } else {
            Ok(DeleteResponse {
                valid: false,
                message: "not found".to_string(),
            })
        }
    }

    pub fn async_process_excel_files(
        &self,
        app_handle: AppHandle,
        files: Vec<String>,
        project_number: String,
    ) -> Result<Response, String> {
        // 1. 生成报告编号和创建时间
        let create_time = format_timestamp((Local::now().timestamp()) as u64, DateFormat::DateTime);
        let report_number = format!("{}", generate_task_number());

        // 2. 创建项目报告
        self.create_project_report(&project_number, &report_number, &create_time)?;

        // 3. 处理每个文件
        let mut miss_indexes = vec![];
        let mut miss_index = 0;
        // 遍历 files，记录缺失的文件索引
        for file_path in files {
            if file_path.clone() != "" {
                let file_path_buf = PathBuf::from(file_path.clone());
                let file_name = file_path_buf
                    .file_stem()
                    .and_then(|name| name.to_str())
                    .unwrap_or_default();

                // 如果 file_path 是 Some，调用 process_file 处理
                self.process_file(app_handle.clone(), &file_path,file_name, &project_number, &report_number, &create_time)?;
            } else {
                // 如果 file_path 是 None，记录缺失的索引
                miss_indexes.push(miss_index);
            }
            miss_index += 1; // 更新索引
        }

        // 示例的 source_files
        let mut source_files = vec![
            "query_detail".to_string(),
            "data_clean_progress".to_string(),
            "missing_pages".to_string(),
        ];

        info!("报告处理完成");

        // 根据缺失的索引移除 source_files 中的对应元素
        for &index in miss_indexes.iter().rev() {
            if index < source_files.len() {
                source_files.remove(index);
            }
        }

        Ok(self.build_response(&report_number, &create_time, source_files))
    }

    /// 创建项目报告
    fn create_project_report(
        &self,
        project_number: &str,
        report_number: &str,
        create_time: &str,
    ) -> Result<ProjectReport, String> {
        let report = NewProjectReport {
            project_number: project_number.to_string(),
            report_number: report_number.to_string(),
            state: 1, // 1 导入基础数据 2 做完数据分析
            create_time: create_time.to_string(),
            update_time: "".to_string(),
        };
        self.repository
            .create_report(report)
            .map_err(|e| e.to_string())
    }

    /// 处理单个文件
    fn process_file(
        &self,
        app_handle: AppHandle,
        file_path: &str,
        file_name: &str,
        project_number: &str,
        report_number: &str,
        create_time: &str,
    ) -> Result<(), String> {
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
        self.source_repository
            .create(source)
            .map_err(|e| e.to_string())?;


        crate::utils::handle_progress_events(app_handle.clone(), "import_data_progress", file_name, 1u8);

        // 处理文件内容
        let callback = |results: Vec<ValidationResult>, file_name: &str| {
            self.process_results(app_handle.clone(), results, file_name, report_number, create_time);
        };
        FileProcessor::process_file(app_handle.clone(), file_path.to_string(), callback).map_err(|e| e.to_string())
    }

    /// 处理文件解析结果
    fn process_results(
        &self,
        app_handle: AppHandle,
        results: Vec<ValidationResult>,
        file_name: &str,
        report_number: &str,
        create_time: &str,
    ) {
        // 将 results 按每 200 个一批进行拆分
        let total = results.clone().chunks(200).len();
        let mut processed_index = 0;
        for batch in results.chunks(200) {
            processed_index +=1;
            crate::utils::handle_progress_events(app_handle.clone(), "import_data_progress", file_name, crate::utils::calculate_percentage_u8(processed_index as f64, total as f64));
            std::thread::sleep(std::time::Duration::from_millis(100));
            let batch_results: Vec<ValidationResult> = batch.to_vec();
            let json_values: Vec<serde_json::Value> = batch_results
                .into_iter()
                .map(|result| serde_json::to_value(&result.data).unwrap())
                .collect();
            match file_name.to_lowercase().as_str() {
                s if s.contains("clean") => {
                    self.batch_process_clean_data(json_values, report_number, create_time)
                }
                s if s.contains("query") => {
                    self.batch_process_query_detail(json_values, report_number, create_time)
                }
                s if s.contains("missing") => {
                    self.batch_process_missing_page(json_values, report_number, create_time)
                }
                _ => println!("未知文件类型: {}", file_name),
            }
        }
    }

    /// 处理清理数据
    fn process_clean_data(
        &self,
        json_value: serde_json::Value,
        report_number: &str,
        create_time: &str,
    ) {
        if let Ok(mut data) = serde_json::from_value::<NewProjectDataCleanProgress>(json_value) {
            data.report_number = report_number.to_string();
            data.create_time = create_time.to_string();
            self.clean_data_repository
                .create_data_clean_progress(data)
                .unwrap();
        } else {
            println!("解析清理数据失败");
        }
    }

    /// 处理查询详情
    fn process_query_detail(
        &self,
        json_value: serde_json::Value,
        report_number: &str,
        create_time: &str,
    ) {
        if let Ok(mut data) = serde_json::from_value::<NewProjectQueryDetail>(json_value) {
            data.report_number = report_number.to_string();
            data.create_time = create_time.to_string();
            self.query_detail_repository
                .create_query_detail(data)
                .unwrap();
        } else {
            println!("解析查询详情失败");
        }
    }

    /// 处理缺失页面
    fn process_missing_page(
        &self,
        json_value: serde_json::Value,
        report_number: &str,
        create_time: &str,
    ) {
        if let Ok(mut data) = serde_json::from_value::<NewProjectMissingPage>(json_value) {
            data.report_number = report_number.to_string();
            data.create_time = create_time.to_string();
            self.missing_page_repository
                .create_missing_page(data)
                .unwrap();
        } else {
            println!("解析缺失页面失败");
        }
    }

    fn batch_process_clean_data(
        &self,
        json_values: Vec<Value>,
        report_number: &str,
        create_time: &str,
    ) {
        let mut valid_data = Vec::new();
        for json_value in json_values {
            if let Ok(mut data) = serde_json::from_value::<NewProjectDataCleanProgress>(json_value)
            {
                data.report_number = report_number.to_string();
                data.create_time = create_time.to_string();
                valid_data.push(data);
            } else {
                println!("解析清理数据失败");
            }
        }
        if !valid_data.is_empty() {
            self.clean_data_repository
                .batch_create_data_clean_progress(valid_data)
                .unwrap();
        }
    }
    fn batch_process_query_detail(
        &self,
        json_values: Vec<Value>,
        report_number: &str,
        create_time: &str,
    ) {
        let mut valid_data = Vec::new();
        for json_value in json_values {
            if let Ok(mut data) = serde_json::from_value::<NewProjectQueryDetail>(json_value) {
                data.report_number = report_number.to_string();
                data.create_time = create_time.to_string();
                valid_data.push(data);
            } else {
                println!("解析查询详情失败");
            }
        }
        if !valid_data.is_empty() {
            self.query_detail_repository
                .batch_create_query_detail(valid_data)
                .unwrap();
        }
    }
    fn batch_process_missing_page(
        &self,
        json_values: Vec<Value>,
        report_number: &str,
        create_time: &str,
    ) {
        let mut valid_data = Vec::new();
        for json_value in json_values {
            if let Ok(mut data) = serde_json::from_value::<NewProjectMissingPage>(json_value) {
                data.report_number = report_number.to_string();
                data.create_time = create_time.to_string();
                valid_data.push(data);
            } else {
                println!("解析缺失页面失败");
            }
        }
        if !valid_data.is_empty() {
            self.missing_page_repository
                .batch_create_missing_page(valid_data)
                .unwrap();
        }
    }

    /// 构建返回的 JSON 数据
    fn build_response(
        &self,
        report_number: &str,
        create_time: &str,
        source_files: Vec<String>,
    ) -> Response {
        Response {
            valid: true,
            data: ResponseData {
                project_name: "".to_string(),
                report_number: report_number.to_string(),
                source_files,
                create_time: create_time.to_string(),
                state: 1,
                state_name: "".to_string(),
            },
        }
    }

    /// 根据项目编号查询报告总数
    pub fn count_reports_by_project_number(
        &self,
        project_number: Option<String>,
    ) -> Result<i64, String> {
        let count_result = self
            .repository
            .count_reports_by_project_number(project_number);

        count_result
    }

    /// 根据项目编号分页查询报告
    pub fn find_reports_paginated(
        &self,
        current_page: i64,
        page_size: i64,
        keyword: Option<String>,
    ) -> Result<ReportListResponse, String> {
        let pagination = Pagination {
            current_page,
            page_size,
            keyword,
        };
        let report_list = self
            .repository
            .find_project_reports_paginated(pagination.clone())?;

        // 2. 查询总记录数
        let total = self
            .repository
            .count_reports_by_project_number(pagination.keyword)?;
        let data = report_list
            .into_iter()
            .map(|report| {
                // 查询 source_files
                let source_files_list = self
                    .source_repository
                    .find_by_report_number(&report.report_number)
                    .unwrap_or_default(); // 如果查询失败，返回空列表

                let source_files = source_files_list
                    .into_iter()
                    .map(|source_file| source_file.source_file_name.to_string())
                    .collect::<Vec<String>>();
                ResponseData {
                    project_name: report.project_number.clone(),
                    report_number: report.report_number.clone(),
                    source_files, // 使用查询结果
                    create_time: report.create_time.clone(),
                    state: report.state.clone(),
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
    pub fn summary_report_data(
        &self,
        app: AppHandle,
        report_number: &str,
        project_no: &str,
    ) -> Result<SummaryResponse, String> {
        // 单个项目以中心为维度，
        let result_report = self
            .repository
            .find_report_by_report_number(report_number)?;

        let report = result_report.unwrap();
        let report_id = report.id;
        // 先看看目前维护的中心是否全乎，如果不存在，就维护进去
        self.process_site_numbers(project_no.to_string().clone())
            .expect("TODO: panic message");
        // 获取该项目的所有中心
        let mut current_page = 1;
        let page_size = 10;
        let mut total_pages = 1;
        while total_pages >= current_page {
            let result = self.site_service.fetch_site_list(
                current_page,
                page_size,
                project_no.parse().unwrap(),
                None,
            );
            if let Ok(site_list_response) = result {
                // 成功获取 SiteListResponse
                self.summary_every_site_data(report_number, site_list_response.sites);
                total_pages = (site_list_response.total as f64 / page_size as f64).ceil() as i64;
                current_page += 1;
            } else {
                // 处理错误情况
                println!("获取站点列表失败: {:?}", result.err());
                break;
            }
        }
        self.repository
            .update_report_state(report_id, 2)
            .expect("TODO: panic message");
        Ok(SummaryResponse {
            valid: true,
            sate: 2,
        })
    }

    pub fn summary_every_site_data(&self, report_number: &str, sites: Vec<ProjectSite>) {
        sites.iter().for_each(|site| {
            let site_number = site.site_number.clone();
            let vec_result = self
                .missing_page_repository
                .find_missing_page_stats(report_number, &(site_number.clone()));

            let mut missing_pages = 0;
            let mut md_gt7 = 0;
            let mut md_gt14 = 0;
            match vec_result {
                Ok(result) => {
                    missing_pages = result.data_page_count;
                    md_gt7 = result.gt_7;
                    md_gt14 = result.gt_14;
                }
                _ => {}
            }

            let clean_data_stats = self
                .clean_data_repository
                .find_data_clean_progress_stats(report_number, &site_number)
                .unwrap();
            // clean文件里面的记录
            let pages = clean_data_stats.total_pages; // 总页数;
                                                      // 查询所有entered
            let pages_entered = clean_data_stats.total_entered; // 已录入的页数;
                                                                // 汇总所有为1的verify required
            let sdv_backlog = clean_data_stats.total_verify_required; // 需要验证的页数;

            let percent_pages_entered = if pages > 0 {
                format!("{:.2}", (pages_entered as f64 / pages as f64) * 100.0)
            } else {
                "0.0".to_string()
            };

            let percent_pages_sdved = if pages > 0 {
                format!(
                    "{:.2}",
                    ((pages_entered - sdv_backlog) as f64 / pages_entered as f64) * 100.0
                )
            } else {
                "0.0".to_string()
            };

            // 查询查询详情统计数据
            let query_stats = self
                .query_detail_repository
                .get_query_statistics(report_number, &site_number)
                .unwrap();

            let answered_query = query_stats.answered_query_count;
            let opened_query = query_stats.opened_query_count;
            let op_gt7 = query_stats.query_age_distribution.more_than_7_days;
            let op_gt14 = query_stats.query_age_distribution.more_than_14_days;
            let op_gt21 = query_stats.query_age_distribution.more_than_21_days;
            let op_gte30 = query_stats.query_age_distribution.more_than_30_days;

            // 看看每个中心总页数
            // missing-subject_id
            // data_clean -subject
            // query_detail - subject_name
            // 以上数据取唯一，按照每个报告的每个site进行分组
            let data = NewProjectReportData {
                site: site.site_number.to_string(),
                site_name: site.site_name.to_string(),
                cra: site.site_cra.to_string(),
                pages: pages.try_into().unwrap(),
                pages_entered: pages_entered.try_into().unwrap(),
                missing_pages,
                md_gt7,
                md_gt14,
                sdv_backlog: sdv_backlog.try_into().unwrap(),
                edc_status_comment: "".to_string(),
                percent_pages_entered,
                percent_pages_sdved,
                answered_query: answered_query.try_into().unwrap(),
                opened_query: opened_query.try_into().unwrap(),
                op_gt7,
                op_gt14,
                op_gt21,
                op_gt30: op_gte30,
                report_number: format!("{}", report_number),
            };
            self.data_repository
                .create_report_data(data)
                .expect("TODO: panic message");
        });
    }

    pub fn summary_report_detail(
        &self,
        report_number: &str,
    ) -> Result<ReportDetailResponse, String> {
        // 获取报告基本信息
        let report = self
            .repository
            .find_report_by_report_number(report_number)?;
        let report = report.ok_or("Report not found")?;

        // 获取报告数据
        let report_data = self.data_repository.find_report_data_by_no(report_number)?;

        // 获取报告源文件
        let source_files = self
            .source_repository
            .find_by_report_number(report_number)
            .unwrap()
            .into_iter()
            .map(|source| source.source_file_name)
            .collect();

        // 构建返回的 JSON 数据
        Ok(ReportDetailResponse {
            valid: true,
            project_name: report.project_number,
            report_number: report.report_number,
            source_files,
            create_time: report.create_time,
            state: report.state,
            state_name: match report.state {
                0 | 1 => "已导入数据".to_string(),
                2 => "数据分析完成".to_string(),
                _ => "未知状态".to_string(),
            },
            report_data, // 返回完整的报告数据
        })
    }

    // 获取原始 Excel 数据
    pub fn origin_excel_data(
        &self,
        report_number: &str,
        source_file_name: &str,
        current_page: i64,
        page_size: i64,
    ) -> Result<OriginExcelDataResponse, String> {
        // 根据文件名判断文件类型
        let source_file_type = match source_file_name.to_lowercase().as_str() {
            s if s.contains("query") => "query_detail",
            s if s.contains("clean") => "data_clean_progress",
            s if s.contains("missing") => "missing_pages",
            _ => return Err("Unknown file type".to_string()),
        };

        let pagination = Pagination {
            current_page,
            page_size,
            keyword: None,
        };
        info!("pagination：{:?}",pagination);
        // 根据文件类型查询相应的数据
        let data = match source_file_type {
            "query_detail" => {
                self.query_detail_repository
                    .find_query_details_by_report_number(report_number, pagination)?
                    .into_iter()
                    .map(|mut d| {
                        let date =
                            excel_date_to_naive_date((&d.qry_open_date_localized).parse().unwrap());
                        let formatted = date.format_with_items(
                            chrono::format::strftime::StrftimeItems::new("%d-%B-%y"),
                        );
                        d.qry_open_date_localized = formatted.to_string();
                        let mut value = serde_json::to_value(&d).unwrap();

                        if let Some(pos) = d.qry_open_date.find(' ') {
                            let output = &d.qry_open_date[0..pos];
                            // 解析日期字符串，并将结果绑定到一个新的变量名
                            let parsed_qry_open_date =
                                NaiveDate::parse_from_str(output, "%m/%d/%Y").unwrap();

                            let current_date = Local::now().naive_local().date();
                            // 计算日期差值
                            println!(
                                "current_date:{}, parsed_qry_open_date: {:?}",
                                current_date, parsed_qry_open_date
                            );
                            let days = (current_date - parsed_qry_open_date).num_days();
                            if let serde_json::Value::Object(ref mut obj) = value {
                                obj.insert(
                                    "op_gt7".to_string(),
                                    serde_json::Value::Number(((days > 7) as u8).into()),
                                );
                                obj.insert(
                                    "op_gt14".to_string(),
                                    serde_json::Value::Number(((days > 14) as u8).into()),
                                );
                                obj.insert(
                                    "op_gt21".to_string(),
                                    serde_json::Value::Number(((days > 21) as u8).into()),
                                );
                                obj.insert(
                                    "op_gt30".to_string(),
                                    serde_json::Value::Number(((days >= 30) as u8).into()),
                                );
                            }
                        }
                        value
                    })
                    .collect()
            }
            "data_clean_progress" => self
                .clean_data_repository
                .find_data_clean_progress_by_report_number(report_number, pagination)?
                .into_iter()
                .map(|d| serde_json::to_value(d).unwrap())
                .collect(),
            "missing_pages" => self
                .missing_page_repository
                .find_missing_pages_by_report_number(report_number, pagination)?
                .into_iter()
                .map(|d| {
                    let mut value = serde_json::to_value(&d).unwrap();
                    let days = d.days_of_missing_pages;
                    if let serde_json::Value::Object(ref mut obj) = value {
                        obj.insert(
                            "md_gt7".to_string(),
                            serde_json::Value::Number(((days > 7) as u8).into()),
                        );
                        obj.insert(
                            "md_gt14".to_string(),
                            serde_json::Value::Number(((days > 14) as u8).into()),
                        );
                    }
                    value
                })
                .collect(),
            _ => return Err("Unknown file type".to_string()),
        };
        // 封装结果到结构体
        Ok(OriginExcelDataResponse {
            valid: true, // 固定为 true
            data,
        })
    }

    // 在某个服务或逻辑层中调用
    pub fn process_site_numbers(&self, project_number: String) -> Result<(), String> {
        let missing_page_site_numbers = self.missing_page_repository.find_unique_site_numbers()?;
        let query_detail_site_numbers = self.clean_data_repository.find_site_numbers()?;
        let clean_data_site_numbers = self
            .query_detail_repository
            .find_unique_study_environment_sites()?;

        // 3. 合并两个列表并去重
        let mut all_site_numbers: Vec<String> = missing_page_site_numbers
            .into_iter()
            .chain(query_detail_site_numbers.into_iter())
            .chain(clean_data_site_numbers.into_iter())
            .map(|s| s.trim().to_string()) // 去除空格
            .collect();
        all_site_numbers.sort();
        all_site_numbers.dedup(); // 去重
                                  // 4. 遍历所有 site_number，调用 SiteService 检查并插入
        for site_number in all_site_numbers {
            // 检查站点是否存在
            if self.site_service.not_exist(site_number.clone())? {
                // 如果不存在，则插入
                let _ = self.site_service.save_site(
                    project_number.clone(),
                    site_number.clone(),
                    None, // site_name
                    None, // site_cra
                )?;
            }
        }

        Ok(())
    }
}
