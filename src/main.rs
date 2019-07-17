use actix_rt;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use chrono::prelude::*;
use prometheus::{Encoder, Registry, TextEncoder};
use std::env;
use std::sync::{Arc, Mutex};

mod metrics;
mod parser;

use parser::config;

// Metrics Endpoint handler
fn metrics_ep(state: web::Data<Arc<Mutex<Registry>>>, req: HttpRequest) -> HttpResponse {
  let mut buffer = vec![];
  let encoder = TextEncoder::new();
  // Gather the metrics
  let metric_families = state.lock().unwrap().gather();
  encoder.encode(&metric_families, &mut buffer).unwrap();

  let dt = Local::now();
  println!("{}: Scraped. Details: ", dt);
  println!("{:?}", req);

  HttpResponse::Ok().body(format!("{}", String::from_utf8(buffer).unwrap()))
}

// Healthcheck/Liveness Endpoint handler
fn liveness_ep(req: HttpRequest) -> HttpResponse {
  println!("{:?}", req);

  HttpResponse::Ok()
    .content_type("text/plain")
    .body(format!("Ok!"))
}

// Main function
fn main() -> std::io::Result<()> {
  let sys = actix_rt::System::new("pinger-rs");
  let args: Vec<String> = env::args().collect();
  let mut filename_arg = None;

  if args.len() > 1 {
    filename_arg = Some(&args[1]);
  }

  let conf = config::get_config(filename_arg).expect("Failed to load YAML config.");

  let metrics_o = metrics::Metrics::new(conf.clone())?;
  metrics_o.init();
  let registry = metrics_o.registry;

  //TODO: Try to understand this, not sure why do I need Arc Mutex
  let shared_registry = Arc::new(Mutex::new(registry.clone()));

  let host = conf["server"]["host"].as_str().unwrap_or("127.0.0.1");
  let port = conf["server"]["port"].as_u64().unwrap_or(8080);

  let metrics_endpoint = conf["server"]["endpoint"].as_str().unwrap_or("/metrics");
  let liveness_endpoint = conf["server"]["health"].as_str().unwrap_or("/healthcheck");

  let metrics_endpoint_str = metrics_endpoint.to_string();
  let liveness_endpoint_str = liveness_endpoint.to_string();

  HttpServer::new(move || {
    App::new()
      .data(shared_registry.clone())
      .service(web::resource(&liveness_endpoint_str).to(liveness_ep))
      .service(web::resource(&metrics_endpoint_str).to(metrics_ep))
  })
  .bind(format!("{}:{}", host, port))?
  .start();

  println!(
    "Endpoint ready to be scraped at: http://{}:{}{}",
    host, port, metrics_endpoint
  );
  println!(
    "Healthcheck ready at: http://{}:{}{}",
    host, port, liveness_endpoint
  );

  sys.run()
}
