use tonic::{Request, Response, Status, transport::Server};

use lycee::{catch_backtrace, innermost_symbol};
use lycee::proto::helloworld::{HelloReply, HelloRequest};
use lycee::proto::helloworld::greeter_server::{Greeter, GreeterServer};

#[derive(Debug, Default)]
pub struct MyGreeter {}


#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);
        let b = catch_backtrace(0, 5);
        println!("symbol:\n {}\nbacktrace:\n{:?}", innermost_symbol(&b), b);
        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };
        Ok(Response::new(reply))
    }
}


#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50052".parse()?;
    let greeter = MyGreeter::default();
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
