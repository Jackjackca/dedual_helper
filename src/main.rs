use dedual_helper::ocr;
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::vec::Vec;
use std::fs;
use dedual_helper::ocr::create_modified_path_with;

#[allow(unused)]
fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    // 定义修改闭包，描述已筛选目录
    let modify_fn = |s: &str| format!("{} - 已筛选", s);

    //图片绝对路径
    let image_paths = ocr::get_image_paths(&args[1])?;
    let old_path =Path::new(&args[2]);
    let new_path = Path::new(&image_paths[0]);

    //描述相关路径
    let parent_path = Path::new(&image_paths[0]).parent().unwrap();
    if let Some(new_path) = create_modified_path_with(&parent_path, modify_fn) {
        let mut new_path = new_path;
    }

    // 创建
    match create_modified_path_with(&parent_path, modify_fn) {
        Ok(new_path) => {
            // 创建新的目录
            println!("成功创建新目录: {:?}", new_path)
        }
        Err(e) => { eprintln!("无法创建目录") }
    }

    //图片总数
    let total_images_count = image_paths.len();

    //进度指示器
    let pb = ProgressBar::new(image_paths.len() as u64);

    let images_dir = &parent_path; // 替换为你的图片目录
    let image_paths = ocr::get_image_paths(images_dir.to_str().unwrap())?;

    if image_paths.is_empty() { println!("给定目录没有图片。") }

    let mut result_map: HashMap<String, Vec<String>> = HashMap::new();

    image_paths.iter().for_each(|image_path| {
        //存储结果
        let image_path = Path::new(image_path);
        let output = Command::new("ocrs")
            .arg(image_path)
            .output();
        match output {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stdout = stdout.trim().replace("\n", "").replace("\r", "");
                    match ocr::get_number_from_string(&stdout) {
                        Some(number) => {
                            //序号重复，vec推入图片路径数组；序号第一次出现，新增num，vec键值
                            &mut result_map.entry(number.clone()).and_modify(|vec| {
                                vec.push(image_path.to_string());
                                fs::copy(&image_path, &new_path);
                                println!("编号{}重复了{}次，第一次在\"{}中\"", &number, &vec.len(), &image_path);
                            }).or_insert_with(|| {
                                println!("有效编号：{}", number);
                                vec![image_path.to_string()]
                            });
                        }
                        None => println!("未找到序号")
                    }
                    println!("{}: \"{}\"", image_path, stdout);
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("ocrs报错。 {}: \"{}\"", image_path, stderr);
                }
            }
            Err(e) => {
                eprintln!("无法执行ocrs命令。它在环境变量中吗？ {}: {}", image_path, e);
            }
        }
    });
    ocr::print_result(total_images_count, &result_map);
    let image_to_copy = ocr::image_path_with_unique_number(&result_map);

    Ok(())
}

