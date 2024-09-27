use std::fs::File;
use std::io::Write;

pub fn write_string_to_file(file_name: &str, content: String) -> Option<usize> {
   match File::create(file_name){
      Ok(mut file) => {
         file.write(content.as_bytes()).ok()
      }
      Err(_) => None
   }
}