extern crate colored;
extern crate tabled;
use colored::*;

use tabled::{Tabled, Table, Style};
use tabled::{Full, Modify, Row, Alignment, Indent, Head, Format};
use std::process::Command;
use std::vec;
use crate::tools::net::{
    init_toml,
};

#[derive(Tabled)]
pub struct Cpe {
    pub(crate) sn:      String,
    pub(crate) model:   String,
    pub(crate) version: String,
    pub(crate) updatetime:  String,
    pub(crate) masterpopip: String,
    pub(crate) mastercpeip: String,
    pub(crate) backuppopip: String,
    pub(crate) backupcpeip: String,
    pub(crate) remoteport:  String,
}

impl Cpe {

    pub fn show(&self) {
        let v = vec![self];
        let table = Table::new(v)
            //.with(Style::GITHUB_MARKDOWN)
            .with(Style::ASCII)
            .with(Modify::new(Full).with(Indent::new(1, 1, 0, 0)))
            .with(Modify::new(Head).with(Alignment::center_horizontal()))
            .with(Modify::new(Row(1..)).with(Alignment::center_horizontal()))
            .with(Modify::new(Row(0..1)).with(Format(|s|s.white().to_uppercase())))
            .with(Modify::new(Row(1..)).with(Format(|s|s.white().to_string())));

        println!("{}", table);
    }
    pub fn check_master(&self) -> bool {
        if self.mastercpeip.is_empty() && self.masterpopip.is_empty() {
            return false;
        }
        true
    }
    pub fn check_backup(&self) -> bool {
        if self.backupcpeip.is_empty() && self.backuppopip.is_empty() {
            return false;
        }
        true
    }
    pub fn conn_master(&self) {
        let conf = init_toml();
        if self.mastercpeip.as_str() == "0.0.0.0" || self.masterpopip.as_str() == "0.0.0.0" {
            println!("{}", "CPE Master pop or cpe is 0.0.0.0".red().bold());
            return
        }
        if self.mastercpeip.is_empty() || self.masterpopip.is_empty() {
            println!("{}", "CPE Master pop or cpe is None".red().bold());
            return
        }
        if conf.jump.username.is_empty() || conf.jump.password.is_empty() {
            println!("{}", "LOGIN CPE Username or password is None".red().bold());
            return
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
        };
    }
    pub fn conn_backup(&self) {
        let conf = init_toml();
        if self.backupcpeip.as_str() == "0.0.0.0" || self.backuppopip.as_str() == "0.0.0.0" {
            println!("{}", "CPE Backup pop or cpe is 0.0.0.0".red().bold());
            return
        }
        if self.backupcpeip.is_empty() || self.backuppopip.is_empty() {
            println!("{}", "CPE Backup pop or cpe is None".red().bold());
            return
        }
        if conf.jump.username.is_empty() || conf.jump.password.is_empty() {
            println!("{}", "LOGIN CPE Username or password is None".red().bold());
            return
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
        }
    }
}


pub type Cpes = Vec<Cpe>;

pub trait Dis {
    fn show(self);
}

impl Dis for Cpe {
    fn show(self) {
        let v = vec![self];
        let table = Table::new(v)
            //.with(Style::GITHUB_MARKDOWN)
            .with(Style::ASCII)
            .with(Modify::new(Full).with(Indent::new(1, 1, 0, 0)))
            .with(Modify::new(Head).with(Alignment::center_horizontal()))
            .with(Modify::new(Row(1..)).with(Alignment::center_horizontal()))
            .with(Modify::new(Row(0..1)).with(Format(|s|s.to_uppercase())))
            .with(Modify::new(Row(1..)).with(Format(|s|s.to_string())));

        println!("{}", table);

    }
}

impl Dis for Cpes {
    fn show(self) {
        let v = self;
        let table = Table::new(v)
            //.with(Style::GITHUB_MARKDOWN)
            .with(Style::ASCII)
            .with(Modify::new(Full).with(Indent::new(1, 1, 0, 0)))
            .with(Modify::new(Head).with(Alignment::center_horizontal()))
            .with(Modify::new(Row(1..)).with(Alignment::center_horizontal()))
            .with(Modify::new(Row(0..1)).with(Format(|s|s.to_uppercase())))
            .with(Modify::new(Row(1..)).with(Format(|s|s.to_string())));

        println!("{}", table);
    }

}
