use crate::location::Location;
#[allow(unused_imports)]
use crate::{log_bad, log_good, log_info, log_warn};
use reqwest::Error;
use serde_derive;
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

        let data_to_request = "current_weather=true";

        // data to request
        let url = format!("{WEATHER_API}latitude={lat}&longitude={long}&{data_to_request}");

        let mut attempts = 0;

        log_info!(
            "attempting to retrive current weather for {}, {}",
            place_name,
            country_name
        );

        loop {
            // request the url, if it fails try again 3 times, if that fails return nothing
            match reqwest::blocking::get(&url) {
                Ok(response) => {
                    // deserialise response
                    let deserialised_response = response.json::<WeatherResponse>().unwrap();

                    log_good!(
                        "retrieved current weather data - data: \n {:#?}",
                        deserialised_response
                    );

                    // send response back
                    return Ok(deserialised_response);
                }
                Err(err) => {
                    attempts += 1;
                    if attempts == 3 {
                        return Err(err);
                    } else {
                        log_warn!("failed to retrive weather data, attempt {}/3", attempts);
                    }
                }
            }
        }
    })
}
