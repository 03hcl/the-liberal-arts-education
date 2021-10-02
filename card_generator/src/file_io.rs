// use std::env;
use std::fs::File;
use std::io::prelude::*;

use std::io::Read;

pub fn read(path: String) -> Result<String, String> {
  let mut file = File::open(&path)
    .or(Err(String::from(format!("can't open file (path: {})", &path))))?;

  let mut content = String::new();

  file.read_to_string(&mut content)
    .or(Err(format!("can't open file (path: {})", &path)))?;

  Ok(content)
}

pub fn write(content: String, path: String) -> Result<(), String> {
  let mut file = File::create(&path)
    .or(Err(String::from(format!("can't create file (path: {})", &path))))?;
  file.write_all(content.as_bytes())
    .or(Err(String::from(format!("can't write file content (path: {})", &path))))
}