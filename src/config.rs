use std::env;

use anyhow::{Context, Result};
use dotenv::dotenv;
use once_cell::sync::Lazy;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct Url {
    pub like_url: String,
    pub tbs_url: String,
    pub sign_url: String,
}

pub static URL: Lazy<Url> = Lazy::new(|| Url {
    like_url: "https://tieba.baidu.com/mo/q/newmoindex".to_string(),
    tbs_url: "http://tieba.baidu.com/dc/common/tbs".to_string(),
    sign_url: "http://c.tieba.baidu.com/c/c/forum/sign".to_string(),
});

pub static HEADERS: Lazy<RwLock<HeaderMap>> = Lazy::new(|| RwLock::new(HeaderMap::new()));

/// 初始化配置
pub async fn init() -> Result<()> {
    dotenv()?;

    let bduss_value = env::var("BDUSS")?;
    let value =
        HeaderValue::from_str(format!("BDUSS={}", bduss_value).as_str()).context("Cookie error")?;

    let mut headers = HEADERS.write().await;
    headers.insert(COOKIE, value);

    Ok(())
}
