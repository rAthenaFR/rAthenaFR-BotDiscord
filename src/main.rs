mod app;
mod cache;
mod config;
mod discord;
mod infra;
mod rathenafr;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    app::run().await
}
