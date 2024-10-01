use crate::config::Config;
use rust_xlsxwriter::{Workbook, XlsxError};
use std::collections::HashMap;
use std::fs;

pub fn copy_images(config: &Config, result_map: &HashMap<String, Vec<String>>) -> std::io::Result<()> {
    for image_names in result_map.values() {
        let image_name = &image_names[0];
        fs::copy(config.images_path.join(image_name), config.output_path.join(image_name))?;
    }
    Ok(())
}
pub fn save_result_to_excel(config: &Config) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_name(&*config.images_path.file_name().unwrap().to_string_lossy())?;
    worksheet.write(0, 0, "已处理")?;
    worksheet.write(1, 0, "有效")?;
    worksheet.write(2, 0, "重复")?;
    worksheet.write(3, 0, "重复率")?;
    worksheet.write(0, 1, config.total)?;
    worksheet.write(1, 1, config.valid)?;
    worksheet.write(2, 1, config.duplicates)?;
    worksheet.write(3, 1, config.duplicates / config.valid)?;
    workbook.save(&config.parent_path.join("结果.xlsx")).expect("panic");
    Ok(())
}

pub fn print_result(config: &Config) -> Result<(), std::io::Error> {
    println!("已处理{}张图片，有效图片{}张，重复图片{}张，无法识别{}张。", &config.total, &config.valid, &config.duplicates, &config.unable_to_recognize);
    Ok(())
}