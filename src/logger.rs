use colored::*;

pub enum LogType {
    Error,
    Success,
    Loading
}

pub fn log(msg: &str, log_type: LogType) {
    match log_type {
        LogType::Error => println!("{}", msg.red()),
        LogType::Success => println!("{}", msg.green()),
        LogType::Loading => println!("{}", msg.yellow())
    }
}