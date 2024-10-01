use crate::ocr::create_modified_path_with;
use std::{fs, path};
use std::io::{self,ErrorKind::InvalidInput};

pub struct Config {
    pub images_path: path::PathBuf,
    pub output_path: path::PathBuf,
    pub parent_path: path::PathBuf,
    pub total:u32,
    pub valid:u32,
    pub duplicates:u32,
    pub unable_to_recognize:u32
}
impl Config<> {
    pub fn new<F>(input_path: &str, modify_fy: F) -> io::Result<Config>
    where
        F: Fn(&str) -> String,
    {
        match fs::canonicalize(input_path) {
            Ok(images_path) => {
                Ok(
                    Config {
                        images_path: images_path.clone(),
                        output_path: create_modified_path_with(path::Path::new(&input_path), modify_fy)?,
                        parent_path: images_path.parent().unwrap().to_path_buf(),
                        total:0,
                        valid:0,
                        duplicates:0,
                        unable_to_recognize:0
                    })
            }
            Err(_) => Err(io::Error::new(InvalidInput,"路径错误")),
        }
    }
}