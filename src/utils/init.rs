use crate::load::conf::{load_conf, Conf};
use crate::load::route::{load_route, Route};
use anyhow::Result;
use std::path::{Path, PathBuf};

use once_cell::sync::OnceCell;

pub static TOKEN: OnceCell<String> = OnceCell::new();

pub async fn init_token() {
    if let Some(s) = super::net::get_token_by_resp().await {
        TOKEN.set(s).ok();
    }
}

/*

use once_cell::sync::Lazy;
use tokio::sync::Mutex;

static TOKEN: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

/// 获取全局 token，第一次调用时自动初始化
pub async fn get_token() -> String {
    let mut lock = TOKEN.lock().await;

    // 如果还没有初始化，自动获取
    if lock.is_none() {
        let token_value = super::net::get_token_by_resp()
            .await
            .expect("failed to fetch token");
        *lock = Some(token_value);
    }

    // clone 返回，保持内部 token 不被外部修改
    lock.as_ref().unwrap().clone()
}*/

/// 全局获取 token（封装成函数更方便）
pub fn get_token() -> &'static str {
    TOKEN.get().expect("TOKEN not initialized")
}

pub fn get_default_config(conf: &str) -> Result<PathBuf> {
    let paths = [
        format!("/etc/xc/{}", conf),
        format!("./{}", conf),
        format!("{}/xc/{}", std::env::var("HOME").unwrap(), conf),
    ];

    for path in paths.iter() {
        if Path::new(path).exists() {
            return Ok(Path::new(path).to_path_buf());
        }
    }
    Err(anyhow::anyhow!(
        "Config file not found: {}",
        paths.join(", ")
    ))
}

pub fn init_conf() -> Conf {
    let mut path = PathBuf::new();
    if let Ok(buf) = super::init::get_default_config("xc.toml") {
        path = buf
    }
    load_conf(path)
}

pub fn init_route() -> Route {
    load_route(init_conf().sys.path)
}
