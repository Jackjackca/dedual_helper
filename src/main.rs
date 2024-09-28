use dedual_helper::ocr;
use dedual_helper::ocr::create_modified_path_with;
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::vec::Vec;
use std::{env, fs};
use dedual_helper::config::Config;
#[allow(unused)]
fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    // let config = Config::new(args[1].as_str(),args[2].as_str());

    // 定义修改闭包，描述已筛选目录
    let modify_fn = |s: &str| format!("{} - 已筛选", s);

    //描述相关路径
    let binding = args[1].to_string();
    let original_parent_path = Path::new(&binding);
    let image_paths_string = ocr::get_image_paths(&args[1])?;
    let mut new_parent_path=create_modified_path_with(&original_parent_path,modify_fn)?;
    println!("{}", original_parent_path.display());


    // 创建
    match create_modified_path_with(&original_parent_path, modify_fn) {
        Ok(new_path) => {
            // 创建新的目录
            fs::create_dir_all(&new_path)?;
            println!("成功创建新目录: {}", new_path.display());
        }
        Err(e) => { panic!("无法创建目录"); }
    }

    //图片总数
    let total_images_count = image_paths_string.len();

    //进度指示器
    let pb = ProgressBar::new(image_paths_string.len() as u64);

    let images_dir = &original_parent_path; // 替换为你的图片目录
    let image_paths = ocr::get_image_paths(images_dir.to_str().unwrap())?;

    if image_paths.is_empty() { println!("给定目录没有图片。") }

    let mut result_map: HashMap<String, Vec<&str>> = HashMap::new();

    image_paths.iter().for_each(|image_path| {
        pb.inc(1);
        //存储结果
        let image_path = Path::new(image_path);
        let image_path_str = image_path.file_name().unwrap().to_str().unwrap();
        let output = Command::new("ocrs")
            .arg(image_path)
            .output();
        match output {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stdout = stdout.replace("\n","").replace("\r","").replace(" ", "");
                    println!("{}: \"{}\"", image_path.display(), stdout);
                    match ocr::get_number_from_string(&stdout) {
                        Some(number) => {
                            //序号重复，vec推入图片路径数组；序号第一次出现，新增num，vec键值
                            &mut result_map.entry(number.clone()).and_modify(|vec| {
                                vec.push(&image_path_str);
                                println!("编号{}重复了{}次，第一次在\"{}中\"", &number, &vec.len(), &image_path.display());
                            }).or_insert_with(|| {
                                println!("有效编号：{}", number);
                                vec![image_path.to_str().unwrap()]
                            });
                        }
                        None => println!("未找到序号")
                    }
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("ocrs报错。 {}: \"{}\"", image_path.display(), stderr);
                }
            }
            Err(e) => {
                eprintln!("无法执行ocrs命令。它在环境变量中吗？ {}: {}", image_path.display(), e);
            }
        }
    });
    for value in result_map.values() {
        let image_name = Path::new(&value[0]).file_name().unwrap().to_str().unwrap();
        // println!("from:`{}`to'{}'",&value[0],&new_parent_path.join(image_name).display());
        fs::copy(&value[0], &new_parent_path.join(image_name));
    }
    pb.finish_with_message("完成。");
    ocr::print_result(total_images_count, &result_map);
    Ok(())
}

