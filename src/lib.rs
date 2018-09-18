extern crate colored;
extern crate json;

pub mod logger;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Error;
use std::process;


#[derive(Debug)]
pub struct PKGManager {
    name: String,
    install_command: String
}

impl PKGManager {
    pub fn install(&self, file: File) -> Result<(), Error> {
        let mut reader = BufReader::new(file);
        let mut buffers = String::new();
        reader.read_to_string(&mut buffers)?;
        match json::parse(&buffers) {
            Ok(json) => {
                let pkgs: Vec<&str> = json.members().map(|pkg| pkg.as_str().unwrap()).collect();
                println!("{:?}", pkgs);
            },
            Err(_) => {
                logger::log("Failed to parse the json", logger::LogType::Error);
                process::exit(0);
            }
        }
        Ok(())
    }
}


fn get_pkgm() -> Vec<PKGManager> {
    vec![PKGManager{
        name: String::from("apt"),
        install_command: String::from("apt install {pkg}")
    }]
}

pub fn is_valid_pkg(name: &str) -> Option<PKGManager> {
    get_pkgm().into_iter().find(|pkgm| pkgm.name == name.to_lowercase()) 
}