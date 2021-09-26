use colored::Colorize;

#[allow(dead_code)]
pub enum LogLevel {
    Debug,
    Success,
    Info,
    Warn,
    Error,
}

pub fn soft_panic(msg: String) -> ! {
    log(&msg, LogLevel::Error);
    std::process::exit(1);
}

pub fn log(msg: &str, level: LogLevel) {
    match level {
        LogLevel::Debug => println!("{}", msg.blue()),
        LogLevel::Warn => println!("{}", msg.yellow()),
        LogLevel::Error => println!("{}", msg.red()),
        LogLevel::Success => println!("{}", msg.green()),
        _ => println!("{}", msg),
    };
}
