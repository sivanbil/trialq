use log4rs::init_file;
use std::path::Path;
pub fn init_logging() {
    // 读取 YAML 配置文件
    let config_path = Path::new("logging.yaml");
    // 初始化日志系统
    init_file(config_path, Default::default()).expect("Failed to initialize log4rs from file");
}