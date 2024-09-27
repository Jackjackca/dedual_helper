use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::process::Command;
use std::path::Path;

pub fn get_text_from_img(img_name: &str) -> Option<String> {
    let output = Command::new("ocrs")
        .arg(img_name)
        .output()
        .expect("failed to execute process");
    match output.status.success() {
        true => Some(String::from_utf8_lossy(&output.stdout).to_string()),
        false => None,
    }
}

pub fn find_eight_digit_number(input: &str) -> Option<String> {
    // 创建一个匹配8位数字的正则表达式
    let re = Regex::new("NO:(.{10})").unwrap();

    // 使用正则表达式查找匹配项
    if let Some(cap) = re.captures(input) {
        // 如果找到匹配项，则返回匹配的字符串
        Some(cap[1].to_string())
    } else {
        // 如果没有匹配项，返回 None
        None
    }
}

pub fn print_result(total_files_count: usize, result_map: &HashMap<String, Vec<String>>) {
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



/// 根据原始路径和修改函数创建一个新路径，并创建该新目录。
///
/// # 参数
/// - `original_path`: 原始的路径，例如 "a/b/c"
/// - `modify`: 一个闭包，定义如何修改最后一个组件
///
/// # 返回
/// - `Ok(new_path)`: 成功时返回新创建的路径
/// - `Err(io::Error)`: 复制过程中发生的错误
pub fn create_modified_path_with<F>(original_path: &Path, modify: F) -> io::Result<std::path::PathBuf>
where
    F: Fn(&str) -> String,
{
    // 确保原始路径有父目录
    let parent = match original_path.parent() {
        Some(p) => p,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "原始路径没有父目录",
            ))
        }
    };

    // 获取最后一个组件
    let last_component = match original_path.file_name() {
        Some(name) => name,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "无法获取原始路径的最后一个组件",
            ))
        }
    };

    // 将最后一个组件转换为 &str
    let last_component_str = match last_component.to_str() {
        Some(s) => s,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "无法将最后一个组件转换为字符串",
            ))
        }
    };

    // 使用闭包修改最后一个组件
    let modified_component = modify(last_component_str);

    // 构建新的路径
    let new_path = parent.join(&modified_component);

    // 创建新的目录
    fs::create_dir_all(&new_path)?;

    Ok(new_path)
}

