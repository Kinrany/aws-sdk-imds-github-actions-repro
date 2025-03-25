use std::error::Error;

use aws_config::BehaviorVersion;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    aws_config::load_defaults(BehaviorVersion::latest()).await;
    Ok(())
}
