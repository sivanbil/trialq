use chrono::{NaiveDate, ParseError};
use log::info;

pub fn parse_date(date_str: &str) -> Result<NaiveDate, ParseError> {
    // 尝试使用 "%Y/%m/%d" 格式解析
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y/%m/%d") {
        return Ok(date);
    }

    // 尝试使用 "%d-%b-%y" 格式解析
    NaiveDate::parse_from_str(date_str, "%d-%b-%y")
}

pub fn excel_date_to_naive_date(days: i32) -> NaiveDate {
    // Excel 日期从 1900 年 1 月 1 日开始，不过 Excel 错误地认为 1900 年是闰年
    let start_date = NaiveDate::from_ymd_opt(1900, 1, 1).unwrap();
    // 减去 2 是因为 Excel 错误地将 1900 年 2 月 29 日算进去了
    start_date + chrono::Days::new((days - 2) as u64)
}

use tauri::{AppHandle, Emitter};

// 通用的处理函数，接收 AppHandle、事件基础名称、URL 和当前进度
pub fn handle_progress_events(app: AppHandle, event_base_name: &str, url: &str, progress: u8) {
    // 触发开始事件（仅在进度为 0 时触发）
    info!(">>>>>>>>>>>>>>>>>>>>>>event_base_name:{}", event_base_name);
    if progress == 0 {
        let message = format!("{}-started", url);
        app.emit(&event_base_name, message).unwrap();
    }

    // 触发进度事件
    let message = format!("{}-progress. {}%", url, progress);
    info!("handle_progress_events:{}", message);
    app.emit(&event_base_name, message).unwrap();

    // 触发完成事件（仅在进度为 100 时触发）
    if progress == 100 {
        let message = format!("{}-finished", url);
        app.emit(&event_base_name, message).unwrap();
    }
}

pub fn calculate_percentage_u8(part: u8, whole: u8) -> u8 {
    if whole == 0 {
        0
    } else {
        // 将 u8 转换为 f64 进行计算
        // 将 u8 转换为 f64 进行计算
        let percentage = (part as f64 / whole as f64) * 100.0;
        percentage as u8
    }
}

use rust_xlsxwriter::{Format, Workbook, XlsxError, FormatAlign};
use tauri_plugin_dialog::DialogExt;
use std::fs;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::env;
use serde_json::Value;
use crate::modules::service::projects::report_service::OriginExcelDataResponse;

pub fn export_excel(
    app: AppHandle,
    header_order: Vec<String>,
    table_header_map: HashMap<String, String>,
    data: Vec<OriginExcelDataResponse>,
    file_name: String,
) -> Result<(), String> {
    // 创建 Excel 工作簿
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // 创建格式对象
    let bold_format = Format::new().set_bold();
    let center_format = Format::new().set_align(FormatAlign::Left);

    // 动态写入表头并应用加粗格式
    let mut col_index = 0;
    for header in header_order.clone() {
        let value = table_header_map.get(&header).unwrap();
        worksheet.set_column_width(col_index, 40).expect("set_column_width failed");
        worksheet.write_string_with_format(0, col_index, value, &bold_format)
            .map_err(|e| e.to_string())?;
        col_index += 1;
    }

    let mut row_index = 1;
    for response in data {
        for row in response.data {
            if let Value::Object(row_obj) = row {
                for (col_index, key) in header_order.iter().enumerate() {
                    if let Some(cell_value) = row_obj.get(key) {
                        let cell_str = match cell_value {
                            Value::String(s) => s.clone(),
                            _ => cell_value.to_string(),
                        };
                        worksheet.write_string_with_format(row_index as u32, col_index as u16, &cell_str, &center_format)
                            .map_err(|e| e.to_string())?;
                    }
                }
                row_index += 1;
            }
        }
    }

    // 将工作簿保存到临时文件
    let temp_dir = env::temp_dir();
    let temp_path = temp_dir.join("temp_excel.xlsx");
    workbook.save(&temp_path)
        .map_err(|e| e.to_string())?;

    // 弹出保存文件对话框
    let file_path = app
        .dialog()
        .file()
        .set_file_name(&file_name) // 设置默认文件名
        .add_filter("Excel Files", &["xlsx"]) // 添加文件类型过滤器
        .blocking_save_file()
        .ok_or("用户取消了保存操作")?;
    // 将 `FilePath` 转换为 `PathBuf`
    let save_path: PathBuf = file_path.into_path().unwrap();

    // 尝试移动文件，如果失败则复制并删除临时文件
    if let Err(_) = fs::rename(&temp_path, &save_path) {
        fs::copy(&temp_path, &save_path)
            .map_err(|e| e.to_string())?;
        fs::remove_file(&temp_path)
            .map_err(|e| e.to_string())?;
    }

    info!("文件已保存到: {:?}", save_path);
    Ok(())
}
