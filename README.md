# Commands

## Build the application
* Build Docker image: `docker build -t echo_service .`
* Run container: `docker run -p 50051:50051 -it echo_service`


## Using the client
* `cd echo_service`
* `cargo build`
* `cargo run --bin client`
