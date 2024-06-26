extern crate toml;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Jump {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Sys {
    pub id:     String,
    pub path:   String,
    pub secret: String,
    pub loginurl: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Conf {
    pub sys: Sys,
    pub jump: Jump,
}

pub fn load_conf(path: PathBuf) -> Conf {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("exception:{}", e),
    };

    let mut str = String::new();
    match file.read_to_string(&mut str) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file: {}", e),
    };

    toml::from_str(&str).unwrap()
}
