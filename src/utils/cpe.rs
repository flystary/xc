extern crate colored;
extern crate tabled;
// use colored::*;

use crate::utils::net::init_toml;
use std::process::Command;
use std::vec;
use tabled::{Alignment, Format, Full, Head, Indent, Modify, Row};
use tabled::{Style, Table, Tabled};

#[derive(Tabled)]
pub struct Cpe {
    pub(crate) sn: String,
    pub(crate) model: String,
    pub(crate) version: String,
    pub(crate) updatetime: String,
    pub(crate) masterpopip: String,
    pub(crate) mastercpeip: String,
    pub(crate) backuppopip: String,
    pub(crate) backupcpeip: String,
    pub(crate) remoteport: String,
}

pub type Cpes = Vec<Cpe>;

pub trait Dis {
    fn show(&self);
}

impl Dis for Cpe {
    fn show(&self) {
        let table = Table::new(vec![self])
            //.with(Style::GITHUB_MARKDOWN)
            .with(Style::ASCII)
            .with(Modify::new(Full).with(Indent::new(1, 1, 0, 0)))
            .with(Modify::new(Head).with(Alignment::center_horizontal()))
            .with(Modify::new(Row(1..)).with(Alignment::center_horizontal()))
            .with(Modify::new(Row(0..1)).with(Format(|s| s.to_uppercase())))
            .with(Modify::new(Row(1..)).with(Format(|s| s.to_string())));

        println!("{}", table);
    }
}

impl Dis for Cpes {
    fn show(&self) {
        let table = Table::new(self)
            //.with(Style::GITHUB_MARKDOWN)
            .with(Style::ASCII)
            .with(Modify::new(Full).with(Indent::new(1, 1, 0, 0)))
            .with(Modify::new(Head).with(Alignment::center_horizontal()))
            .with(Modify::new(Row(1..)).with(Alignment::center_horizontal()))
            .with(Modify::new(Row(0..1)).with(Format(|s| s.to_uppercase())))
            .with(Modify::new(Row(1..)).with(Format(|s| s.to_string())));

        println!("{}", table);
    }
}

pub trait Conn {
    fn check_master(&self) -> bool;
    fn check_backup(&self) -> bool;
    fn conn_master(&self);
    fn conn_backup(&self);
}

impl Conn for Cpe {
    fn check_master(&self) -> bool {
        if self.mastercpeip.is_empty() && self.masterpopip.is_empty() {
            return false;
        }
        true
    }
    fn check_backup(&self) -> bool {
        if self.backupcpeip.is_empty() && self.backuppopip.is_empty() {
            return false;
        }
        true
    }
    fn conn_master(&self) {
        let conf = init_toml();
        if self.mastercpeip.as_str() == "0.0.0.0" || self.masterpopip.as_str() == "0.0.0.0" {
            println!("CPE Master pop or cpe is 0.0.0.0");
            return;
        }
        if self.mastercpeip.is_empty() || self.masterpopip.is_empty() {
            println!("CPE Master pop or cpe is None");
            return;
        }
        if conf.jump.username.is_empty() || conf.jump.password.is_empty() {
            println!("LOGIN CPE Username or password is None");
            return;
        }
        if cfg!(target_os = "linux") {
            Command::new("/usr/bin/expect")
                .arg("/etc/xc/bin/connet")
                .arg(&self.masterpopip)
                .arg(&self.mastercpeip)
                .arg(conf.jump.username)
                .arg(conf.jump.password)
                .status()
                .expect("登录失败!");
        } else if cfg!(target_os = "window") {
            println!("Windows此功能暂不支持")
        } else {
            println!("此功能暂不支持")
        }
    }
    fn conn_backup(&self) {
        let conf = init_toml();
        if self.backupcpeip.as_str() == "0.0.0.0" || self.backuppopip.as_str() == "0.0.0.0" {
            println!("CPE Backup pop or cpe is 0.0.0.0");
            return;
        }
        if self.backupcpeip.is_empty() || self.backuppopip.is_empty() {
            println!("CPE Backup pop or cpe is None");
            return;
        }
        if conf.jump.username.is_empty() || conf.jump.password.is_empty() {
            println!("LOGIN CPE Username or password is None");
            return;
        }
        if cfg!(target_os = "linux") {
            Command::new("/usr/bin/expect")
                .arg("/etc/xc/bin/connet")
                .arg(&self.backuppopip)
                .arg(&self.backupcpeip)
                .arg(conf.jump.username)
                .arg(conf.jump.password)
                // .arg(format!("{}@{}", user_name, ip_name))
                .status()
                .expect("登录失败!");
        } else if cfg!(target_os = "window") {
            println!("Windows此功能暂不支持")
        } else {
            println!("此功能暂不支持")
        }
    }
}
