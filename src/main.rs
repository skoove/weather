mod location;
mod ui;
mod utils;
mod weather;
use eframe::NativeOptions;
use ui::WeatherApp;

fn main() {
    log_info!("hello!");

    // default run options
    let native_options = NativeOptions::default();
    eframe::run_native(
        "weather",
        native_options,
        Box::new(|cc| Ok(Box::new(WeatherApp::new(&cc)))),
    )
    .unwrap();
}
