use crate::location::Location;
use crate::{log_bad, log_good, log_info, log_warn};
use reqwest::Error;
use serde_derive;
use std::sync::mpsc::Sender;
use std::thread;

const WEATHER_API: &str = "https://api.open-meteo.com/v1/forecast?";

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

pub fn request_weather(location: Location, tx: Sender<Result<WeatherResponse, Error>>) {
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
                    log_good!("retrieved current weather data");

                    // deserialise response
                    let deserialised_response = response.json::<WeatherResponse>().unwrap();

                    // send response back
                    tx.send(Ok(deserialised_response))
                        .expect("expected to send to thread");
                    break;
                }
                Err(err) => {
                    attempts += 1;
                    if attempts == 3 {
                        log_bad!("failed to retrieve current weather!");
                        tx.send(Err(err)).expect("expected to send to thread");
                        break;
                    } else {
                        log_warn!("failed to retrive weather data, attempt {}/3", attempts);
                    }
                }
            }
        }
    });
}
