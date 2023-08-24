mod data_model;
mod discord;
mod nhl;

use discord::client::create_client;

#[tokio::main]
async fn main() {
    let mut client = create_client().await;
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}