use serde::Serialize;

#[derive(Serialize)]
pub struct SupportedTemplateResponse {
    pub(crate) valid: bool,
    pub(crate) templates: Vec<String>,
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
