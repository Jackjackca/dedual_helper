use dedual_helper::articles::*;
use dedual_helper::ocr::find_eight_digit_number;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::f32::consts::E;
use std::ops::Add;
use std::time::{Duration, Instant};
use std::vec::Vec;
use std::{fs, thread};

#[allow(unused)]
fn main() {
    println!("{}", 183 % 301);
    fs::copy(r"C:\Users\lenovo\Desktop\反邪教去重\大孔 - 副本\微信图片_202409181432481.jpg", r"C:\Users\lenovo\Desktop\反邪教去重\大孔 - 副本 - 已筛选");
    // vec_push_bench();
    // match dedual_helper::files::write_string_to_file("\\","hello, world".to_string()) {
    //     Some(num_write_bytes)=>println!("Wrote {} bytes", num_write_bytes),
    //     _=>println!("No file written"),
    // }

    let args: Vec<String> = std::env::args().collect();
    println!("args: {:?}", args);

    //获取目录参数（第一个）
    let original_path = std::path::Path::new(&args[1]);
    //获取所有条目
    let entries = fs::read_dir(original_path).expect("cannot read directory.");
    //创建一个存储文件名的数组
    let mut file_names: Vec<String> = Vec::new();
    for entry in entries {
        let entry = entry.unwrap(); //解包条目，处理错误
        let file_name = entry.file_name().to_string_lossy().to_string(); //转换为字符串
        file_names.push(file_name);
    }

    let total_files_count = file_names.len();
    // println!("file_names: {:?}", file_names);

    //进度指示器
    let pb = ProgressBar::new(file_names.len() as u64);

    // 定义修改函数：将 "c" 变为 "cc"
    let modify_fn = |s: &str| format!("{} - 已筛选", s);

    // 调用函数创建新路径
    match dedual_helper::ocr::create_modified_path_with(original_path, modify_fn) {
        Ok(new_path) => println!("成功创建新目录: {:?}", new_path),
        Err(e) => eprintln!("创建新目录时出错: {}", e),
    }
    //存储结果
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for file_name in file_names {
        pb.inc(1);
        match dedual_helper::ocr::get_text_from_img(
            &original_path
                .to_string_lossy()
                .to_string()
                .add(file_name.as_str()),
        ) {
            Some(result) => {
                let result = result.trim().replace(" ", "").replace("\n", "");
                println!("{}", result);
                //应用找8位数的正则
                match find_eight_digit_number(result.as_str()) {
                    Some(number) => {
                        if let 10 = number.len() {
                            let number = &number[number.len() - 8..];
                            //检查重复，有则插入文件名，无责插入新项
                            match map.get_mut(number) {
                                Some(duplicate_files) => {
                                    let _file_name = file_name.clone();
                                    duplicate_files.push(file_name);
                                    println!(
                                        "编号{}重复了{}次，第一次在{}。",
                                        number,
                                        duplicate_files.len(),
                                        _file_name
                                    )
                                }
                                None => {
                                    map.insert(number.to_string(), vec![file_name]);
                                    println!("编号{}第一次出现。", number);
                                }
                            }
                        }
                    }
                    None => {
                        println!("ocr number not found.")
                    }
                }
            }
            None => {
                println!("ocr failed. Maybe caused by file not found.");
            }
        }
    }
    dedual_helper::ocr::print_result(total_files_count, &map);
    for (number, files) in map.iter() {
        let file = original_path.join(files[0].clone());
        match dedual_helper::ocr::create_modified_path_with(original_path, modify_fn) {
            Ok(new_path) => {
                let new_path = new_path.join(files[0].clone());
                println!("file: {},new_path:{}",file.as_os_str().to_str().expect("msg"),new_path.as_os_str().to_str().expect("msg"));
                fs::copy(file, new_path);
            }
            Err(e) => eprint!("{}", e),
        }
    }
}

