use std::fs;
use std::path::{Path, PathBuf};
// 递归扫描目录
fn scan_dir_recursive(path: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    if path.is_dir() {
        for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                // 如果是目录，递归扫描
                scan_dir_recursive(&entry_path, files)?;
            } else if entry_path.is_file() {
                // 如果是文件，检查扩展名
                if let Some(ext) = entry_path.extension() {
                    if ext == "xlsx" || ext == "xls" || ext == "csv" {
                        files.push(entry_path);
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn scan_directory(path: String) -> Result<Vec<String>, String> {
    let mut excel_files = Vec::new();
    let path = Path::new(&path);

    // 调用递归扫描函数
    scan_dir_recursive(path, &mut excel_files)?;

    // 将 PathBuf 转换为字符串
    let files: Vec<String> = excel_files
        .into_iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();

    Ok(files)
}