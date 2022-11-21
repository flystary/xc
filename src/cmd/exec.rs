extern crate colored;
use clap::{App, Arg, ArgMatches, SubCommand};
use colored::*;

pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("exec")
        .about("Use show to obtain CPE information and display it on the current terminal")
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
                .possible_value("watsonsha")
                .multiple(false)
                .case_insensitive(true)
                .value_name("Mode")
        )
}

pub async fn run(args: &ArgMatches<'_>) {
    let sns: Vec<_> = args.values_of("sn").unwrap().collect();
    let mode: &str = match args.value_of("mode") {
        Some(m) => m,
        None => "valor",
    };
    println!("CPE {} is: {}", "Mode".blue().bold(), mode.bold());
    println!("{:?}", sns);
    println!("此功能暂不支持！")
}
