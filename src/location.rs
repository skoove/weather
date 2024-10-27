pub struct Location {
    pub name: String,
    pub coordinates: (f32, f32),
}

impl Default for Location {
    fn default() -> Self {
        Self {
            name: "Brisbane".to_string(),
            coordinates: (-27.4705, 153.0260),
        }
    }
}
