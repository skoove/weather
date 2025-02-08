use std::{sync::mpsc::Sender, thread, time::Instant};

#[allow(unused_imports)]
use crate::{log_bad, log_good, log_info};

#[derive(Debug)]
pub struct Location {
    pub place_name: String,
    pub country_name: String,
    pub coordinates: (f32, f32),
    pub request_time: Instant,
}

pub struct LocationResponse {
    pub locations: Box<[Location]>,
    pub request_time: Instant,
}

impl Default for Location {
    fn default() -> Self {
        // default location to brisbane
        // this is mostly for testing and maybe as a fall back?
        Self {
            place_name: "Brisbane".to_string(),
            country_name: "Australia".to_string(),
            coordinates: (-27.4705, 153.0260),
            request_time: Instant::now(),
        }
    }
}

pub fn location_query(query: String, count: i8 /* , tx: Sender<LocationResponse> */) {
    thread::spawn(move || {
        let request = format!(
            "https://geocoding-api.open-meteo.com/v1/search?name={}&count={}&language=en&format=json",
            query, count
        );
        let response = reqwest::blocking::get(request).unwrap().text().unwrap();
        log_info!("\n{}", response);
    });
}
