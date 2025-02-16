use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
use std::env;
use std::path::PathBuf;
// 导入第三方工具依赖包
use dotenv;
use lazy_static::lazy_static;

pub mod core;
pub mod models;

// 导入需要包漏出去的函数所在包
pub mod modules;
pub mod utils;

use core::connections::db::init_pool;

use modules::{
    // 工具
    api::{
        licence_api::send_license,
        project_api::{
            analyze_report_data, delete_project, delete_project_site, fetch_origin_detail,
            fetch_project_list, fetch_report_detail, fetch_report_list, fetch_site_list,
            fetch_supported_template_list, get_project_by_id, handle_site_file,
            handle_template_and_files, save_project, save_project_site,
            update_site_by_id,delete_report_item,
            get_progress
        },
        tools_api::{delete_tool, fetch_tool_list, save_tool, execute_ollama_serve},
    },
    service::{
        projects::{
            project_service::ProjectService, report_service::ProjectReportService,
            site_service::SiteService,
        },
        tools::tool_service::ToolsService,
        user::user_service::UserService,
    },
};
use tauri::{AppHandle, Manager, Emitter};
pub struct AppState {
    pub db_pool: diesel::r2d2::Pool<ConnectionManager<SqliteConnection>>,
    pub user_service: UserService,
    pub tools_service: ToolsService,
    pub project_service: ProjectService,
    pub report_service: ProjectReportService,
    pub site_service: SiteService,
    pub app_handle: AppHandle,

}

lazy_static! {
    // 全局 CONFIG_DIR
    static ref CONFIG_DIR: PathBuf = {
        let config_dir = env::var("CONFIG_PATH").expect("CONFIG_PATH 环境变量未设置");
        PathBuf::from(config_dir)
    };
}
use log::info;
use once_cell::sync::OnceCell;

// 使用线程安全的 OnceCell 存储全局状态
// 使用 once_cell 的 OnceCell 存储全局状态
static GLOBAL_APP_STATE: OnceCell<AppState> = OnceCell::new();

// 提供全局访问函数，用于获取 AppHandle
pub fn get_app_handle() -> Option<&'static AppHandle> {
    GLOBAL_APP_STATE.get().map(|state| &state.app_handle)
}

#[tauri::command]
fn process_message(app_handle: AppHandle) {
    app_handle.emit_to(tauri::EventTarget::any(),"listen_message", format!("目前进度 {}%", 100)).unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv::dotenv().ok();
    // 获取当前项目目录
    let current_dir = env::current_dir().expect("无法获取当前工作目录");
    let log_dir = current_dir.join("logs"); // 在项目根目录下创建 logs 目录

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{} {:?} line:{:?}] {}",
                    record.level(),
                    record.target(),
                    record.line().unwrap(),

                    message
                ))
            })
            .max_file_size(50_000 /* bytes */)
            .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
            .target(tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::Folder {
                file_name: Some("trailq_logs".to_string()),
                path: log_dir, // 指定日志存储路径
            },
        )).build())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // 初始化连接池
            let db_pool = init_pool();
            let user_service = UserService::new(db_pool.clone()); // 初始化 UserService
            let tools_service = ToolsService::new(db_pool.clone()); // 初始化 ToolsService
            let project_service = ProjectService::new(db_pool.clone()); // 初始化 ToolsService
            let report_service = ProjectReportService::new(db_pool.clone()); // 初始化 ToolsService
            let site_service = SiteService::new(db_pool.clone()); // 初始化 ToolsService
            let app_handle = app.handle().clone();
            // 初始化全局状态
            app.manage(AppState {
                app_handle: app_handle.clone(),
                db_pool,
                user_service,
                tools_service,
                project_service,
                report_service,
                site_service,
            });
            info!("TrialQ is started successfully!");
            Ok(())
        })

        .invoke_handler(tauri::generate_handler![
            send_license,
            save_tool,
            fetch_tool_list,
            delete_tool,
            fetch_project_list,
            save_project,
            delete_project,
            get_project_by_id,
            handle_template_and_files,
            fetch_supported_template_list,
            fetch_report_list,
            fetch_site_list,
            save_project_site,
            delete_project_site,
            update_site_by_id,
            handle_site_file,
            analyze_report_data,
            fetch_report_detail,
            fetch_origin_detail,
            delete_report_item,

            get_progress,

            execute_ollama_serve
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
