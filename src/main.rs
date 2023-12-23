use anyhow::Result;
use config::init;
use env_logger::Env;
use site_list::tieba::TiebaSignInClient;

mod client;
mod config;
mod site_list;

#[tokio::main]
async fn main() -> Result<()> {
    init().await?;
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let mut client = TiebaSignInClient::new();

    client.init().await?;

    client.sign_in().await?;

    Ok(())
}
