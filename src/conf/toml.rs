extern crate toml;
 
use std::fs::File;
use std::io::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String
}
#[derive(Debug, Deserialize)]
pub struct Url {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct Conf {
    pub user: User,
    pub url:  Url,
}

pub fn load_conf(path: String) -> Conf {
    
    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => panic!("no such file {} exception:{}", path, e)
    };

    let mut str = String::new();
    match file.read_to_string(&mut str) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file: {}", e)
    };
    
    toml::from_str(&str).unwrap()

}