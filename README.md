# pinger-rs

HTTP Status checker and latency metrics exporter based on Rust :gear:

It will export the data to the endpoint that can be scraped by Prometheus :fire:

More features are coming soon.

## Features
- [x] Configurable metrics endpoint
- [x] Configurable url targets to be called
- [x] Configurable polling duration
- [x] Configurable metrics name
- [x] Debug for every polling (Configurable)
- [x] Configurable healthcheck endpoint (e.g for Kubernetes)
- [x] Config file name as executable argument parameter
- [x] Counter, Gauge and Histogram metrics included
- [ ] Configurable buckets on histogram metric
- [ ] Configurable target alias

## How to run
1. Make sure `cargo` and `rustc` are installed
2. Check out the `default.yaml` at `config` directory or `config.yaml`
3. Do the changes necessary
4. Either run `cargo run` or `cargo build` and then `./start.sh` (You can check the content of `start.sh` on how to run it)
5. Wait until the dispatched requests
6. Go to the endpoint `/metrics` or to the endpoint that has been configured

## License
MIT
