use std::time::SystemTime;
// project_service.rs
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;
use crate::models::projects::project_base::project_model::{Project, NewProject};
use crate::models::projects::project_base::project_repository::{ProjectRepository, Pagination};
use date_formatter::utils::{format_date, DateFormat};
pub struct ProjectService {
    repository: ProjectRepository,
}

impl ProjectService {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        let repository = ProjectRepository::new(pool);
        ProjectService { repository }
    }

    /// 获取分页项目列表
    pub fn fetch_project_list(
        &self,
        current_page: i64,
        page_size: i64,
        keyword: Option<String>,
    ) -> Result<(Vec<Project>, i64), String> {
        // 创建分页参数
        let pagination = Pagination {
            current_page,
            page_size,
            keyword: keyword.clone(),
        };

        // 查询分页数据
        let projects = self.repository.read_projects(pagination)?;

        // 查询总记录数
        let total_count = self.repository.count_projects(keyword)?;

        Ok((projects, total_count))
    }

    /// 保存项目
    pub fn save_project(
        &self,
        project_name: String,
        description: String
    ) -> Result<Project, String> {
        // 检查项目是否存在
        if self.not_exist(project_name.clone())? {
            // 创建 NewProject 对象
            let new_project = NewProject {
                project_name,
                description,
                create_time: format_date(SystemTime::now(), "%Y-%m-%d"),
                update_time: format_date(SystemTime::now(), "%Y-%m-%d")
            };

            // 保存项目到数据库
            let project = self.repository.create_project(new_project)?;
            Ok(project)
        } else {
            Err("项目已存在".to_string()) // 返回错误信息
        }
    }

    /// 删除项目
    pub fn delete_project(&self, project_id: i32) -> Result<usize, String> {
        // 调用 repository 的删除方法
        self.repository.delete_project(project_id)
    }

    /// 检查项目是否存在，存在则返回 false，不存在则返回 true
    pub fn not_exist(&self, project_name: String) -> Result<bool, String> {
        let existing_project = self.repository.find_project_by_name(&project_name)?; // 处理 Result

        match existing_project {
            Some(_) => Ok(false), // 项目存在
            None => Ok(true),     // 项目不存在
        }
    }

    /// 根据项目 ID 查询项目信息
    pub fn get_project_by_id(&self, project_id: i32) -> Result<Option<Project>, String> {
        self.repository.find_project_by_id(project_id)
    }
}

#[derive(Serialize)]
pub struct ProjectListResponse {
    valid: bool,
    projects: Vec<Project>, // 项目列表
    total: i64,             // 总记录数
    current_page: i64,      // 当前页码
    page_size: i64,         // 每页大小
}

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
    let (projects, total_count) = project_service.fetch_project_list(current_page, page_size, keyword)?;

    // 返回响应
    Ok(ProjectListResponse {
        valid: true,
        projects,
        total: total_count,
        current_page,
        page_size,
    })
}

#[derive(Serialize)]
pub struct SaveProjectResponse {
    valid: bool,
    message: String,       // 返回的消息
    project: Option<Project>, // 保存后的项目信息
}

#[tauri::command]
pub async fn save_project(
    state: State<'_, AppState>, // 从状态中获取 AppState
    project_name: String,
    description: String,
) -> Result<SaveProjectResponse, String> {
    // 调用 ProjectService 的保存项目方法
    let project_service = &state.project_service;


    match project_service.save_project(project_name, description) {
        Ok(project) => {
            // 保存成功
            Ok(SaveProjectResponse {
                valid: true,
                message: "项目保存成功".to_string(),
                project: Some(project),
            })
        }
        Err(e) => {
            // 保存失败
            Ok(SaveProjectResponse {
                valid: false,
                message: e, // 错误信息
                project: None,
            })
        }
    }
}

#[derive(Serialize)]
pub struct DeleteProjectResponse {
    valid: bool,
    message: String, // 返回的消息
}

#[tauri::command]
pub async fn delete_project(
    state: State<'_, AppState>, // 从状态中获取 AppState
    project_id: i32, // 项目的唯一标识符
) -> Result<DeleteProjectResponse, String> {
    // 调用 ProjectService 的删除项目方法
    let project_service = &state.project_service;

    // 删除项目
    match project_service.delete_project(project_id) {
        Ok(deleted_count) => {
            if deleted_count > 0 {
                // 删除成功
                Ok(DeleteProjectResponse {
                    valid: true,
                    message: "项目删除成功".to_string(),
                })
            } else {
                // 未找到项目
                Ok(DeleteProjectResponse {
                    valid: false,
                    message: "未找到项目".to_string(),
                })
            }
        }
        Err(e) => {
            // 删除失败
            Ok(DeleteProjectResponse {
                valid: false,
                message: e, // 错误信息
            })
        }
    }
}

#[derive(Serialize)]
pub struct GetProjectResponse {
    valid: bool,
    message: String,       // 返回的消息
    project: Option<Project>, // 查询到的项目信息
}

#[tauri::command]
pub async fn get_project_by_id(
    state: State<'_, AppState>, // 从状态中获取 AppState
    project_id: i32, // 项目的唯一标识符
) -> Result<GetProjectResponse, String> {
    // 调用 ProjectService 的查询方法
    let project_service = &state.project_service;

    // 查询项目信息
    match project_service.get_project_by_id(project_id) {
        Ok(Some(project)) => {
            // 查询成功
            Ok(GetProjectResponse {
                valid: true,
                message: "项目查询成功".to_string(),
                project: Some(project),
            })
        }
        Ok(None) => {
            // 未找到项目
            Ok(GetProjectResponse {
                valid: false,
                message: "未找到项目".to_string(),
                project: None,
            })
        }
        Err(e) => {
            // 查询失败
            Ok(GetProjectResponse {
                valid: false,
                message: e, // 错误信息
                project: None,
            })
        }
    }
}