// use std::env;
use std::fs::File;
// use std::io::prelude::*;

use std::io::Read;

pub fn read(path: String) -> Result<String, String> {
  let mut f = File::open(&path)
    .or(Err(String::from(format!("can't open file (path: {})", &path))))?;

  let mut contents = String::new();

  f.read_to_string(&mut contents)
    .or(Err(format!("can't open file (path: {})", &path)))?;

  Ok(contents)
}

pub fn write(_path: String) -> Result<(), String> {
  Ok(())
}