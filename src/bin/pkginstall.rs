
extern crate clap;
extern crate colored;
extern crate pkginstall;
use clap::{App, Arg, ArgMatches};
use colored::*;
use pkginstall::logger::{LogType, log};

fn build_cli() -> App<'static, 'static> {
    App::new("pkginstall")
        .version("1.0.0")
        .author("WebD")
        .about("Easily install packages on your new OS")
        .arg(Arg::with_name("pkgmanager")
            .short("pkgm")
            .long("pkgmanager")
            .value_name("PKG_MANAGER")
            .help("The package manager that you want to use.")
            .takes_value(true))
}

fn main() {
    let args: ArgMatches = build_cli().get_matches();
    match args.value_of("pkgmanager") {
        Some(pkgm_name) => {
            match pkginstall::is_valid_pkg(pkgm_name) {
                Some(pkgm) => println!("{:?}", pkgm),
                None => log("Please provide a valid package manager.", LogType::Error)
            }
        },
        None => log("Please provide the package manager that you want to use.", LogType::Error)
    }
}