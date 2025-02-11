use crate::location::Location;
#[allow(unused_imports)]
use crate::{log_bad, log_good, log_info, log_warn};
use reqwest::Error;
use std::thread::{self, JoinHandle};

const WEATHER_API: &str = "https://api.open-meteo.com/v1/forecast?";

#[derive(serde_derive::Deserialize, Debug)]
#[allow(dead_code)]
pub struct WeatherResponse {
    pub current_weather_units: CurrentWeatherUnits,
    pub current_weather: CurrentWeather,
    #[serde(skip_deserializing)]
    pub location: Location,
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

pub fn request_weather(location: Location) -> JoinHandle<Result<WeatherResponse, Error>> {
    thread::spawn(move || {
        // pull coords out
        let (lat, long) = location.coordinates;
        let Location {
            place_name,
            country_name,
            ..
        } = location;

        let url = format!("{WEATHER_API}latitude={lat}&longitude={long}&current_weather=true");

        log_info!(
            "attempting to retrive current weather for {}, {}",
            place_name,
            country_name
        );
        let response = reqwest::blocking::get(&url)?;
        let deserialised_response = response.json::<WeatherResponse>()?;
        log_good!(
            "retrieved current weather data - data: \n {:#?}",
            deserialised_response
        );
        Ok(deserialised_response)
    })
}
