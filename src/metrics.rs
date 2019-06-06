use schedule_recv::{periodic_ms};
use std::io::{Error};
use std::thread;
use serde_yaml::Value;

use prometheus::{Opts, Registry, Counter};
use curl::easy::Easy;

pub struct Metrics {
    pub registry: Registry,
    config: Value,
}

impl Metrics {

  pub fn new(conf: Value) -> Result<Metrics, Error>{

    let registry = Registry::new();
    let config = conf;
    Ok(Metrics { registry, config,})
  }

  fn polling(&self, counter: Counter, url: &'static str) {

    thread::spawn(move || {

      let delay = periodic_ms(3000);
      let mut x = 0;
      let mut easy = Easy::new();

      loop {

        delay.recv().unwrap();
        let closure_count = counter.clone();

        easy.url(url).unwrap();
        easy.write_function(move |data| {
          closure_count.inc();
          Ok(data.len())
        }).unwrap();
        easy.perform().unwrap();

        x += 1;
        println!("{} - {}", x, url);
      }
    });

  }

  pub fn init(&self) {

    // Create a Counter.
    let metrics_name = self.config["pinger"]["metric-name"] 
      .as_str().unwrap_or("pinger_metrics");

    let counter_opts = Opts::new(metrics_name, "test counter help");
    let counter = Counter::with_opts(counter_opts).unwrap();

    // Register Counter
    self.registry.register(Box::new(counter.clone())).unwrap();

    let urls: Vec<Value> = self.config["pinger"]["hosts"]
      .as_sequence().unwrap().to_vec();

    for url in urls {
      match url_clone.as_str() {
        Some(x) => self.polling(counter, x),
        None => println!("No match found :("),
      }
    }
  }
}