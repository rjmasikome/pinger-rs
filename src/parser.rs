extern crate serde_yaml;
use serde_yaml::Value;
use std::io::prelude::*;

fn read_file(filename: &str) -> Option<String> {
  let mut file_handle = std::fs::File::open(filename).expect("file not found");
  let mut content = String::new();
  match file_handle.read_to_string(&mut content) {
    Ok(_) => Some(content),
    Err(_) => None,
  }
}

pub fn parse_yaml(yaml: String) -> Option<Value> {
  let c_str: &str = &yaml;
  let parsed = serde_yaml::from_str(c_str).unwrap();
  match parsed {
    Some(value) => Some(value),
    None => None,
  }
}

pub mod config {

  use serde_yaml::Value;

  pub fn get_config(opt_filename: Option<&String>) -> Option<Value> {
    let default_filename = "./config/default.yaml".to_string();
    let filename = opt_filename.unwrap_or(&default_filename);

    let content: String = super::read_file(&filename).expect("URL must be set");
    super::parse_yaml(content)
  }
}
