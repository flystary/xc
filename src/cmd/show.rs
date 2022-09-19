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
                .possible_values(&["nexus","valor", "watsons", "watsons_ha", "tassadar"])
                // .value_name("Connet Mode")
                .help("Use show to select the CPE, the default version is nexus."),
        )
}
pub fn run(args: &ArgMatches) {
    let sn = args.value_of("sn").unwrap();
    let mode: &str = match args.value_of("mode") {
        Some(m) => m,
        None    => "nexus",
    };
    let cpe = get_cpe_by_sn_and_mode(sn, mode);
    if !cpe.check_master() && !cpe.check_backup() {
        println!("{}","Use CPE mode is Error.".red());
        return;
    }
    cpe.show();
}
