use crate::location::Location;
use crate::log_bad;
use crate::log_good;
use crate::log_info;
use crate::weather::request_weather;
use crate::weather::WeatherResponse;
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
        // check if data is there
        match self.rx.try_recv() {
            Ok(data) => {
                self.weather_data = {
                    log_info!("{:#?}", data);
                    self.weather_request_in_progress = false;
                    Some(data.unwrap())
                }
            }
            Err(_) => (),
        }
    }
}

impl eframe::App for WeatherApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
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
