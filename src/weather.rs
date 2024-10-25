use crate::location::Location;
use serde_derive;

const WEATHER_API: &str = "https://api.open-meteo.com/v1/forecast?";

pub struct WeatherRequest {
    location: Location,
}

#[derive(serde_derive::Deserialize, Debug)]
#[allow(dead_code)]
pub struct WeatherResponse {
    pub current_weather_units: CurrentWeatherUnits,
    pub current_weather: CurrentWeather,
}

#[derive(serde_derive::Deserialize, Debug)]
#[allow(dead_code)]
pub struct CurrentWeather {
    pub time: String,
    pub interval: i16,
    pub temperature: f32,
    pub windspeed: f32,
    pub winddirection: i16,
    pub is_day: i16,
    pub weathercode: i16,
}

#[derive(serde_derive::Deserialize, Debug)]
#[allow(dead_code)]
pub struct CurrentWeatherUnits {
    pub time: String,
    pub interval: String,
    pub temperature: String,
    pub windspeed: String,
    pub winddirection: String,
    pub is_day: String,
    pub weathercode: String,
}

impl WeatherRequest {
    pub async fn request_weather(location: &Location) -> WeatherResponse {
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
