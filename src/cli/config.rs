use serde::Deserialize;
extern crate toml;
 
use std::fs::File;
use std::io::prelude::*;


#[derive(Deserialize)]
    #[derive(Debug)]
struct User {
    username: String,
    password: String
}


#[derive(Deserialize)]
#[derive(Debug)]
struct Config {
    user: User,
}

pub fn read_config(path: String) -> (String, String) {
    
    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => panic!("no such file {} exception:{}", path, e)
    };

    let mut str = String::new();
    match file.read_to_string(&mut str) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file: {}", e)
    };

    let conf: Config = toml::from_str(&str).unwrap();
    println!("当前登录堡垒机的用户: {}", conf.user.username);
    println!("当前登录堡垒机的密码: {}\n", conf.user.password);

    return (conf.user.username, conf.user.password)

}