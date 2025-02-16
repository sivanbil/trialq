use std::fs;
use std::path::Path;
use std::fmt;
use crate::modules::service::enums::SupportedTemplate;

// 自定义错误类型
#[derive(Debug)]
pub enum EnvCheckError {
    EnvFileNotFound,
    EnvFileReadError(String),
    InvalidEnvLine(String),
    PathNotFound(String),
}

// 实现 Display trait 以便打印错误信息
impl fmt::Display for EnvCheckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvCheckError::EnvFileNotFound => write!(f, ".env 文件不存在"),
            EnvCheckError::EnvFileReadError(err) => write!(f, "无法读取 .env 文件: {}", err),
            EnvCheckError::InvalidEnvLine(line) => write!(f, "无效的环境变量行: {}", line),
            EnvCheckError::PathNotFound(path) => write!(f, "路径不存在: {}", path),
        }
    }
}

pub fn check_env() -> Result<(), EnvCheckError> {
    // 检查 .env 文件是否存在
    if !Path::new(".env").exists() {
        return Err(EnvCheckError::EnvFileNotFound);
    }

    // 读取 .env 文件内容
    let env_contents = fs::read_to_string(".env")
        .map_err(|err| EnvCheckError::EnvFileReadError(err.to_string()))?;

    println!(".env 文件内容:\n{}", env_contents);

    // 解析 .env 文件中的环境变量
    let env_vars: Vec<(&str, &str)> = env_contents
        .lines()
        .filter_map(|line| {
            if line.trim().is_empty() || line.starts_with('#') {
                None // 忽略空行和注释
            } else {
                let parts: Vec<&str> = line.splitn(2, '=').map(|s| s.trim()).collect();
                if parts.len() == 2 {
                    Some((parts[0], parts[1]))
                } else {
                    eprintln!("无效的环境变量行: {}", line);
                    None
                }
            }
        })
        .collect();

    // 检查 CONFIG_PATH 和 UPLOADS_DIR 是否存在
    for (key, value) in env_vars {
        match key {
            "CONFIG_PATH" | "UPLOADS_DIR" => {
                if !Path::new(value).exists() {
                    return Err(EnvCheckError::PathNotFound(format!("{}: {}", key, value)));
                } else {
                    println!("路径 {} 存在: {}", key, value);
                }
            }
            _ => {}
        }
    }

    Ok(())
}

// 自定义错误类型
#[derive(Debug)]
pub enum ConfigCheckError {
    ConfigDirNotFound,
    RequiredConfigMissing(String),
}

// 实现 Display trait 以便打印错误信息
impl fmt::Display for ConfigCheckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigCheckError::ConfigDirNotFound => write!(f, "CONFIG_DIR 目录不存在"),
            ConfigCheckError::RequiredConfigMissing(file) => {
                write!(f, "缺少必要的配置文件: {}", file)
            }
        }
    }
}

pub fn check_configs_exits(tpl_type: &SupportedTemplate) -> Result<(), ConfigCheckError> {
    // 定义 CONFIG_DIR 路径
    let config_dir = "CONFIG_DIR";

    // 检查 CONFIG_DIR 是否存在
    if !Path::new(config_dir).exists() {
        return Err(ConfigCheckError::ConfigDirNotFound);
    }

    // 根据模板类型检查必要的配置文件
    match tpl_type {
        SupportedTemplate::Medidata => {
            let required_files = [
                "missing_page_config.yaml",
                "data_cleaning_config.yaml",
                "query_detail_config.yaml",
            ];

            for file in required_files.iter() {
                let file_path = Path::new(config_dir).join(file);
                if !file_path.exists() {
                    return Err(ConfigCheckError::RequiredConfigMissing(file.to_string()));
                }
            }

            println!("Medidata 模板的所有配置文件均存在");
            Ok(())
        }
        // 其他模板暂时不检测
        _ => {
            println!("暂不支持检测该模板的配置文件");
            Ok(())
        }
    }
}
