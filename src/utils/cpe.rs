extern crate colored;
extern crate tabled;
use colored::*;

use tabled::{Tabled, Table, Style};
use tabled::{Full, Modify, Row, Alignment, Indent, Head, Format};
use std::process::Command;
use std::vec;
use crate::utils::net::{
    init_toml,
};
#[derive(Tabled)]
pub struct Cpe {
    sn: String,
    model: String,
    version: String,
    port: String,
    sync_time: String,
    master_pop_ip: String,
    master_cpe_ip: String,
    backup_pop_ip: String,
    backup_cpe_ip: String,
}

impl Cpe {
    pub fn new(sn:String, model:String, version:String, sync_time: String,port:String, master_pop_ip:String, master_cpe_ip:String, backup_pop_ip:String, backup_cpe_ip:String) -> Cpe {
        Cpe {sn,model,version,sync_time,port,master_pop_ip,master_cpe_ip,backup_pop_ip,backup_cpe_ip,}
    }
    pub fn show(&self) {
        let v = vec![self];
        let table = Table::new(v)
            .with(Style::GITHUB_MARKDOWN)
            .with(Modify::new(Full).with(Indent::new(1, 1, 0, 0)))
            .with(Modify::new(Head).with(Alignment::center_horizontal()))
            .with(Modify::new(Row(1..)).with(Alignment::center_horizontal()))
            .with(Modify::new(Row(0..1)).with(Format(|s|s.white().bold().to_string())))
            .with(Modify::new(Row(1..)).with(Format(|s|s.white().bold().to_string())));

        println!("{}", table);
    }
    pub fn check_master(&self) -> bool {
        if self.master_cpe_ip.is_empty() && self.master_pop_ip.is_empty() {
            return false;
        }
        true
    }
    pub fn check_backup(&self) -> bool {
        if self.backup_cpe_ip.is_empty() && self.backup_pop_ip.is_empty() {
            return false;
        }
        true
    }
    pub fn connet_master(&self) {
        let conf = init_toml();
        if self.master_cpe_ip.as_str() == "0.0.0.0" || self.master_pop_ip.as_str() == "0.0.0.0" {
            println!("{}", "CPE Master pop or cpe is 0.0.0.0".red().bold());
            return
        }
        if self.master_cpe_ip.is_empty() || self.master_pop_ip.is_empty() {
            println!("{}", "CPE Master pop or cpe is None".red().bold());
            return
        }
        if conf.user.username.is_empty() || conf.user.password.is_empty() {
            println!("{}", "LOGIN CPE Username or password is None".red().bold());
            return
        }
        Command::new("/usr/bin/expect")
                .arg("/etc/xc/bin/connet")
                .arg(&self.master_pop_ip)
                .arg(&self.master_cpe_ip)
                .arg(conf.user.username)
                .arg(conf.user.password)
                // .arg(format!("{}@{}", user_name, ip_name))
                .status()
                .expect("登录失败!");
    }
    pub fn connet_backup(&self) {
        let conf = init_toml();
        if self.backup_cpe_ip.as_str() == "0.0.0.0" || self.backup_pop_ip.as_str() == "0.0.0.0" {
            println!("{}", "CPE Backup pop or cpe is 0.0.0.0".red().bold());
            return
        }
        if self.backup_cpe_ip.is_empty() || self.backup_pop_ip.is_empty() {
            println!("{}", "CPE Backup pop or cpe is None".red().bold());
            return
        }
        if conf.user.username.is_empty() || conf.user.password.is_empty() {
            println!("{}", "LOGIN CPE Username or password is None".red().bold());
            return
        }
        Command::new("/usr/bin/expect")
                .arg("/etc/xc/bin/connet")
                .arg(&self.backup_pop_ip)
                .arg(&self.backup_cpe_ip)
                .arg(conf.user.username)
                .arg(conf.user.password)
                // .arg(format!("{}@{}", user_name, ip_name))
                .status()
                .expect("登录失败!");  
    }
}

