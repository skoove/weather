use crate::location::Location;
use serde_derive;

const WEATHER_API: &str = "https://api.open-meteo.com/v1/forecast?";

struct WeatherRequest {
    location: Location,
}

#[derive(serde_derive::Deserialize)]
struct WeatherResponse {
    current_weather_units: CurrentWeatherUnits,
    current_weather: CurrentWeather,
}

#[derive(serde_derive::Deserialize)]
struct CurrentWeather {
    time: String,
    interval: i16,
    temperature: f32,
    windspeed: f32,
    winddirection: i16,
    is_day: i16,
    weathercode: i16,
}

#[derive(serde_derive::Deserialize)]
struct CurrentWeatherUnits {
    time: String,
    interval: String,
    temperature: String,
    windspeed: String,
    winddirection: String,
    is_day: String,
    weathercode: String,
}

impl WeatherRequest {
    pub async fn request_weather(location: Location) -> WeatherResponse {
        let (lat, long) = location.coordinates;
        let data_to_request = "current_weather=true";

        let url = String::from(format!(
            "{WEATHER_API}latitude={lat}&longitude={long}&{data_to_request}"
        ));

        let api_response = reqwest::get(url)
            .await
            .unwrap()
            .json::<WeatherResponse>()
            .await
            .unwrap();

        api_response
    }
}
