use chrono::Local;
use colored::Colorize;

pub enum LogStatus {
    Good,
    Bad,
    Info,
    Warn,
}

pub fn log(text: String, status: LogStatus) {
    let status = match status {
        LogStatus::Bad => "[bad!]".red(),
        LogStatus::Good => "[good]".green(),
        LogStatus::Info => "[info]".blue(),
        LogStatus::Warn => "[warn]".yellow(),
    };
    let timestamp = Local::now().format("[%H:%M:%S]");
    println!("{} {} {}", status, timestamp, text);
}
