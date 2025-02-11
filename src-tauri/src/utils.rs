use chrono::{NaiveDate, ParseError};

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
