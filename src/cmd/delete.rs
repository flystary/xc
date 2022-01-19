use clap::{App, Arg, ArgMatches, SubCommand};
use std::io::{self, Write};

pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("delete")
        .about("Delete an net service")
        .arg(
            Arg::with_name("net")
                .required(true)
                .possible_values(&["arp", "gre", "lan", "vlan", "ipsec"])
                .help("Delete net type is"),
        )
        .arg(
            Arg::with_name("name")
                .required(true)
                .takes_value(true)
                // .validator(is_file_exists)
                .help("Delete net name"),
        )
        .arg(
            Arg::with_name("all")
                .long("all")
                .conflicts_with("all")
                .help("delte all net name (all)"),
        )
}

pub fn run(args: &ArgMatches) {
    let net_type = args.value_of("net").unwrap();
    let net_name = args.value_of("name").unwrap();
    println!(
        "Are you sure you want to delete type : {} name : {} [N/y]? ",
        net_type, net_name
    );
    io::stdout().flush().unwrap();
    let mut answer = String::new();
    match io::stdin().read_line(&mut answer) {
        Ok(_) => {
            if answer.trim().to_lowercase() == "y" {
                if net_name == "asasa" {
                    println!("{}_{} successfully deleted", net_type, net_name);
                } else {
                    println!("{}{} does not exist", net_type, net_name);
                }
            } else {
                println!("Abort.");
            }
        }
        Err(_) => eprintln!("Failed to read input"),
    };
}

// fn is_net_name_exists(net_name: &str) ->  bool {
//     // let mut net_name = net_name.to_string();
//     if net_name == "sas" {
//         return true;
//     } else {
//         return false;
//         println!("{}",net_name);
//     }
// }
