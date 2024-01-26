use env_logger;
use log::info;
use credit_scoring::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting gRPC server...");
    run().await?;
    Ok(())
}
