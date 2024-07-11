use anyhow::Result;
use dotenvy::dotenv;
use outro_08::{serve, Config};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL IS NOT SET!");

    serve(Config { database_url }).await?;

    Ok(())
}
