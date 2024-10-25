use chrono::Local;
use colored::Colorize;

pub fn log(text: &str, is_bad: bool) {
    let status = match is_bad {
        true => "[bad] ".red(),
        false => "[good]".green(),
    };
    let timestamp = Local::now().format("[%H:%M:%S]");
    println!("{} {} {}", status, timestamp, text);
}
