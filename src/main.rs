#[macro_use]
extern crate clap;
mod cmd;
mod load;
mod utils;
use clap::App;
use utils::net::TOKEN;

fn run() {
    let matches = App::new(crate_name!())
        .author("flyZer0 <flyoney@163.com>")
        .about(crate_description!())
        .version(crate_version!())
        .subcommand(cmd::conn::subcommand())
        .subcommand(cmd::show::subcommand())
        .subcommand(cmd::list::subcommand())
        .subcommand(cmd::exec::subcommand())

        .get_matches();
    match matches.subcommand() {
        //cmd
        ("conn", Some(ucpe)) => cmd::conn::run(ucpe),
        ("show", Some(ucpe)) => cmd::show::run(ucpe),
        ("list", Some(ucpe)) => cmd::list::run(ucpe),
        ("exec", Some(ucpe)) => cmd::exec::run(ucpe),
        _ => eprintln!("No subcommand chosen. use --help | -h to view the subcommands."),
    }
}

fn main() {
    run();
    println!("{}", TOKEN.lock().unwrap());
}
