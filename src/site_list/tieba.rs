use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use log::{info, warn};
use md5::{Digest, Md5};
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};
use serde_json::Value;
use tokio::sync::RwLock;

use crate::{
    client::{SignInClient, CLIENT},
    config::URL,
};

const MAX_ATTEMPTS: usize = 5;

pub struct TiebaSignInClient {
    client: Arc<RwLock<SignInClient>>,
    tbs: String,
    list: (Vec<String>, Vec<String>),
}

impl TiebaSignInClient {
    pub fn new() -> Self {
        Self {
            client: CLIENT.clone(),
            tbs: String::new(),
            list: (Vec::<String>::new(), Vec::<String>::new()),
        }
    }

    /// 初始化
    pub async fn init(&mut self) -> Result<()> {
        self.tbs = self.get_tbs_json().await?;
        self.list = self.get_following_list().await?;

        Ok(())
    }

    pub async fn update(&mut self) -> Result<()> {
        self.init().await?;

        Ok(())
    }

    /// 获取tbs
    async fn get_tbs_json(&self) -> Result<String> {
        let client = self.client.read().await;

        let data: Value = client
            .get(URL.tbs_url.as_str())
            .await
            .context("请求发送失败")?
            .json()
            .await
            .context("Json数据解析失败")?;

        let tbs = data["is_login"]
            .as_i64()
            .and_then(|is_login| {
                if is_login == 1 {
                    data["tbs"].as_str()
                } else {
                    None
                }
            })
            .map(String::from)
            .ok_or_else(|| anyhow!("TBS获取失败"))?;

        Ok(tbs)
    }

    /// 获取关注列表并分为已签到和未签到两部分
    async fn get_following_list(&self) -> Result<(Vec<String>, Vec<String>)> {
        let client = self.client.read().await;

        let data: Value = client
            .get(URL.like_url.as_str())
            .await
            .context("请求发送失败")?
            .json()
            .await
            .context("Json数据解析失败")?;

        let following_list = data["data"]["like_forum"]
            .as_array()
            .ok_or_else(|| anyhow!("关注列表解析失败"))?;

        let (present, absent): (Vec<_>, Vec<_>) = following_list
            .into_par_iter()
            .filter_map(|item| {
                let is_sign = item["is_sign"].as_i64()?;
                let forum_name = item["forum_name"].as_str()?.into();
                Some((is_sign, forum_name))
            })
            .partition(|&(is_sign, _)| is_sign == 1);

        Ok((
            present.into_iter().map(|(_, name)| name).collect(),
            absent.into_iter().map(|(_, name)| name).collect(),
        ))
    }

    pub async fn sign_in(&mut self) -> Result<()> {
        info!("贴吧: 开始签到");

        let tbs = self.tbs.clone();
        let (_, absent) = self.list.clone();

        info!("贴吧: 未签到的贴吧数量: {}", absent.len());

        let pre_sign_in: Vec<(String, String)> = absent
            .par_iter()
            .map(|name| {
                let source = format!("kw={}tbs={}tiebaclient!!!", name, tbs);
                let digest = Md5::digest(source.as_bytes());
                format!("{:x}", digest)
            })
            .zip(absent.clone())
            .collect();

        {
            let client = self.client.read().await;

            for (sign, name) in pre_sign_in.iter() {
                let mut code = false;
                let mut attempt = 0;

                while !code && attempt < MAX_ATTEMPTS {
                    let body = format!("kw={}&tbs={}&sign={}", name, tbs, sign);

                    let res: Value = client
                        .post(&URL.sign_url, body.as_str())
                        .await?
                        .json()
                        .await?;

                    code = match res["error_code"].as_str() {
                        Some("0") => {
                            info!("{}: 签到成功", name.as_str());
                            true
                        }
                        _ => {
                            warn!("{}: 签到失败", name.as_str());
                            println!("{res:?}");
                            false
                        }
                    };

                    attempt += 1;
                }
            }
        }

        self.update().await?;

        // if present.is_empty() {
        //     println!("没有需要签到的贴吧");
        // } else {
        //     println!("已签到的贴吧：");
        //     for name in present {
        //         println!("{}", name);
        //     }
        // }

        if absent.is_empty() {
            info!("贴吧: 所有贴吧都已签到");
        } else {
            info!("未签到的贴吧：");
            for name in absent {
                info!("{}", name);
            }
        }

        Ok(())
    }
}
