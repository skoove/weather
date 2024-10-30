use std::thread::{self, JoinHandle};

use crate::location::Location;
use crate::utils::*;
use reqwest::Error;
use serde_derive;
use std::sync::mpsc::{self, Receiver, Sender};

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

pub fn request_weather(location: Location, tx: Sender<WeatherResponse>) {
    thread::spawn(move || {
        // pull coords out
        let (lat, long) = location.coordinates;
        let data_to_request = "current_weather=true";

        // data to request
        let url = format!("{WEATHER_API}latitude={lat}&longitude={long}&{data_to_request}");

        let mut attempts = 0;

        log(
            format!(
                "attempting to retrive current weather for {}, {}",
                location.place_name, location.country_name
            ),
            LogStatus::Info,
        );

        loop {
            // request the url, if it fails try again 3 times, if that fails return nothing
            match reqwest::blocking::get(&url) {
                Ok(response) => {
                    log(
                        "retrieved current weather data".to_string(),
                        LogStatus::Good,
                    );

                    // deserialise response
                    let deserialised_response = response.json::<WeatherResponse>().unwrap();

                    // send response back
                    tx.send(deserialised_response);
                    break;
                }
                Err(err) => {
                    attempts += 1;
                    if attempts == 3 {
                        log(
                            format!(
                                "failed to retrive data attempt {}/3, giving up, error: {}",
                                attempts, err
                            ),
                            LogStatus::Bad,
                        );
                        break;
                    } else {
                        log(
                            format!("failed to retrive data attempt {}/3", attempts),
                            LogStatus::Warn,
                        )
                    }
                }
            }
        }
    });
}
