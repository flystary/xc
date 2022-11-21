#[macro_use]
extern crate clap;
extern crate lazy_static;
mod cmd;
mod load;
mod utils;
use clap::App;


async fn run() {
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
        ("conn", Some(ucpe)) => cmd::conn::run(ucpe).await,
        ("show", Some(ucpe)) => cmd::show::run(ucpe).await,
        ("list", Some(ucpe)) => cmd::list::run(ucpe).await,
        ("exec", Some(ucpe)) => cmd::exec::run(ucpe).await,
        _ => eprintln!("No subcommand chosen. use --help | -h to view the subcommands."),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run().await;
    Ok(())
}
