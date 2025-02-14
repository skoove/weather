use serde_derive::Deserialize;
use std::{
    thread::{self, JoinHandle},
    time::Instant,
};

#[allow(unused_imports)]
use crate::{log_bad, log_good, log_info};

#[derive(Debug, Clone)]
pub struct Location {
    pub place_name: String,
    pub country_name: String,
    pub coordinates: (f32, f32),
    pub request_time: Instant,
}

#[derive(Deserialize, Debug)]
struct LocationResponse {
    results: Vec<LocationResponseItem>,
}

#[derive(Deserialize, Debug)]
struct LocationResponseItem {
    name: String,
    country: String,
    latitude: f32,
    longitude: f32,
}

impl Default for Location {
    fn default() -> Self {
        // default location to brisbane
        // this is mostly for testing and maybe as a fall back?
        Self {
            place_name: "Brisbane".to_string(),
            country_name: "Australia".to_string(),
            coordinates: (-27.4705, 153.026),
            request_time: Instant::now(),
        }
    }
}

pub fn location_query(
    query: String,
    count: i8,
) -> JoinHandle<Result<Vec<Location>, reqwest::Error>> {
    log_info!("searching for {count} results for {query}");
    let time = Instant::now();
    thread::spawn(move || {
        let request = format!(
            "https://geocoding-api.open-meteo.com/v1/search?name={}&count={}&language=en&format=json",
            query, count
        );
        let response = reqwest::blocking::get(request)?;
        let response = response.json::<LocationResponse>()?;
        println!("{response:#?}");
        let mut locations = Vec::new();
        for location in response.results {
            let fmt_location = Location {
                place_name: location.name,
                country_name: location.country,
                coordinates: (location.latitude, location.longitude),
                request_time: time,
            };
            locations.push(fmt_location);
        }
        log_good!("got {count} location results for query {query}");
        Ok(locations)
    })
}
