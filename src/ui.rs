use crate::location::Location;
use crate::weather::request_weather;
use crate::weather::WeatherResponse;
#[allow(unused_imports)]
use crate::{log_bad, log_good, log_info};
use eframe::egui::Layout;
use eframe::egui::Ui;
use eframe::{self, egui};
use reqwest::Error;
use std::sync::mpsc::{self, Receiver, Sender};

pub struct WeatherApp {
    weather_request_in_progress: bool,
    weather_data: Option<WeatherResponse>,
    tx: Sender<Result<WeatherResponse, Error>>,
    rx: Receiver<Result<WeatherResponse, Error>>,
}

impl WeatherApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        log_good!("created app");

        // make thread communication channels
        let (tx, rx) = mpsc::channel();

        Self {
            weather_request_in_progress: false,
            tx,
            rx,
            weather_data: None,
        }
    }

    fn refresh_button(&mut self, ui: &mut Ui) {
        if ui.button("↻").clicked() {
            request_weather(Location::default(), self.tx.clone());
            self.weather_request_in_progress = true;
        }
    }

    fn try_recv_wdata(&mut self) {
        // check if data from thread, dont care about if thread returns err because if it does there is no data
        if let Ok(data) = self.rx.try_recv() {
            // we do care about error from reqwest tho
            match data {
                Ok(data) => {
                    self.weather_data = Some(data);
                    self.weather_request_in_progress = false;
                }
                Err(e) => {
                    log_bad!("failed to retrive weather data! error: \n{}", e);
                }
            }
        }
    }

    fn main_content(&mut self) {}
}

impl eframe::App for WeatherApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // if there is no weather request in progress and there is no data present, start request for default location

        // if there is a request in progress, check for data then write data down
        if self.weather_request_in_progress == true {
            self.try_recv_wdata();
        }

        // make a top bar for some buttons
        egui::TopBottomPanel::top("top bar")
            .min_height(30.0)
            .show(ctx, |ui| {
                // make buttons be side by side
                ui.horizontal_centered(|ui| {
                    // first group: left aligned
                    ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
                        self.refresh_button(ui);
                    });

                    // second group: right aligned
                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {});
                })
            });
        // centeral panel for main content
        egui::CentralPanel::default().show(ctx, |ui| {});
    }
}
