use std::path;

pub struct Config{
    images_path:path::PathBuf,
    output_path:path::PathBuf,
}

impl Config{
    pub fn new(images_path_str:&str,output_path_str:&str) -> Config{
        Config{
            images_path:path::Path::new(images_path_str).to_path_buf(),
            output_path:path::Path::new(output_path_str).to_path_buf(),
        }
    }
}