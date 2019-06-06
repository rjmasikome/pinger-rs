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

  println!("{}", String::from_utf8(buffer.clone()).unwrap());
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

  HttpServer::new(move || 
    App::new()
      .data(shared_registry.clone())
      .service(
        web::resource("/metrics").to(index))
      )
      .bind("127.0.0.1:8080")?
      .run()
}