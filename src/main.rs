#[macro_use]
#[warn(unreachable_code)]
extern crate clap;
// extern crate data_encoding;
use clap::App;
mod cli;
mod cmd;


fn main() {
    let matches = App::new(crate_name!())
        .author("flyZer0 <flyoney@163.com>")
        .about(crate_description!())
        .version(crate_version!())
        .subcommand(cli::net::subcommand())
        .subcommand(cli::show::subcommand())
        .subcommand(cli::connet::subcommand())
        .subcommand(cmd::add::subcommand())
        // .subcommand(cmd::view::subcommand())
        // .subcommand(cmd::list::subcommand())
        .subcommand(cmd::delete::subcommand())
        .get_matches();

    match matches.subcommand() {
        //cli
        ("net", Some(box_m)) => cli::net::run(box_m),
        ("show", Some(box_m)) => cli::show::run(box_m),
        ("connet", Some(box_m)) => cli::connet::run(box_m),
        //cmd
        ("add", Some(sub_m)) => cmd::add::run(sub_m),
        ("delete", Some(sub_m)) => cmd::delete::run(sub_m),
        _ => eprintln!("No subcommand chosen. add --help | -h to view the subcommands."),
    }
}
//#[warn(unreachable_code)]
