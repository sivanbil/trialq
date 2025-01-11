use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
// 导入第三方工具依赖包
use dotenv;


// 导入需要包漏出去的函数所在包
pub mod modules {
    pub mod licence {
        pub mod licence_service;
    }

    pub mod user {
        pub mod user_service;
    }

    pub mod tools {
        pub mod tool_service;
    }

    pub mod projects {
        pub mod project_service;
    }

}

pub mod models {
    pub mod user {
        pub mod schema;

        pub mod user_model;

        pub mod user_repository;
    }

    pub mod tools {
        pub mod tools_model;

        pub mod tools_repository;

        pub mod schema;
    }

    pub mod projects {
        pub mod project_base {
            pub mod project_model;
            pub mod project_repository;
            pub mod schema;

        }

    }
}

pub mod connections;
pub mod utils;


use modules::{
    // 工具
    licence::licence_service::{send_license},
    user::user_service::UserService,

    tools::tool_service::{
        ToolsService,
        save_tool,
        fetch_tool_list,
        delete_tool
    },
    projects::project_service::{
        ProjectService,
        delete_project,
        save_project,
        fetch_project_list,
        get_project_by_id,
    },
};



use connections::db::{init_pool};

pub struct AppState {
    pub db_pool: diesel::r2d2::Pool<ConnectionManager<SqliteConnection>>,
    pub user_service: UserService,
    pub tools_service: ToolsService,
    pub project_service: ProjectService
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    dotenv::dotenv().ok();
    // 初始化连接池
    let db_pool = init_pool();
    let user_service = UserService::new(db_pool.clone());  // 初始化 UserService
    let tools_service = ToolsService::new(db_pool.clone());  // 初始化 ToolsService
    let project_service = ProjectService::new(db_pool.clone());  // 初始化 ToolsService

    tauri::Builder::default()
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
        .manage(AppState { db_pool, user_service, tools_service, project_service})
        .invoke_handler(tauri::generate_handler![
            send_license,
            save_tool,
            fetch_tool_list,
            delete_tool,

            fetch_project_list,
            save_project,
            delete_project,
            get_project_by_id, // 注册新的命令
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
