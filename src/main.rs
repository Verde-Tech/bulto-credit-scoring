use env_logger;
use log::info;

fn main() {
    env_logger::init();
    info!("Service is starting up.");
    // Rest of the main function
}
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting gRPC server...");
    service::run().await?;
    Ok(())
}
