use std::io::Error;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::prelude::*;
use curl::easy::Easy;
use prometheus::{CounterVec, GaugeVec, HistogramVec, HistogramOpts, Opts, Registry};
use schedule_recv::periodic_ms;
use serde_yaml::Value;

pub struct Metrics {
  pub registry: Registry,
  config: Value,
}

struct MetricsCollection {
  rate: CounterVec,
  latency: CounterVec,
  gauge: GaugeVec,
  histogram: HistogramVec,
}

impl Metrics {
  pub fn new(conf: Value) -> Result<Metrics, Error> {
    let registry = Registry::new();
    let config = conf;
    Ok(Metrics { registry, config })
  }

  fn polling(&self, metrics_collection: MetricsCollection, urls: Vec<Value>) {
    let interval_ms = self.config["pinger"]["interval"]
      .as_u64() // Can't parse straight to u32
      .unwrap_or(10) as u32
      * 1000;
    let debug = self.config["pinger"]["debug"].as_bool().unwrap_or(true);

    let counter_rate_vec = metrics_collection.rate;
    let counter_lat_vec = metrics_collection.latency;
    let gauge_lat_vec = metrics_collection.gauge;
    let histogram_lat_vec = metrics_collection.histogram;

    thread::spawn(move || {
      let delay = periodic_ms(interval_ms);
      let mut easy = Easy::new();

      println!("Polling every {} seconds", interval_ms / 1000);
      println!("Debug is set to {}", debug);

      loop {
        delay.recv().unwrap();

        for url_serde in urls.clone() {
          let url = url_serde.as_str().expect("URL must be set");
          let start = SystemTime::now();
          let start_ts = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

          easy.url(url).unwrap();
          easy.write_function(|data| Ok(data.len())).unwrap();

          let dt = Local::now();
          match easy.perform() {
            Ok(_) => {
              let code = easy.response_code().unwrap().to_string();
              let finish = SystemTime::now();
              let finish_ts = finish
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis();
              let latency = (finish_ts - start_ts) as f64;

              // Increase Rate by 1
              counter_rate_vec.with_label_values(&[&code, url]).inc();

              // Increase Latency by the difference of timestamps
              counter_lat_vec
                .with_label_values(&[&code, url])
                .inc_by(latency);

              gauge_lat_vec
                .with_label_values(&[&code, url])
                .set(latency);

              histogram_lat_vec
                .with_label_values(&[&code, url])
                .observe(latency);

              if debug {
                println!("{}: {} - {}", dt, code, url);
              }
            }
            Err(_) => println!("{}: Error accessing {}", dt, url),
          }
        }
      }
    });
  }

  pub fn init(&self) {

    let metrics_name = self.config["pinger"]["metric-name"]
      .as_str()
      .unwrap_or("pinger_metrics");

    let counter_rate_suffix = self.config["pinger"]["rate-suffix"]
      .as_str()
      .unwrap_or("_rates");

    let gauge_lat_suffix = self.config["pinger"]["gauge-suffix"]
      .as_str()
      .unwrap_or("_gauge");

    let histogram_lat_suffix = self.config["pinger"]["histogram-suffix"]
      .as_str()
      .unwrap_or("_histogram");

    let default_buckets : Vec<f64> =
      [50f64, 100f64, 150f64, 200f64, 300f64, 400f64, 500f64, 750f64, 1000f64, 1500f64, 2000f64].to_vec();

    let counter_lat_name = metrics_name;
    let counter_lat_opts = Opts::new(counter_lat_name, "Latency counter");
    let counter_lat = CounterVec::new(counter_lat_opts, &["code", "url"]).unwrap();

    let counter_rates_name = [metrics_name, counter_rate_suffix].concat();
    let counter_rate_opts = Opts::new(counter_rates_name, "Rates Counter".to_string());
    let counter_rates = CounterVec::new(counter_rate_opts, &["code", "url"]).unwrap();

    let gauge_lat_name = [metrics_name, gauge_lat_suffix].concat();
    let gauge_lat_opts = Opts::new(gauge_lat_name, "Latency gauge".to_string());
    let gauge_lat = GaugeVec::new(gauge_lat_opts, &["code", "url"]).unwrap();

    let histogram_lat_name = [metrics_name, histogram_lat_suffix].concat();
    let histogram_lat_opts = HistogramOpts::new(histogram_lat_name, "Latency histogram".to_string()).buckets(default_buckets);
    let histogram_lat = HistogramVec::new(histogram_lat_opts, &["code", "url"]).unwrap();

    // Register Metrics
    self
      .registry
      .register(Box::new(counter_rates.clone()))
      .unwrap();

    self
      .registry
      .register(Box::new(counter_lat.clone()))
      .unwrap();

    self
      .registry
      .register(Box::new(gauge_lat.clone()))
      .unwrap();

    self
      .registry
      .register(Box::new(histogram_lat.clone()))
      .unwrap();

    let urls: Vec<Value> = self.config["pinger"]["targets"]
      .as_sequence()
      .unwrap()
      .to_vec();

    let metrics_collection = MetricsCollection {
      rate: counter_rates,
      latency: counter_lat,
      gauge: gauge_lat,
      histogram: histogram_lat,
    };

    self.polling(metrics_collection, urls);
  }
}
