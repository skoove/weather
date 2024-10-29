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
                    Some(value).expect("expected to be able to recive weather data")
                } else {
                    None
                }
            }

            if self.weather_data.is_some() {
                ui.heading(format!("{}", weather_data.CurrentWeather.temperature_2m);
            }
        });
    }
}
