extern crate serde_yaml;
use serde_yaml::Value;
use std::io::prelude::*;

fn read_file(filename: &'static str) -> Option<String> {
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

  const DEFAULT_YAML: &'static str = r#"
  server:
    port: 8080
    host: "127.0.0.1"
  pinger:
    metric-name: "pinger_metrics"
    hosts:
      - "https://en.wikipedia.org/"
      - "http://example.com/"
    # interval in second
    interval: 3"#;

  pub fn get_config(filename: Option<&'static str>) -> Option<Value> {
    let content: String = super::read_file(filename.unwrap_or("./config/default.yaml"))
            .unwrap_or_else(|| String::from(DEFAULT_YAML));
    super::parse_yaml(content)
  }
}