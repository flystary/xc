use clap::{App, Arg, ArgMatches, SubCommand};
extern crate colored;
extern crate tabled;
use colored::*;
// use tabled::Column;
use tabled::{Tabled, Table, Style};
use tabled::{Full, Modify, Row, Alignment, Indent, Head, Format};
use crate::cli::node;
use crate::cli::oss;


pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("show")
        .about("show cpe")
        .arg(
            Arg::with_name("sn")
                .required(true)
                .help("vbox serial number"),
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
    
    let sn = args.value_of("sn").unwrap().to_string();
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
    println!("{}: {}", cpe_mode, sn); 

    let cpe_path = format!("/etc/xc/cpe/{}_cpe_kv.json", cpe_mode).to_string();
    let of = String::from(cpe_path);
    let (model, version, master_cpe_id, master_cpe_ip,backup_cpe_id, backup_cpe_ip) = oss::oss(of, sn.clone());

    let net_path = format!("/etc/xc/net/{}_net_kv.json", cpe_mode).to_string();
    let nf = String::from(net_path);
    let (master_node_ip, backup_node_ip) = node::net(nf, master_cpe_id, backup_cpe_id);
    
    show(sn, model, version, &master_node_ip, &master_cpe_ip, &backup_node_ip, &backup_cpe_ip);
    
}



pub fn show(sn: String, model: String, version: String, master_node_ip: &String, master_cpe_ip: &String,backup_node_ip: &String, backup_cpe_ip: &String) {
    #[derive(Tabled)]
    pub struct Cpe {
        // connet_type: String,
        cpesn: String,
        model: String,
        version: String,
        // jump_server: String,
        master_entry_ip: String,
        master_cpe_ip: String,
        backup_entry_ip: String,
        backup_cpe_ip: String,
    }
    
    let v: Vec<Cpe> =  vec![
        Cpe {
            cpesn:String::from(sn),
            model:String::from(model),
            version:String::from(version),
            // connet_type:String::from(connet_mode),
            // jump_server:String::from("master.jump.7x-networks.net"),
            master_entry_ip:String::from(master_node_ip),
            master_cpe_ip:String::from(master_cpe_ip),
            backup_entry_ip:String::from(backup_node_ip),
            backup_cpe_ip:String::from(backup_cpe_ip),
    }];
    let table = Table::new(v)
        // .with(Style::psql())
        // .with(Style::noborder());
        .with(Style::GITHUB_MARKDOWN)
        // .with(Rotate::Left);
        .with(Modify::new(Full).with(Indent::new(1, 1, 0, 0)))
        .with(Modify::new(Head).with(Alignment::center_horizontal()))
        .with(Modify::new(Row(1..)).with(Alignment::center_horizontal()))
        // .with(Modify::new(Column(0..)).with(Alignment::center_horizontal()))
        .with(Modify::new(Row(0..1)).with(Format(|s| s.bold().green().to_string())))
        .with(Modify::new(Row(1..)).with(Format(|s| s.italic().white().to_string())));
    // .with(Modify::new(Column(..1).and(Column(2..))).with(Format(|s| s.red().to_string())));
    println!("\n{}", table);
}