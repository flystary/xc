extern crate colored;
extern crate tabled;

use crate::cli::node;
use crate::cli::oss;
use crate::cli::show;
use crate::cli::config;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::process::Command;



pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("connet")
        .about("connet vbox")
        .arg(
            Arg::with_name("sn")
                .required(true)
                .help("vbox serial number"),
        )
        .arg(
            Arg::with_name("connet_mode")
                .required(false)
                .short("c")
                // .long("mode")
                .takes_value(true)
                .possible_values(&["ssh", "telnet", "crt", "xshell"])
                // .value_name("Connet Mode")
                .help("ConnetMode"),
        )
        .arg(
            Arg::with_name("cpe_mode")
                .required(false)
                .short("m")
                // .long("mode")
                .takes_value(true)
                .possible_values(&["nexus", "watsons", "watsons_ha"])
                // .value_name("Connet Mode")
                .help("CpeMode: "),
        )
        
}

pub fn run(args: &ArgMatches) {
    let c_type = args.value_of("connet_mode");
    let sn = args.value_of("sn").unwrap().to_string();
    let connet_mode: &str;
    match c_type {
        Some(c_type) => {
            connet_mode = c_type;
        }
        None => {
            connet_mode = "ssh";
        }
    }

    let cpe_type = args.value_of("cpe_mode");
    let cpe_mode: &str;
    match cpe_type {
        Some(c_type) => {
            cpe_mode = c_type;
        }
        None => {
            cpe_mode = "nexus";
        }
    }

    let cpe_path = format!("/etc/xc/cpe/{}_cpe_kv.json", cpe_mode).to_string();
    let cf = String::from(cpe_path);
    let (model, version, master_cpe_id, master_cpe_ip, backup_cpe_id, backup_cpe_ip) = oss::oss(cf, sn.clone());

    let net_path = format!("/etc/xc/net/{}_net_kv.json", cpe_mode).to_string();
    let nf = String::from(net_path);
    let (master_node_ip, backup_node_ip) = node::net(nf, master_cpe_id, backup_cpe_id);

    show::show(model, version, sn, &master_node_ip, &master_cpe_ip, &backup_node_ip, &backup_cpe_ip);
    println!("你好, 当前选择的远程方式: {}\n", connet_mode);
    let path = String::from("/etc/xc/xc.toml");
    let (user, password) = config::read_config(path);

    Command::new("/usr/bin/expect")
    .arg("/etc/xc/bin/connet")
    .arg(master_node_ip)
    .arg(master_cpe_ip)
    .arg(user)
    .arg(password)
    // .arg(format!("{}@{}", user_name, ip_name))
    .status()
    .expect("登录失败!");
}

