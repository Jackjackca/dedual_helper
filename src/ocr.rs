use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Error;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn get_text_from_image(image_path: &Path) -> Result<String, io::Error> {
    let output = Command::new("ocrs")
        .arg(image_path)
        .output()
        .expect("failed to execute process");
    match output.status.success() {
        true => Ok(String::from_utf8_lossy(&output.stdout).to_string()),
        false => Err(io::Error::new(io::ErrorKind::Other, String::from_utf8_lossy(&output.stderr).to_string())),
    }
}

pub fn get_number_from_string(input: &str) -> Option<String> {
    // 创建一个匹配8位数字的正则表达式
    let re = Regex::new(r"NO:(.{10})").unwrap();

    // 使用正则表达式查找匹配项
    if let Some(cap) = re.captures(input) {
        // 如果找到匹配项，则返回匹配的字符串
        let number = cap.get(1)?;
        if let 10 = number.len() {
            Some(number.as_str()[2..].to_string())
        } else { None }
    } else {
        // 如果没有匹配项，返回 None
        None
    }
}

pub fn print_result(total_files_count: usize, result_map: &HashMap<String, Vec<&str>>) {
    let valid_count = result_map.len();
    let mut duplication: usize = 0;
    for (_string, vec) in result_map.iter() {
        if vec.len() > 1 {
            let i = vec.len() - 1;
            duplication += i;
        };
    }
    println!(
        "已处理{}张图片，有效图片{}张，重复图片{}张。",
        total_files_count,
        valid_count,
        duplication
    )
}


/// 根据原始路径和修改函数创建一个新路径。
///
/// # 参数
/// - `original_path`: 原始的路径，例如 "a/b/c"
/// - `modify`: 一个闭包，定义如何修改最后一个组件
///
/// # 返回
/// - `Ok(new_path)`: 成功时返回新创建的路径
/// - `Err(io::Error)`: 复制过程中发生的错误
pub fn create_modified_path_with<F>(original_path: &Path, modify: F) -> io::Result<PathBuf>
where
    F: Fn(&str) -> String,
{
    // 确保原始路径有父目录
    let parent = match original_path.parent() {
        Some(parent) => parent,
        None => return Err(Error::new(io::ErrorKind::InvalidInput, ""))
    };

    // 获取最后一个组件
    let last_component = match original_path.file_name() {
        Some(file_name) => file_name,
        None => return Err(Error::new(io::ErrorKind::InvalidInput, ""))
    };

    // 将最后一个组件转换为 &str
    let last_component_str = match last_component.to_str() {
        Some(last_component_str) => last_component_str,
        None => return Err(Error::new(io::ErrorKind::InvalidInput, ""))
    };

    // 使用闭包修改最后一个组件
    let modified_component = modify(last_component_str);

    // 构建新的路径
    let new_path = parent.join(&modified_component);

    Ok(new_path)
}
pub fn get_image_paths(dir: &str) -> Result<Vec<PathBuf>, Error> {
    let mut paths = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext.eq_ignore_ascii_case("png") ||
                    ext.eq_ignore_ascii_case("jpg") ||
                    ext.eq_ignore_ascii_case("jpeg") {
                    paths.push(path);
                }
            }
        }
    }
    Ok(paths)
}

pub fn image_paths_with_unique_number(result_map: &HashMap<String, Vec<&str>>) -> Option<Vec<String>> {
    Some(result_map.values().map(|v| v[0].to_string()).collect())
}