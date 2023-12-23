use std::sync::Arc;

use once_cell::sync::Lazy;
use reqwest::{Client, Response, Result};
use tokio::sync::RwLock;

use crate::config::HEADERS;

pub struct SignInClient {
    client: Client,
}

pub static CLIENT: Lazy<Arc<RwLock<SignInClient>>> =
    Lazy::new(|| Arc::new(RwLock::new(SignInClient::new())));

impl SignInClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// 发送get请求
    pub async fn get(&self, url: &str) -> Result<Response> {
        let headers = HEADERS.read().await.clone();

        self.client.get(url).headers(headers).send().await
    }

    pub async fn post(&self, url: &str, body: &str) -> Result<Response> {
        let headers = HEADERS.read().await.clone();

        self.client
            .post(url)
            .headers(headers)
            .body(body.to_string())
            .send()
            .await
    }
}
