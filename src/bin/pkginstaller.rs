extern crate clap;
extern crate pkginstaller;
use clap::{App, Arg, ArgMatches};
use std::fs::File;
use pkginstaller::logger::{LogType, log};

fn build_cli() -> App<'static, 'static> {
    App::new("pkginstaller")
        .version("1.0.0")
        .author("WebD")
        .about("Easily install packages on your new OS")
        .arg(Arg::with_name("pkgmanager")
            .short("pkgm")
            .long("pkgmanager")
            .value_name("PKG_MANAGER")
            .help("The package manager that you want to use.")
            .takes_value(true))
        .arg(Arg::with_name("file")
            .short("f")
            .value_name("FILE")
            .help("JSON file where the pkg are listed")
            .default_value("packages.json")
            .takes_value(true))
}

fn main() {
    let args: ArgMatches = build_cli().get_matches();
    match args.value_of("pkgmanager") {
        Some(pkgm_name) => {
            match pkginstaller::is_valid_pkg(pkgm_name) {
                Some(pkgm) => {
                   let file_path = args.value_of("file").unwrap();
                   match File::open(file_path) {
                       Ok(file) => println!("{:?}", file),
                       Err(_s) => log("Unable to open the file", LogType::Error)
                   }
                },
                None => log("Please provide a valid package manager.", LogType::Error)
            }
        },
        None => log("Please provide the package manager that you want to use.", LogType::Error)
    }
}