use crate::CONFIG_DIR;
use calamine::{open_workbook, Reader, Xls, Xlsx};
use csv::ReaderBuilder;
use log::{debug, error, info};
use mlua::Lua;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use tauri::{AppHandle, Emitter};
// 定义 DSL 规则的结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
struct HeaderRule {
    name: String,      // 表头名称
    alias: String,     // 验证规则的别名（用于生成列名）
    data_type: String, // 数据类型（number, string, date）
    required: bool,    // 是否必填
    #[serde(default)] // 如果字段不存在，使用默认值
    custom_validation: Option<Vec<CustomValidationRule>>, // 自定义验证规则数组
}

// 定义自定义验证规则的结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomValidationRule {
    title: String, // 验证规则的描述
    alias: String, // 验证规则的别名（用于生成列名）
    rule: String,  // 验证规则
    #[serde(default)]
    is_ok: u8, // 验证结果，1 表示通过，0 表示失败
}

// 定义验证结果的结构体
#[derive(Debug, Serialize, Clone)]
pub struct ValidationResult {
    pub data: HashMap<String, String>, // 解析的数据
    is_valid: bool,                    // 是否通过验证
    errors: Vec<String>,               // 错误信息
    pub custom_validation_results: Vec<CustomValidationRule>, // 自定义验证结果
}

// 定义验证规则的结构体
#[derive(Debug, Serialize, Deserialize)]
struct ValidationRule {
    name: String, // 验证规则的名称
    rule: String, // 验证规则的代码
}

// 定义验证规则集合的结构体
#[derive(Debug, Serialize, Deserialize)]
struct ValidationRules {
    validations: Vec<ValidationRule>, // 验证规则列表
}
use rayon::ThreadPoolBuilder; // 引入 ThreadPoolBuilder

// 定义文件处理器结构体
pub struct FileProcessor;

impl FileProcessor {
    // 解析数据的通用函数
    fn parse_data_with_rules(
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
        rules: &[HeaderRule],
        validation_rules: &ValidationRules,
    ) -> Result<Vec<ValidationResult>, Box<dyn Error>> {
        // 验证表头是否匹配规则
        for rule in rules {
            if !headers.contains(&rule.name) {
                let x = format!("表头不匹配: 未找到预期的表头 '{}'", rule.name).into();
                error!("{}", x);
                return Err(x);
            }
        }

        // 创建自定义线程池并设置最大线程数为 10//todo
        let pool = ThreadPoolBuilder::new().num_threads(4).build()?;
        let results: Vec<ValidationResult> = pool.install(|| {
            rows.par_iter().map(|row| {
                let thread_id = std::thread::current().id();
                println!("Processing row on thread: {:?}", thread_id);
                let mut row_data = HashMap::new();
                let mut is_valid = true;
                let mut errors = Vec::new();
                let mut custom_validation_results = Vec::new();

                for (i, cell) in row.iter().enumerate() {
                    if i >= headers.len() {
                        break;
                    }
                    let header_name = headers[i].clone();
                    if let Some(rule) = rules.iter().find(|r| r.name == header_name) {
                        let value = cell.to_string();

                        // 检查必填字段
                        if rule.required && value.is_empty() {
                            is_valid = false;
                            errors.push(format!("必填字段为空: {}", rule.name));
                        }

                        let new_column_name = format!("{}", rule.alias);
                        row_data.insert(new_column_name, value.clone());
                        // 自定义验证
                        if let Some(validations) = &rule.custom_validation {
                            for validation in validations.iter() {
                                let is_ok = Self::eval_custom_validation(
                                    &value,
                                    &validation.rule,
                                    validation_rules,
                                ).unwrap_or_else(|e| {
                                    error!("自定义验证出错: {}", e);
                                    false
                                });
                                let mut validation_result = validation.clone(); // 克隆验证规则
                                validation_result.is_ok = if is_ok { 1 } else { 0 }; // 更新验证结果
                                custom_validation_results.push(validation_result); // 存储验证结果

                                // 动态增加列
                                let new_column_name = format!("{}", validation.alias);
                                let new_column_value = if is_ok { 1 } else { 0 };
                                row_data.insert(new_column_name, new_column_value.to_string());
                            }
                        }
                    }
                }
                ValidationResult {
                    data: row_data,
                    is_valid,
                    errors,
                    custom_validation_results,
                }
            }).collect()
        });
        Ok(results)
    }

    // 解析 Excel 数据
    fn parse_excel_with_rules(
        path: &str,
        rules: &[HeaderRule],
        validation_rules: &ValidationRules,
    ) -> Result<Vec<ValidationResult>, Box<dyn Error>> {
        // 打开 Excel 文件
        let mut workbook: Xlsx<_> = open_workbook(path)?;

        // 读取第一个工作表
        let range = workbook.worksheet_range("Sheet1")?;

        // 获取表头行
        let headers: Vec<String> = range
            .rows()
            .next()
            .unwrap()
            .iter()
            .map(|h| h.to_string())
            .collect();

        // 获取数据行
        let rows: Vec<Vec<String>> = range
            .rows()
            .skip(1)
            .map(|row| row.iter().map(|cell| cell.to_string()).collect())
            .collect();
        // 调用通用解析函数
        Self::parse_data_with_rules(headers, rows, rules, validation_rules)
    }

