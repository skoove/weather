pub struct Location {
    pub place_name: String,
    pub country_name: String,
    pub coordinates: (f32, f32),
}

impl Default for Location {
    fn default() -> Self {
        // default location to brisbane
        // this is mostly for testing and maybe as a fall back?
        Self {
            place_name: "Brisbane".to_string(),
            country_name: "Australia".to_string(),
            coordinates: (-27.4705, 153.0260),
        }
    }
}
