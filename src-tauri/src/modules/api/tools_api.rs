use crate::AppState;
use tauri::State;
use crate::modules::service::projects::report_service::SummaryResponse;
use crate::modules::service::tools::tool_service::{
    ToolListResponse,
    SaveToolResponse,
    DeleteToolResponse
};
#[tauri::command]
pub async fn fetch_tool_list(
    state: State<'_, AppState>, // 从状态中获取 AppState
    current_page: i64,
    page_size: i64,
) -> Result<ToolListResponse, String> {
    // 调用 ToolsService 的分页查询方法
    let tools_service = &state.tools_service;

    // 查询分页数据
    tools_service.fetch_tool_list(current_page, page_size)

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
    tools_service.save_tool(title, link_url)
}



#[tauri::command]
pub async fn delete_tool(
    state: State<'_, AppState>, // 从状态中获取 AppState
    tool_id: i32,               // 工具的唯一标识符
) -> Result<DeleteToolResponse, String> {
    // 调用 ToolsService 的删除工具方法
    let tools_service = &state.tools_service;

    // 删除工具
    tools_service.delete_tool(tool_id)
}
