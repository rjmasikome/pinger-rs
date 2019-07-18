use actix_rt;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use chrono::prelude::*;
use prometheus::{Encoder, Registry, TextEncoder};
use serde_yaml::Value;
use std::io::Error;
use std::sync::{Arc, Mutex};

struct SharedData {
  registry: Registry,
  debug: bool,
}

pub struct Service {
  registry: Registry,
  config: Value,
}

// Metrics Endpoint handler
fn metrics_ep(data: web::Data<Arc<Mutex<SharedData>>>, req: HttpRequest) -> HttpResponse {
  let mut buffer = vec![];
  let encoder = TextEncoder::new();
  // Gather the metrics
  let metric_families = data.lock().unwrap().registry.gather();
  encoder.encode(&metric_families, &mut buffer).unwrap();

  let debug = data.lock().unwrap().debug;

  if debug {
    let dt = Local::now();
    println!("{}: Scraped. Details: ", dt);
    println!("{:?}", req);
  }

  HttpResponse::Ok().body(format!("{}", String::from_utf8(buffer).unwrap()))
}

// Healthcheck/Liveness Endpoint handler
fn liveness_ep(data: web::Data<Arc<Mutex<SharedData>>>, req: HttpRequest) -> HttpResponse {
  let debug = data.lock().unwrap().debug;

  if debug {
    let dt = Local::now();
    println!("{}: Checking Service Health. Details: ", dt);
    println!("{:?}", req);
  }

  HttpResponse::Ok()
    .content_type("text/plain")
    .body(format!("Ok!"))
}

impl Service {
  pub fn new(conf: Value, reg: Registry) -> Result<Service, Error> {
    let registry = reg;
    let config = conf;
    Ok(Service { registry, config })
  }

  pub fn start(&self) -> std::io::Result<()> {

    let sys = actix_rt::System::new("pinger-rs");

    let host = self.config["server"]["host"]
      .as_str()
      .unwrap_or("127.0.0.1");
    let port = self.config["server"]["port"].as_u64().unwrap_or(8080);

    let metrics_endpoint = self.config["server"]["endpoint"]
      .as_str()
      .unwrap_or("/metrics")
      .to_string();
    let liveness_endpoint = self.config["server"]["health"]
      .as_str()
      .unwrap_or("/healthcheck");

    let metrics_endpoint_str = metrics_endpoint.to_string();
    let liveness_endpoint_str = liveness_endpoint.to_string();

    let shared_data = Arc::new(Mutex::new(SharedData {
      registry: self.registry.clone(),
      debug: self.config["pinger"]["debug"].as_bool().unwrap_or(true),
    }));

    HttpServer::new(move || {
      App::new()
        .data(shared_data.clone())
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
}
