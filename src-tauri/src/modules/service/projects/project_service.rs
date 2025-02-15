use std::time::SystemTime;
// project_service.rs
use crate::models::projects::project_base::project_model::{NewProject, Project};
use crate::models::projects::project_base::project_repository::ProjectRepository;
use crate::models::projects::Pagination;
use date_formatter::utils::format_date;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use serde::Serialize;

#[derive(Serialize)]
pub struct DeleteProjectResponse {
    valid: bool,
    message: String, // 返回的消息
}

#[derive(Serialize)]
pub struct ProjectListResponse {
    valid: bool,
    projects: Vec<Project>, // 项目列表
    total: i64,             // 总记录数
    current_page: i64,      // 当前页码
    page_size: i64,         // 每页大小
}

#[derive(Serialize)]
pub struct SaveProjectResponse {
    valid: bool,
    message: String,          // 返回的消息
    project: Option<Project>, // 保存后的项目信息
}

#[derive(Serialize)]
pub struct SupportedTemplateResponse {
    pub(crate) valid: bool,
    pub(crate) templates: Vec<String>,
}

#[derive(Serialize)]
pub struct ResponseData {
    #[serde(rename = "projectName")] // 转换为驼峰命名
    project_name: String,
    #[serde(rename = "reportNumber")] // 转换为驼峰命名
    report_number: String,
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
pub struct GetProjectResponse {
    valid: bool,
    message: String,          // 返回的消息
    project: Option<Project>, // 查询到的项目信息
}

// 定义返回数据的结构体

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
    ) -> Result<ProjectListResponse, String> {
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

        // 返回响应
        Ok(ProjectListResponse {
            valid: true,
            projects,
            total: total_count,
            current_page,
            page_size,
        })
    }

    /// 保存项目
    pub fn save_project(
        &self,
        project_name: String,
        description: String,
    ) -> Result<SaveProjectResponse, String> {
        // 检查项目是否存在
        let result = if self.not_exist(project_name.clone())? {
            // 创建 NewProject 对象
            let new_project = NewProject {
                project_name,
                description,
                create_time: format_date(SystemTime::now(), "%Y-%m-%d"),
                update_time: format_date(SystemTime::now(), "%Y-%m-%d"),
            };

            // 保存项目到数据库
            let project = self.repository.create_project(new_project)?;
            Ok(project)
        } else {
            Err("项目已存在".to_string()) // 返回错误信息
        };

        match result {
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

    /// 删除项目
    pub fn delete_project(&self, project_id: i32) -> Result<DeleteProjectResponse, String> {
        // 调用 repository 的删除方法
        let result = self.repository.delete_project(project_id);

        match result {
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

    /// 检查项目是否存在，存在则返回 false，不存在则返回 true
    pub fn not_exist(&self, project_name: String) -> Result<bool, String> {
        let existing_project = self.repository.find_project_by_name(&project_name)?; // 处理 Result

        match existing_project {
            Some(_) => Ok(false), // 项目存在
            None => Ok(true),     // 项目不存在
        }
    }

    /// 根据项目 ID 查询项目信息
    pub fn get_project_by_id(&self, project_id: i32) -> Result<GetProjectResponse, String> {
        let result = self.repository.find_project_by_id(project_id);

        match result {
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

    // pub fn async_process_excel_files(&self, files: Vec<String>) -> Result<Response, String> {
    //     // 模拟处理逻辑
    //     let project_name = "Example Project".to_string();
    //     let report_number = "REPORT-12345".to_string();
    //     let create_time = format_date(SystemTime::now(), "%Y-%m-%d");
    //     let state_name = "任务执行中".to_string();
    //     //存储线程句柄
    //     let mut handles = vec![];
    //
    //     // 为每个文件创建一个线程
    //     for (index, file_path) in files.clone().into_iter().enumerate() {
    //         let handle = std::thread::spawn(move || {
    //             // 这里可以添加文件处理逻辑
    //         });
    //         handles.push(handle); // 将线程句柄存储到向量中
    //     }
    //
    //     // 等待所有线程完成
    //     for handle in handles {
    //         handle.join().unwrap(); // 阻塞主线程，直到当前线程完成
    //     }
    //     // 进行数据汇总
    //
    //     // 构建返回的 JSON 数据
    //     Ok(Response {
    //         valid: true,
    //         data: ResponseData {
    //             project_name,
    //             report_number,
    //             create_time,
    //             state_name
    //         },
    //     })
    // }
}
