use crate::utils::*;
use eframe::{self, egui};

#[derive(Default)]
pub struct WeatherApp {}

impl WeatherApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        log(format!("created app"), LogStatus::Good);
        Self::default()
    }
}

impl eframe::App for WeatherApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| ui.label("test"));
    }
}
