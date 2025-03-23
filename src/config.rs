use std::env;

use anyhow::{Context, Result};
use colored::Colorize;
use dotenv::dotenv;
use once_cell::sync::Lazy;
use reqwest::header::{COOKIE, HeaderMap, HeaderValue};
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
    dotenv().ok();

    let bduss_value = env::var("BDUSS")?;
    let value = HeaderValue::from_str(format!("BDUSS={}", bduss_value).as_str())
        .context("Cookie 错误".red())?;

    let mut headers = HEADERS.write().await;
    headers.insert(COOKIE, value);
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36"));
    headers.insert(
        "Referer",
        HeaderValue::from_static("http://tieba.baidu.com/"),
    );
    headers.insert("connection", HeaderValue::from_static("keep-alive"));
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers.insert("charset", HeaderValue::from_static("UTF-8"));

    Ok(())
}
