use crate::weather::{self, request_weather};
use crate::{utils::*, weather::WeatherResponse};
use catppuccin_egui::{set_theme, MOCHA};
use eframe::{self, egui};
use std::sync::mpsc::{self, Receiver, Sender};

pub struct WeatherApp {
    weather_request_in_progress: bool,
    weather_data: Option<WeatherResponse>,
    tx: Sender<WeatherResponse>,
    rx: Receiver<WeatherResponse>,
}

impl WeatherApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        log(format!("created app"), LogStatus::Good);

        // make thread communication channels
        let (tx, rx) = mpsc::channel();
        Self {
            weather_request_in_progress: false,
            tx,
            rx,
            weather_data: None,
        }
    }
}

impl eframe::App for WeatherApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        set_theme(ctx, MOCHA);

        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("request weather").clicked() {
                log(format!("weather request button clicked"), LogStatus::Info);

                request_weather(crate::location::Location::default(), self.tx.clone());
                self.weather_request_in_progress = true;
            };

            if self.weather_request_in_progress {
                ui.spinner();
                self.weather_data = if let Ok(value) = self.rx.try_recv() {
                    self.weather_request_in_progress = false;
                    Some(value)
                } else {
                    None
                }
            }

            if let Some(weather) = &self.weather_data {
                ui.label(format!(
                    "current temperature: {0:.0}{1}",
                    weather.current_weather.temperature, weather.current_weather_units.temperature,
                ));
            }
        });
    }
}
