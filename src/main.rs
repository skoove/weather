use reqwest::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let response = reqwest::get(
        "https://api.open-meteo.com/v1/forecast?latitude=52.52 &longitude=13.41 &current=temperature_2m,wind_speed_10m&hourly=temperature_2m,relative_humidity_2m,wind_speed_10m",
    );

    println!("fetching data");

    let response = response.await?;

    println!("data recived!\nprocessing...");

    let data = response.text().await?;

    println!("{}", data);

    Ok(())
}

struct WeatherRequest {
    location: String,
    data_to_request: Vec<String>,
}

struct OpenStreetMapRequest {}
