use dedu_helper::config::Config;
use dedu_helper::ocr;
use dedu_helper::result;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::collections::HashMap;
use std::io::ErrorKind::InvalidInput;
use std::process::Command;
use std::vec::Vec;
use std::{env, fs};

#[allow(unused)]
fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    // 定义修改闭包，描述已筛选目录
    let modify_fn = |s: &str| format!("{} - 已筛选", s);
    if args.len() < 2 {
        eprintln!("参数错误：用法：dedual_helper <图片所在路径>");
        return Err(std::io::Error::new(InvalidInput, "参数错误"));
    }
    println!("{}", args[1]);
    //描述相关路径
    let mut config = Config::new(args[1].as_str(), modify_fn)?;
    let images_path = &config.images_path;
    let output_path = &config.output_path;
    let parent_path = &config.parent_path;

    let binding = args[1].to_string();
    println!("{}", images_path.display());


    // 创建
    fs::create_dir_all(output_path)?;
    println!("成功创建新目录: {}", output_path.display());

    //图片总数
    let total_images_count = ocr::get_image_paths(&args[1])?.len();
    config.total = total_images_count as u32;

    let images_dir = &images_path; // 替换为你的图片目录
    let image_paths = ocr::get_image_paths(images_dir.to_str().unwrap())?;

    if image_paths.is_empty() { return Err(std::io::Error::new(InvalidInput, "给定目录没有图片。")); }

    let mut result_map: HashMap<String, Vec<String>> = HashMap::new();

    //进度指示器
    let pb = ProgressBar::new(image_paths.len() as u64);
    pb.set_style(ProgressStyle::default_bar().template("[{elapsed_precise}]{spinner:1.blue}[{bar:40}]{pos}/{len} ETA:{eta}").unwrap().progress_chars("=>-"));
    image_paths.iter().for_each(|image_path| {
        pb.inc(1);
        let image_name_str = image_path.file_name().unwrap().to_str().unwrap();
        let output = Command::new("ocrs")
            .arg(image_path)
            .output();
        match output {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stdout = stdout.replace("\n", "").replace("\r", "").replace(" ", "");
                    println!("{}\n{}", &image_path.display(), &stdout);
                    match ocr::get_number_from_string(&stdout) {
                        Some(number) => {
                            //序号重复，vec推入图片路径数组；序号第一次出现，新增num，vec键值
                            &mut result_map.entry(number.clone()).and_modify(|vec| {
                                config.duplicates += 1;
                                vec.push(String::from(image_name_str));
                                println!("编号{}重复了{}次，第一次在\"{}\"中", &number, &vec.len(), &image_path.display());
                            }).or_insert_with(|| {
                                config.valid += 1;
                                println!("有效编号：{}", &number);
                                vec![String::from(image_name_str)]
                            });
                        }
                        None => {
                            config.unable_to_recognize+=1;
                            println!("未找到序号")
                        }
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
    pb.finish_with_message("识别完成。");
    result::print_result(&config);
    result::copy_images(&config,&result_map);
    result::save_result_to_excel(&config);
    Ok(())
}