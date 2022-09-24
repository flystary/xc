extern crate colored;
use clap::{App, Arg, ArgMatches, SubCommand};
use crate::utils::net::get_cpe_by_sn_and_mode;
// use colored::*;

pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("conn")
        .about("Connect can be used to remotely connect CPE and display the process on the terminal.")
        .arg(
            Arg::with_name("sn")
                .required(true)
                .help("cpe serial number"),
        )
        //.arg(
            //Arg::with_name("remode")
                //.required(false)
                //.short("c")
                //.long("conn-mode")
                //.takes_value(true)
                //.possible_values(&["ssh", "telnet", "crt", "xshell"])
                // .value_name("Connet Mode")
                //.help("Use this option to select the remote CPE mode. Otherwise, the default version is SSH."),
        //)
        .arg(
            Arg::with_name("mode")
                .required(false)
                .short("m")
                .long("mode")
                .takes_value(true)
                .possible_value("valor")
                .possible_value("nexus")
                .possible_value("watsons")
                .possible_value("tassadar")
                .possible_value("watsons_ha")
                .multiple(false)
                .case_insensitive(false)
                .value_name("Mode")
                //.help("Use connet to business the CPE,the default version is valor."),
        )
}

pub fn run(args: &ArgMatches) {
    let sn = args.value_of("sn").unwrap();
    //let conn: &str = match args.value_of("connmode") {
    //    Some(m) => m,
    //    None    => "ssh",
    //};

    let mode: &str = match args.value_of("mode") {
        Some(m) => m,
        None    => "valor",
    };
    let cpe = get_cpe_by_sn_and_mode(sn, mode);
    if !cpe.check_master() && !cpe.check_backup() {
        println!("{}","Use CPE mode is Error.");
        return
    }
    println!("CPE {} is: {}","Mode",mode);
    cpe.show();

    let mut input = String::new();
    println!("Please select {} or {} login CPE :\t", "Master", "Backup");
    println!("\t1) Please select {} use Master entry login CPE.\t", "a");
    println!("\t2) Please select {} use Backup entry login CPE.\t", "b");
    println!("\t3) Please select {} use Remote port login CPE.\t", "c");
    println!("\t4) Please select {} or {} Exit terminal.\t","q","exit");

    let _bytes = std::io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "A" => {
            cpe.conn_master()
        }
        "a" => {
            cpe.conn_master()
        }
        "B" => {
            cpe.conn_backup()
        }
        "b" => {
            cpe.conn_backup()
        }
        "C" => {
            cpe.conn_backup()
        }
        "c" => {
            cpe.conn_backup()
        }
        "q" => {}
        "exit" => {}
        "" => {
            cpe.conn_master()
        }
        _ => {
            println!("{}","Input Error.");
        }
    }
}
