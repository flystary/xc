use clap::{App, Arg, ArgMatches, SubCommand};
extern crate colored;
extern crate tabled;

pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("update")
        .about("Use update to update local CPE information")

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
                .possible_value("watsons_ha")
                .multiple(true)
                .case_insensitive(true)
                .value_name("Mode")
                //.help("Use show to select the CPE, the default version is valor."),
        )
}

pub fn run(args: &ArgMatches) {
    let sn = args.value_of("sn").unwrap();
    let cpemode = args.value_of("mode");

    let mode: &str =  match cpemode {
        Some(m) => m,
        None    => "valor",
    };
    println!("此功能暂不支持 {} {}", mode, sn);
}
