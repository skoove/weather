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
    log("hello!", false);
    let location = Location::default();

    let weather_response = weather::request_weather(&location).await;

    println!("{}", location.name);
    println!("{}", weather_response.unwrap().current_weather.temperature);

    Ok(())
}
