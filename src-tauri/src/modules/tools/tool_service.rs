use diesel::r2d2::{Pool, ConnectionManager};
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;
use crate::models::tools::tools_model::{Tool, NewTool};
use crate::models::tools::tools_repository::{ToolsRepository, Pagination};

pub struct ToolsService {
    repository: ToolsRepository,
}

impl ToolsService {
    pub fn new(pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        let repository = ToolsRepository::new(pool);
        ToolsService { repository }
    }

    /// 获取分页工具列表

    pub fn fetch_tool_list(&self, current_page: i64, page_size: i64) -> Result<(Vec<Tool>, i64), String> {
        // 创建分页参数
        let pagination = Pagination {
            current_page,
            page_size,
        };

        // 查询分页数据
        let tools = self.repository.read_tools(pagination)?;

        // 查询总记录数
        let total_count = self.repository.count_tools()?;

        Ok((tools, total_count))
    }

    /// 保存工具
    pub fn save_tool(&self, title: String, link_url: String) -> Result<Tool, String> {
        // 检查工具是否存在
        if self.not_exit(link_url.clone())? { // 使用 ? 处理 Result
            // 创建 NewTool 对象
            let new_tool = NewTool {
                name: title,
                link_url: link_url.clone(),
                sort_value: 0, // 默认排序值
                is_valid: true, // 默认有效
            };

            // 保存工具到数据库
            let tool = self.repository.create_tool(new_tool)?;
            Ok(tool)
        } else {
            Err("连接已存在".to_string()) // 返回错误信息
        }
    }

    pub fn delete_tool(&self, tool_id: i32) -> Result<usize, String> {
        // 调用 repository 的删除方法
        self.repository.delete_tool(tool_id)
    }

    /// 检查工具是否存在，存在则返回 false，不存在则返回 true
    pub fn not_exit(&self, link_url: String) -> Result<bool, String> {
        let existing_tool = self.repository.find_tool_by_url(&link_url)?; // 处理 Result

        match existing_tool {
            Some(_) => Ok(false), // 工具存在
            None => Ok(true),     // 工具不存在
        }
    }
}


#[derive(Serialize)]
pub struct ToolListResponse {
    valid: bool,
    tools: Vec<Tool>, // 工具列表
    total: i64,       // 总记录数
    current_page: i64, // 当前页码
    page_size: i64,    // 每页大小
}

#[tauri::command]
pub async fn fetch_tool_list(
    state: State<'_, AppState>, // 从状态中获取 AppState
    current_page: i64,
    page_size: i64,
) -> Result<ToolListResponse, String> {
    // 调用 ToolsService 的分页查询方法
    let tools_service = &state.tools_service;


    // 查询分页数据
    let (tools, total_count) = tools_service.fetch_tool_list(current_page, page_size)?;

    // 返回响应
    Ok(ToolListResponse {
        valid: true,
        tools,
        total: total_count,
        current_page,
        page_size,
    })
}

#[derive(Serialize)]
pub struct SaveToolResponse {
    valid: bool,
    message: String, // 返回的消息
    tool: Option<Tool>, // 保存后的工具信息
}

#[tauri::command]
pub async fn save_tool(
    state: State<'_, AppState>, // 从状态中获取 AppState
    title: String,
    link_url: String,
) -> Result<SaveToolResponse, String> {
    // 调用 ToolsService 的保存工具方法
    let tools_service = &state.tools_service;

    // 保存工具
    match tools_service.save_tool(title, link_url) {
        Ok(tool) => {
            // 保存成功
            Ok(SaveToolResponse {
                valid: true,
                message: "工具保存成功".to_string(),
                tool: Some(tool),
            })
        }
        Err(e) => {
            // 保存失败
            Ok(SaveToolResponse {
                valid: false,
                message: e, // 错误信息
                tool: None,
            })
        }
    }
}


#[derive(Serialize)]
pub struct DeleteToolResponse {
    valid: bool,
    message: String, // 返回的消息
}

#[tauri::command]
pub async fn delete_tool(
    state: State<'_, AppState>, // 从状态中获取 AppState
    tool_id: i32, // 工具的唯一标识符
) -> Result<DeleteToolResponse, String> {
    // 调用 ToolsService 的删除工具方法
    let tools_service = &state.tools_service;

    // 删除工具
    match tools_service.delete_tool(tool_id) {
        Ok(deleted_count) => {
            if deleted_count > 0 {
                // 删除成功
                Ok(DeleteToolResponse {
                    valid: true,
                    message: "工具删除成功".to_string(),
                })
            } else {
                // 未找到工具
                Ok(DeleteToolResponse {
                    valid: false,
                    message: "未找到工具".to_string(),
                })
            }
        }
        Err(e) => {
            // 删除失败
            Ok(DeleteToolResponse {
                valid: false,
                message: e, // 错误信息
            })
        }
    }
}