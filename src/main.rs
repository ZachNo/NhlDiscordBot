mod discord;
mod error;
mod nhl;

use anyhow::Result;
use discord::client::create_client;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = create_client().await?;
    client.start().await?;
    Ok(())
}
