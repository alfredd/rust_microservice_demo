use hello::{hello_client::HelloClient, HelloRequest};
use std::io;

pub mod hello {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = HelloClient::connect("http://0.0.0.0:50051").await?;

    loop {
        println!("What would you like to say to the server? ");
        let mut message = String::new();
        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read line");

        let request = tonic::Request::new(HelloRequest {
            message: String::from(message),
        });
        let response = client.hello_world(request).await?;
        println!("The server says: '{}'", response.into_inner().message);
    }
}
