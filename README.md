# pinger-rs
Lifetime checker based on Rust :gear:

It will export the data to the endpoint that can be scraped by Prometheus :fire:

## Features
- [x] Configurable metrics endpoint
- [x] Configurable hosts to be called
- [x] Configurable polling duration
- [x] Configurable metrics name
- [x] Debug for every polling (Configurable)
- [] Configurable healthcheck endpoint (e.g for Kubernetes)
- [] Config file name as executable argument parameter

## How to run
1. Make sure `cargo` and `rustc` are installed
2. Check out the `default.yaml` at `config` directory
3. Do the changes necessary
4. `cargo run`
5. Wait until the dispatched requests
6. Go to endpoint `/metrics` or the endpoint that has been configured

### Dev Note
This project is basically just another practice, just like testing the water on the Rust world.
There was a CRUD microservice based on Diesel and Rocket. But I wasn't so satisfied with the result.