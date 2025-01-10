use serde::{Deserialize, Serialize};

use std::{env, panic};
use std::time::SystemTime;
use crate::modules::user::user_service::UserService;
use tauri::State;
use crate::AppState;

use crate::models::user::user_model::NewUser;
use regitry_code::{read_key_file, decode_code, generate_random_string, verify_password, encrypt_password};
use date_formatter::utils::format_date;



// 定义许可证验证响应的结构
#[derive(Serialize, Deserialize)]
pub struct LicenseResponse {
    valid: bool,
    expire_date: String,
    message: String,
}

// Tauri 命令：发送许可证到后端验证
// 校验通过后，就
#[tauri::command]
pub async fn send_license(
    state: State<'_, AppState>,  // 从状态中获取 AppState
    email: String,
    license_key: String) -> Result<LicenseResponse, String> {

    let code = &*license_key;
    // 获取环境变量 KEY_PATH
    let private_key_path = env::var("KEY_PATH").expect("KEY_PATH environment variable not set");

    // 读取私钥
    let private_key = read_key_file(private_key_path);

    // 解码注册码
    // 使用 catch_unwind 捕获 panic
    let result = panic::catch_unwind(move || {
        decode_code(code, &*private_key)
    });

    // 处理捕获的结果
    match result {
        Ok((decode_email, decode_expire_time)) => {
            // 校验 email 是否匹配
            if email != decode_email {
                return Err("Failed to validate license.error code: 100001".to_string())
            }

            let response: LicenseResponse = LicenseResponse {
                valid: true,
                expire_date: decode_expire_time.clone().to_string(),
                message: "License is valid".to_string(),
            };


            // 调用 UserService 创建用户
            state.user_service.upsert_or_save_user(email, license_key, decode_expire_time.to_string())
                .map_err(|e| format!("Failed to create user: {}", e))?;

            Ok(response)
        }
        Err(_) => {
            Err("Failed to decode code: panic occurred".parse().unwrap())
        }
    }

}