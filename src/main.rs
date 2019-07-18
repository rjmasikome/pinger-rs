use std::env;

mod metrics;
mod parser;
mod service;

use parser::config;

// Main function
fn main() -> std::io::Result<()> {

  let args: Vec<String> = env::args().collect();
  let mut filename_arg = None;
  if args.len() > 1 {
    filename_arg = Some(&args[1]);
  }

  let conf = config::get_config(filename_arg).expect("Failed to load YAML config.");

  let metrics_obj = metrics::Metrics::new(conf.clone())?;
  metrics_obj.init();
  let registry = metrics_obj.registry;

  let service_obj = service::Service::new(conf.clone(), registry.clone())?;
  service_obj.start()
}
