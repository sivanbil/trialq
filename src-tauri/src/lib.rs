use diesel::r2d2::ConnectionManager;
use diesel::SqliteConnection;
// 导入第三方工具依赖包
use dotenv;
use tauri::Manager;


// 导入需要包漏出去的函数所在包
pub mod modules {
    pub mod licence {
        pub mod licence_service;
    }

    pub mod user {
        pub mod user_service;
    }

}

pub mod models {
    pub mod user {
        pub mod schema;

        pub mod user_model;

        pub mod user_repository;
    }
}

pub mod connections;

use modules::{
    // 工具
    // 注册码
    licence::licence_service::{send_license},
};



use crate::connections::db::{init_pool};
use crate::modules::user::user_service::UserService;

pub struct AppState {
    pub db_pool: diesel::r2d2::Pool<ConnectionManager<SqliteConnection>>,
    pub user_service: UserService,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    dotenv::dotenv().ok();
    // 初始化连接池
    let db_pool = init_pool();
    let user_service = UserService::new(db_pool.clone());  // 初始化 UserService

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
        .manage(AppState { db_pool, user_service })
        .invoke_handler(tauri::generate_handler![
            send_license
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
