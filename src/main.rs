use clap::Parser;
mod grpc;

#[derive(Parser)]
struct Args {
    #[arg(short = 'p', long = "port")]
    port: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    // Start the grpc server
    grpc::internal::server::start(args.port).await?;
    Ok(())
}
