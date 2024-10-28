mod location;
mod ui;
mod utils;
mod weather;

use location::Location;
use utils::*;
use weather::request_weather;

fn main() {
    let location = Location::default();
    log(format!("hello"), LogStatus::Info);

    let weather = request_weather(location);
}
