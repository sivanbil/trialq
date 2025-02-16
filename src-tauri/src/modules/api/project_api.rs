use std::collections::HashMap;
use log::info;
use crate::models::projects::project_base::project_site_model::NewProjectSite;
use crate::modules::service::enums::SupportedTemplate;
use crate::modules::service::projects::report_service::{DeleteResponse, OriginExcelDataResponse, ReportDetailResponse, ReportListResponse, SuccessResponse, SummaryResponse};
use crate::modules::service::projects::site_service::{
    DeleteSiteResponse, ImportSiteResponse, SaveSiteResponse, SiteListResponse,
};
use crate::modules::service::projects::{
    project_service::{
        DeleteProjectResponse, GetProjectResponse, ProjectListResponse, SaveProjectResponse,
        SupportedTemplateResponse,
    },
    report_service::Response,
};
use crate::{AppState};
use tauri::{AppHandle, State};


#[tauri::command]
pub async fn fetch_project_list(
    state: State<'_, AppState>, // 从状态中获取 AppState
    current_page: i64,
    page_size: i64,
    keyword: Option<String>,
) -> Result<ProjectListResponse, String> {
    // 调用 ProjectService 的分页查询方法
    let project_service = &state.project_service;

    // 查询分页数据
    project_service.fetch_project_list(current_page, page_size, keyword)
}

#[tauri::command]
pub async fn save_project(
    state: State<'_, AppState>, // 从状态中获取 AppState
    project_name: String,
    description: String,
) -> Result<SaveProjectResponse, String> {
    // 调用 ProjectService 的保存项目方法
    let project_service = &state.project_service;

    project_service.save_project(project_name, description)
}

#[tauri::command]
pub async fn delete_project(
    state: State<'_, AppState>, // 从状态中获取 AppState
    project_id: i32,            // 项目的唯一标识符
) -> Result<DeleteProjectResponse, String> {
    // 调用 ProjectService 的删除项目方法
    let project_service = &state.project_service;

    // 删除项目
    project_service.delete_project(project_id)
}

#[tauri::command]
pub async fn get_project_by_id(
    state: State<'_, AppState>, // 从状态中获取 AppState
    project_id: i32,            // 项目的唯一标识符
) -> Result<GetProjectResponse, String> {
    // 调用 ProjectService 的查询方法
    let project_service = &state.project_service;

    // 查询项目信息
    project_service.get_project_by_id(project_id)
}

// 定义一个 Tauri 命令
#[tauri::command]
pub async fn handle_template_and_files(
    state: State<'_, AppState>, // 从状态中获取 AppState
    app: AppHandle,
    files: Vec<String>,
    template_name: String,
    project_no: String,
) -> Result<Response, String> {
    let report_service = &state.report_service;
    info!("Received template name: {}", template_name);
    info!("Received project_no: {}", project_no);
    info!("Received files:{:?}", { files.clone() });

    report_service.async_process_excel_files(app, files, project_no)
}

#[tauri::command]
pub async fn fetch_supported_template_list(
    _state: State<'_, AppState>, // 从状态中获取 AppState
) -> Result<SupportedTemplateResponse, String> {
    // 获取所有支持的模板名称
    let templates = SupportedTemplate::all_supported_templates();

    let valid = true;
    // 返回结果
    Ok(SupportedTemplateResponse { valid, templates })
}

#[tauri::command]
pub async fn fetch_report_list(
    state: State<'_, AppState>,
    current_page: i64,
    page_size: i64,
    keyword: Option<String>,
) -> Result<ReportListResponse, String> {
    let report_service = &state.report_service;
    let result = report_service.find_reports_paginated(current_page, page_size, keyword);
    result
}

// 获取分页的项目列表
#[tauri::command]
pub async fn fetch_site_list(
    state: State<'_, AppState>,
    current_page: i64,
    page_size: i64,
    project_no: String,
    keyword: Option<String>,
) -> Result<SiteListResponse, String> {
    info!("获取{}中心", project_no);
    let service = &state.site_service;
    let result = service.fetch_site_list(current_page, page_size, project_no, keyword);
    result
}

