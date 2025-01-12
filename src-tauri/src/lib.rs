use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
// 导入第三方工具依赖包
use dotenv;

pub mod models;
pub mod core;

// 导入需要包漏出去的函数所在包
pub mod modules;

use core::{
    connections::db::init_pool
};

use modules::{
    // 工具
    api::{
        licence_api::send_license,
        tools_api::{delete_tool, fetch_tool_list, save_tool},
        project_api::{delete_project,
                      fetch_project_list,
                      get_project_by_id,
                      save_project,
                      handle_template_and_files,
                      fetch_supported_template_list
        }
    },
    service::{
        projects::project_service::{ProjectService},
        tools::tool_service::{ToolsService},
        user::user_service::UserService,
    },
};



pub struct AppState {
    pub db_pool: diesel::r2d2::Pool<ConnectionManager<SqliteConnection>>,
    pub user_service: UserService,
    pub tools_service: ToolsService,
    pub project_service: ProjectService,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv::dotenv().ok();
    // 初始化连接池
    let db_pool = init_pool();
    let user_service = UserService::new(db_pool.clone()); // 初始化 UserService
    let tools_service = ToolsService::new(db_pool.clone()); // 初始化 ToolsService
    let project_service = ProjectService::new(db_pool.clone()); // 初始化 ToolsService

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .manage(AppState {
            db_pool,
            user_service,
            tools_service,
            project_service,
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
            fetch_supported_template_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
