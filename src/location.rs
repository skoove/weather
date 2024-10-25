use std::str::FromStr;

pub struct Location {
    pub name: String,
    pub coordinates: (f32, f32),
}

impl Default for Location {
    fn default() -> Self {
        Self {
            name: String::from_str("Brisbane").expect("managed to mess that up somehow"),
            coordinates: (-27.4705, 153.0260),
        }
    }
}
