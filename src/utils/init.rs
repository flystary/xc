use crate::load::conf::{load_conf, Conf};
use crate::load::route::{load_route, Route};
use anyhow::Result;
use std::path::{Path, PathBuf};

use lazy_static::lazy_static;


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

lazy_static! {
    pub static ref TOKEN: String = {
        let mut token = String::new();
        if let Some(s) = super::net::get_token_by_resp() {
            token = s
        }
        token
    };
}
