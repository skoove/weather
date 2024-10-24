use crate::location::Location;

struct WeatherRequest {
    location: Location,
    arguments: Vec<String>,
}

struct WeatherResponse {
    location: Location,
    temperature: f32,
}

impl WeatherRequest {
    pub async fn request_weather(location: Location) -> WeatherResponse {
        todo!()
    }
}
