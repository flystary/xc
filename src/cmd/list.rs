extern crate colored;
use clap::{App, Arg, ArgMatches, SubCommand};
use crate::utils::{net::get_cpes_by_sn_mode, cpe::Dis};
// use colored::*;

pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("list")
        .about("Use list to obtain multiple CPE information and display it on the current terminal")
        .arg(
            Arg::with_name("sn")
                .required(true)
                .multiple(true)
                .takes_value(true)
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
                .possible_value("watsons_ha")
                .multiple(false)
                .case_insensitive(true)
                .value_name("Mode")
                //.help("Use show to select the CPE, the default version is valor."),
        )
}

pub fn run(args: &ArgMatches) {
    //let sn = args.values_of("sn").unwrap();
    let sns: Vec<_> = args.values_of("sn").unwrap().collect();
    let mode: &str = match args.value_of("mode") {
        Some(m) => m,
        None    => "valor",
    };
    println!("CPE Mode is: {}", mode);
    println!("{:?}", sns);
    if let Some(cpes) = get_cpes_by_sn_mode(mode, sns) {
        // #[derive(Debug)]
        cpes.show()
    }
    // if !cpe.check_master() && !cpe.check_backup() {
    //     println!("{}","Use CPE mode is Error.");
    //     return
    // }
    //println!("CPE mode is: {}", mode);


}
