# pinger-rs
Lifetime checker based on Rust :gear:

It will export the data to the endpoint that can be scraped by Prometheus :fire:

## How to run
1. Make sure `cargo` and `rustc` are installed
2. Check out the `default.yaml` at `config` directory
3. Do the changes necessary
4. `cargo run`
5. Wait until the dispatched requests
6. Go to endpoint `/metrics`

### Dev Note
This project is basically just another practice, just like testing the water on the Rust world.
There was a CRUD microservice based on Diesel and Rocket. But I wasn't so satisfied with the result.