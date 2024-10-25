mod location;
mod ui;
mod utils;
mod weather;

use location::*;
use reqwest::Error;
use tokio;
use utils::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let location = Location::default();

    log("this is a good thing to happen, yeah", false);
    log("this is NOT good", true);

    let weather_response = weather::request_weather(&location).await;

    println!("{}", location.name);
    println!("{}", weather_response.current_weather.temperature);

    Ok(())
}
