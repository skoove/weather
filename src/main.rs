mod location;
mod ui;
mod utils;
mod weather;

use eframe::NativeOptions;
use location::Location;
use ui::WeatherApp;
use utils::*;
use weather::request_weather;

fn main() {
    log(format!("hello world"), LogStatus::Info);

    let native_options = NativeOptions::default();
    eframe::run_native(
        "weather",
        native_options,
        Box::new(|cc| Ok(Box::new(WeatherApp::new(&cc)))),
    );
}