// 存储项目中心
#[tauri::command]
pub async fn save_project_site(
    state: State<'_, AppState>,
    project_name: String,
    site_number: String,
    site_name: Option<String>,
    site_cra: Option<String>,
) -> Result<SaveSiteResponse, String> {
    let service = &state.site_service;
    let result = service.save_site(project_name, site_number, site_name, site_cra);
    result
}

#[tauri::command]
pub async fn delete_project_site(
    state: State<'_, AppState>,
    site_id: i32,
) -> Result<DeleteSiteResponse, String> {
    let service = &state.site_service;
    let result = service.delete_site(site_id);
    result
}
#[tauri::command]
pub async fn update_site_by_id(
    state: State<'_, AppState>,
    site_id: i32,
    project_name: String,
    site_number: String,
    site_name: Option<String>,
    site_cra: Option<String>,
) -> Result<SaveSiteResponse, String> {
    let service = &state.site_service;

    let result = service.update_site_by_id(
        site_id,
        NewProjectSite {
            project_name,
            site_number,
            site_name: site_name.unwrap_or(String::from("")),
            site_cra: site_cra.unwrap_or(String::from("")),
        },
    );
    result
}

#[tauri::command]
pub async fn handle_site_file(
    state: State<'_, AppState>, // 从状态中获取 AppState
    app_handle: AppHandle,
    file_path: String,
    project_number: String,
) -> Result<ImportSiteResponse, String> {
    println!("{:?}", file_path);
    println!("{:?}", project_number);
    let service = &state.site_service;

    service.async_process_excel_files(app_handle,file_path, project_number)
}

#[tauri::command]
pub async fn analyze_report_data(
    state: State<'_, AppState>, // 从状态中获取 AppState
    app: AppHandle,
    report_number: String,
    project_number: String,
) -> Result<SummaryResponse, String> {
    let service = &state.report_service;

    // 删除工具
    service.summary_report_data(app, &*report_number, &*project_number)
}

#[tauri::command]
pub async fn fetch_report_detail(
    state: State<'_, AppState>,
    report_number: String,
) -> Result<ReportDetailResponse, String> {
    let service = &state.report_service;
    service.summary_report_detail(&*report_number)
}

#[tauri::command]
pub async fn delete_report_item(
    state: State<'_, AppState>,
    report_number: String,
) -> Result<DeleteResponse, String> {
    let service = &state.report_service;
    service.delete_report(report_number)
}

#[tauri::command]
pub async fn fetch_origin_detail(
    state: State<'_, AppState>,
    app: AppHandle,
    report_number: String,
    source_file_name: String,
    table_header: Vec<String>,
    table_header_map: HashMap<String, String>
) -> Result<SuccessResponse, String> {
    let service = &state.report_service;
    let mut current_page = 1;
    let page_size = 500;
    info!("table_header:{:?}", table_header);
    let mut all_responses: Vec<OriginExcelDataResponse> = Vec::new();

    info!("fetch data");
    loop {
        info!("fetch data current_page:{}", current_page);

        let result = service.origin_excel_data(&report_number, &source_file_name, current_page, page_size);

        match result {
            Ok(response) => {
                if response.data.is_empty() {
                    break;
                }
                all_responses.push(response);
                current_page += 1;
            }
            Err(err) => {
                return Err(err);
            }
        }
    }
    info!("start export excel");
    crate::utils::export_excel(app, table_header, table_header_map, all_responses, source_file_name)?;
    Ok(SuccessResponse {
        valid: true,
        message: "success".to_string()
    })
}

use tauri::{Emitter};
#[tauri::command]
pub async fn get_progress(state: State<'_, AppState>,
                          app: AppHandle) -> Result<SuccessResponse, String> {
    // 模拟进度更新
    for progress in 0..=100 {
        // 每隔 500 毫秒发送一次进度
        std::thread::sleep(std::time::Duration::from_millis(500));
        // 发送进度事件到前端
        app.emit("progress_update", progress).map_err(|e| e.to_string())?;
    }
    Ok(SuccessResponse {
        valid: false,
        message: "com".to_string(),
    })
}




