use anyhow::Result;
use config::init;
use env_logger::Env;
use log::{error, info};
use site_list::tieba::TiebaSignInClient;

mod client;
mod config;
mod site_list;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    match init().await {
        Ok(_) => info!("初始化成功"),
        Err(e) => {
            error!("初始化失败: {}", e);
            panic!()
        }
    }

    let mut client = TiebaSignInClient::new();

    match client.init().await {
        Ok(_) => info!("贴吧初始化成功"),
        Err(e) => error!("贴吧初始化失败: {}", e),
    }

    match client.sign_in().await {
        Ok(_) => info!("贴吧签到成功"),
        Err(e) => error!("贴吧签到失败: {}", e),
    }

    Ok(())
}
