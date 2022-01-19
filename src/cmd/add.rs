use clap::{App, Arg, ArgMatches, SubCommand};
// use std::fs::File;
use std::path::Path;

// xc add -net gre -f /tmp/ces
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("add")
        .about("Add a new net service")
        .arg(
            Arg::with_name("net")
                .required(true)
                .possible_values(&["arp", "gre", "lan", "vlan", "ipsec"])
                .help("Name of the net"),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .required(true)
                .takes_value(true)
                .value_name("FILe")
                .validator(is_file_exists)
                .help("net file is json"),
        )
}

fn is_file_exists(value: String) -> Result<(), String> {
    let value = value.to_lowercase();
    // println!("{}", value);
    match Path::new(&value).exists() {
        true => Ok(()),
        false => Err(String::from("No such file or directory.")),
    }
}

pub fn run(args: &ArgMatches) {
    let net_name = args.value_of("net").unwrap();
    let path = args.value_of("file").unwrap();

    println!("{}-{}", net_name, path)
}