    // 解析 CSV 数据
    fn parse_csv_with_rules(
        path: &str,
        rules: &[HeaderRule],
        validation_rules: &ValidationRules,
    ) -> Result<Vec<ValidationResult>, Box<dyn Error>> {
        // 打开 CSV 文件
        let mut rdr = ReaderBuilder::new().from_path(path)?;

        // 获取表头行
        let headers = rdr
            .headers()?
            .iter()
            .map(|h| h.to_string())
            .collect::<Vec<String>>();

        // 获取数据行
        let rows: Vec<Vec<String>> = rdr
            .records()
            .map(|record| {
                record
                    .unwrap()
                    .iter()
                    .map(|cell| cell.to_string())
                    .collect()
            })
            .collect();
        info!("path:{} rows length:{:#?}", path, &rows.len());
        // 调用通用解析函数
        Self::parse_data_with_rules(headers, rows, rules, validation_rules)
    }

    // 执行自定义验证
    fn eval_custom_validation(
        value: &str,
        validation: &str,
        validation_rules: &ValidationRules,
    ) -> Result<bool, Box<dyn Error>> {
        // 查找匹配的验证规则
        if let Some(rule) = validation_rules
            .validations
            .iter()
            .find(|r| r.rule == validation)
        {
            // 使用 mlua 执行 Lua 代码
            let lua = Lua::new();
            let lua_code = format!("value = '{}'; return {}", value, rule.rule);
            let result: bool = lua.load(&lua_code).eval()?;
            Ok(result)
        } else {
            Err(format!("未找到验证规则: {}", validation).into())
        }
    }

    // 读取 YAML 配置文件
    fn read_yaml_config(path: &str) -> Result<Vec<HeaderRule>, Box<dyn Error>> {
        let yaml_content = std::fs::read_to_string(path)?;
        let rules: Vec<HeaderRule> = serde_yaml::from_str(&yaml_content)?;
        Ok(rules)
    }

    // 读取验证规则文件
    fn read_validation_rules(path: &str) -> Result<ValidationRules, Box<dyn Error>> {
        let yaml_content = std::fs::read_to_string(path)?;
        let rules: ValidationRules = serde_yaml::from_str(&yaml_content)?;
        Ok(rules)
    }

    // 根据文件名动态加载配置文件
    fn get_config_path(file_name: &str, config_dir: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
        let file_name_lower = file_name.to_lowercase();

        let config_name = if file_name_lower.contains("query") && file_name_lower.contains("detail")
        {
            "query_detail_config.yaml"
        } else if file_name_lower.contains("cleaning") && file_name_lower.contains("data") {
            "cleaning_data_config.yaml"
        } else if file_name_lower.contains("missing") && file_name_lower.contains("page") {
            "missing_page_config.yaml"
        } else if file_name_lower.contains("import_site") {
            "import_site_config.yaml"
        } else {
            error!("未找到匹配的配置文件: {}", file_name);
            return Err(format!("未找到匹配的配置文件: {}", file_name).into());
        };

        let config_path = config_dir.join(config_name);
        if !config_path.exists() {
            error!("配置文件不存在: {}", config_path.display());
            return Err(format!("配置文件不存在: {}", config_path.display()).into());
        }

        Ok(config_path)
    }

    // 扫描目录并处理文件
    // 处理单个文件并返回结果
    pub fn process_file<F>(app_handle: AppHandle,file_path: String, callback: F) -> Result<(), Box<dyn Error>>
    where
        F: Fn(Vec<ValidationResult>, &str) -> (),
    {
        let file_path = PathBuf::from(file_path);
        let file_name = file_path.file_stem().unwrap().to_str().unwrap();
        let validation_path = CONFIG_DIR.join(format!("{}.yaml", "support_validation"));

        let config_path = Self::get_config_path(file_name, &*CONFIG_DIR)
            .map_err(|e| {
                format!("处理文件 {} 时出错: {}", file_path.display(), e)
            })?;
        // 读取 YAML 配置文件
        let rules = Self::read_yaml_config(config_path.to_str().unwrap())?;

        let validation_rules = Self::read_validation_rules(validation_path.to_str().unwrap())
            .map_err(|e| format!("读取验证规则文件时出错: {}", e))?;
        info!("====>开始读取文件:{}", file_name);

        let results = match file_path.extension().and_then(|s| s.to_str()) {
            Some("csv") => {
                Self::parse_csv_with_rules(file_path.to_str().unwrap(), &rules, &validation_rules)?
            }
            Some("xlsx") | Some("xls") => Self::parse_excel_with_rules(
                file_path.to_str().unwrap(),
                &rules,
                &validation_rules,
            )?,
            _ => return Err("不支持的文件格式".into()),
        };

        info!("====>读取文件完成");
        callback(results, file_name);
        info!("====>数据处理完成");
        Ok(())
    }
}
