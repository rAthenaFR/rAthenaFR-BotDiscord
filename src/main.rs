mod app;
mod cache;
mod config;
mod discord;
#[allow(dead_code)]
mod i18n;
mod infra;
mod rathenafr;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    app::run().await
}
