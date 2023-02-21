use hello::{
    hello_server::{Hello, HelloServer},
    HelloRequest, HelloResponse,
};
use tonic::{transport::Server, Request, Response, Status};

pub mod hello {
    tonic::include_proto!("hello");
}

#[derive(Debug, Default)]
pub struct HelloService {}

#[tonic::async_trait]
impl Hello for HelloService {
    async fn hello_world(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request from {:?}", request.remote_addr());
        let reply = format!("S:{}", request.into_inner().message)
            .trim()
            .to_string();

        Ok(Response::new(HelloResponse { message: reply }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let greeter = HelloService::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(HelloServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
