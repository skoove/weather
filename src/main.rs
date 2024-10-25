mod location;
mod ui;
mod weather;

use location::Location;
use reqwest::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let location = Location::default();

    let weather_response = weather::request_weather(&location).await;

    println!("{}", location.name);
    println!("{}", weather_response.current_weather.temperature);

    Ok(())
}
