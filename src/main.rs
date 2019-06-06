use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse};
use prometheus::{Registry, TextEncoder, Encoder};
use std::sync::{Mutex, Arc};

mod metrics;
mod parser;

use parser::config;

fn index(state: web::Data<Arc<Mutex<Registry>>>, req: HttpRequest) -> HttpResponse {

  let mut buffer = vec![];
  let encoder = TextEncoder::new();
  
  // Gather the metrics
  let metric_families = state.lock().unwrap().gather();
  encoder.encode(&metric_families, &mut buffer).unwrap();
  println!("{:?}", req);

  HttpResponse::Ok()
    .body(format!("{}", String::from_utf8(buffer).unwrap()))
}

fn main() -> std::io::Result<()> {

  let conf = config::get_config(None).expect("Failed to load YAML config.");

  let metrics_o = metrics::Metrics::new(conf.clone())?;
  metrics_o.init();
  let registry = metrics_o.registry;
  let shared_registry = Arc::new(Mutex::new(registry.clone()));

  // let server_conf = conf["server"];
  let host = conf["server"]["host"].as_str().unwrap_or("127.0.0.1");
  let port = conf["server"]["port"].as_str().unwrap_or("8080");

  // TODO: Make endpoint configurable if possible

  HttpServer::new(move ||
    App::new()
      .data(shared_registry.clone())
      .service(
        web::resource("/metrics").to(index))
      )
      .bind(format!("{}:{}", host, port))?
      .run()
}