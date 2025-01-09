use serde::{Deserialize, Serialize};
use regitry_code::{read_key_file, decode_code};
use std::env;

// 定义许可证验证响应的结构
#[derive(Serialize, Deserialize)]
pub struct LicenseResponse {
    valid: bool,
    message: String,
}

// Tauri 命令：发送许可证到后端验证
#[tauri::command]
pub async fn send_license(email: String,license_key: String) -> Result<LicenseResponse, String> {

    let code = &*license_key;
    // 获取环境变量 KEY_PATH
    let private_key_path = env::var("KEY_PATH").expect("KEY_PATH environment variable not set");

    // 读取私钥
    let private_key = read_key_file(private_key_path);

    // 解码注册码
    let (decode_email, decode_expire_time) = decode_code(code, &private_key);
    // 校验 email 是否匹配
    if email != decode_email {
        return Err("Failed to validate license.error code: 100001".to_string())
    }
    println!("Expire time: {}", decode_expire_time);
    let response: LicenseResponse = LicenseResponse {
        valid: true,
        message: "License is valid".to_string(),
    };
    Ok(response)
}