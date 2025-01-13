use crate::AppState;
use tauri::State;
use crate::modules::{
    service::{
        projects::{
            project_service::{
                ProjectListResponse,
                SaveProjectResponse,
                GetProjectResponse,
                DeleteProjectResponse,
                SupportedTemplateResponse,

            },
            report_service::{
                Response,

            }
        }

    }
};
use crate::modules::service::enums::SupportedTemplate;
use crate::modules::service::projects::report_service::ReportListResponse;

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
    files: Vec<String>, template_name: String, project_number:String) -> Result<Response, String> {
    let report_service = &state.report_service;
    println!("Received template name: {}", template_name);
    println!("Received project_number: {}", project_number);
    println!("Received files:{:?}",{files.clone()});

    report_service.async_process_excel_files(files, project_number)
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
    page_size: i64, keyword: Option<String>) -> Result<ReportListResponse, String> {

    let report_service = &state.report_service;
    let result = report_service.find_reports_paginated(current_page, page_size, keyword);
    result
}
