#[macro_export]
macro_rules! log_good {
    ($($arg:tt)*) => {
        let status = colored::Colorize::green("[good]");
        let timestamp = chrono::Local::now().format("[%H:%M:%S]");
        let text = format!($($arg)*);
        println!("{} {} {}", status, timestamp, text);
    };
}

#[macro_export]
macro_rules! log_bad {
    ($($arg:tt)*) => {
        let status = colored::Colorize::red("[bad!]");
        let timestamp = chrono::Local::now().format("[%H:%M:%S]");
        let text = format!($($arg)*);
        println!("{} {} {}", status, timestamp, text);
        };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        let status = colored::Colorize::blue("[info]");
        let timestamp = chrono::Local::now().format("[%H:%M:%S]");
        let text = format!($($arg)*);
        println!("{} {} {}", status, timestamp, text);
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        let status = colored::Colorize::yellow("[warn]");
        let timestamp = chrono::Local::now().format("[%H:%M:%S]");
        let text = format!($($arg)*);
        println!("{} {} {}", status, timestamp, text);
    };
}
