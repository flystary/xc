#[macro_use]
// #[warn(unreachable_code)]
extern crate clap;
mod conf;
mod cmd;
mod utils;
use clap::App;


fn run() {
    let matches = App::new(crate_name!())
        .author("flyZer0 <flyoney@163.com>")
        .about(crate_description!())
        .version(crate_version!())
        .subcommand(cmd::connet::subcommand())
        .subcommand(cmd::show::subcommand())
        .subcommand(cmd::update::subcommand())
        .get_matches();
    match matches.subcommand() {
        //cmd
        ("connet", Some(box_m)) => cmd::connet::run(box_m),
        ("show",   Some(box_m)) => cmd::show::run(box_m),
        ("update", Some(box_m)) => cmd::update::run(box_m),
        _ => eprintln!("No subcommand chosen. use --help | -h to view the subcommands."),
    }
}

fn main() {
    run()
}
