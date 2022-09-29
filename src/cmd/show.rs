extern crate colored;
use clap::{App, Arg, ArgMatches, SubCommand};
use crate::utils::net::get_cpe_by_sn_and_mode;
use colored::*;

pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("show")
        .about("Use show to obtain CPE information and display it on the current terminal")
        .arg(
            Arg::with_name("sn")
                .required(true)
                .help("cpe serial number"),
        )
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
                .possible_value("watsonsha")
                .multiple(false)
                .case_insensitive(true)
                .value_name("Mode")
                //.help("Use show to select the CPE, the default version is valor."),
        )
}

pub fn run(args: &ArgMatches) {
    let sn = args.value_of("sn").unwrap();
    let mode: &str = match args.value_of("mode") {
        Some(m) => m,
        None    => "valor",
    };
    let cpe = get_cpe_by_sn_and_mode(sn, mode);
    if !cpe.check_master() && !cpe.check_backup() {
        println!("{}","Use CPE mode is Error.".red());
        return;
    }
    println!("CPE {} is: {}","Mode".blue().bold(),mode.bold());
    cpe.show();
}
