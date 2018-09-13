#[derive(Debug)]
pub struct PKGManager {
    name: String,
    install_command: String
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