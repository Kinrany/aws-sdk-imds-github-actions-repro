use std::error::Error;

use aws_config::BehaviorVersion;
use aws_credential_types::provider::ProvideCredentials;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let credentials_provider = sdk_config
        .credentials_provider()
        .ok_or("No credentials provider")?;
    let _credentials = credentials_provider.provide_credentials().await?;
    Ok(())
}
