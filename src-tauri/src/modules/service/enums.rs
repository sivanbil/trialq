#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Ongoing, // 任务进行中
    Done,    // 任务完成
    Error,   // 任务出错
}

impl TaskStatus {
    // 判断任务是否正在进行中
    pub fn is_ongoing(&self) -> bool {
        matches!(self, TaskStatus::Ongoing)
    }

    // 判断任务是否已完成
    pub fn is_done(&self) -> bool {
        matches!(self, TaskStatus::Done)
    }

    // 判断任务是否出错
    pub fn is_error(&self) -> bool {
        matches!(self, TaskStatus::Error)
    }
}

use std::str::FromStr; // 导入 FromStr trait
use strum::IntoEnumIterator; // 导入 IntoEnumIterator trait
use strum_macros::{AsRefStr, Display, EnumIter, EnumString}; // 导入宏

#[derive(Debug, Clone, PartialEq, EnumIter, EnumString, AsRefStr, Display)]
pub enum SupportedTemplate {
    Medidata,
    // 可以在此添加更多模板
}

impl SupportedTemplate {
    // 检查是否为支持的模板
    pub fn is_supported(template_name: &str) -> bool {
        // 使用 FromStr 的 from_str 方法
        SupportedTemplate::from_str(template_name).is_ok()
    }

    // 获取所有支持的模板名称
    pub fn all_supported_templates() -> Vec<String> {
        SupportedTemplate::iter()
            .map(|t| t.as_ref().to_string())
            .collect()
    }

    // 获取所有支持的模板枚举值
    pub fn all_supported_templates_enum() -> Vec<SupportedTemplate> {
        // 使用 IntoEnumIterator 的 iter 方法
        SupportedTemplate::iter().collect()
    }
}
