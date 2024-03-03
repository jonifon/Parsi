use colored::Colorize;

#[allow(dead_code)]
pub enum LogLevel {
    Error,
    Info,
    Warn,
    Success,
}
pub fn log(level: LogLevel, message: &str) {
    let (symbol, color) = match level {
        LogLevel::Error => ("[-]", "red"),
        LogLevel::Warn => ("[!]", "yellow"),
        LogLevel::Info => ("[>]", "blue"),
        LogLevel::Success => ("[~]", "green"),
    };
    let formatted_message = format!("{} {}", symbol, message);
    match color {
        "red" => println!("{}", formatted_message.red()),
        "yellow" => println!("{}", formatted_message.yellow()),
        "blue" => println!("{}", formatted_message.blue()),
        "green" => println!("{}", formatted_message.green()),
        _ => println!("{}", formatted_message),
    }
}
#[allow(dead_code)]
pub fn error(message: &str) {
    log(LogLevel::Error, message);
}
#[allow(dead_code)]
pub fn warn(message: &str) {
    log(LogLevel::Warn, message);
}
pub fn info(message: &str) {
    log(LogLevel::Info, message);
}
pub fn success(message: &str) {
    log(LogLevel::Success, message);
}
