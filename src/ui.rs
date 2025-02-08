use crate::location::location_query;
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
    input: Input,
    tx: Sender<Result<WeatherResponse, Error>>,
    rx: Receiver<Result<WeatherResponse, Error>>,
}
#[derive(Debug)]
struct Input {
    location_box_contents: String,
    location_box_query: String,
}

impl WeatherApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        log_good!("created app");

        // make thread communication channels
        let (tx, rx) = mpsc::channel();
        let input = Input {
            location_box_contents: "enter location here!".to_string(),
            location_box_query: "".to_string(),
        };

        Self {
            weather_request_in_progress: false,
            weather_data: None,
            input,
            tx,
            rx,
        }
    }

    fn refresh_button(&mut self, ui: &mut Ui) {
        if ui.button("↻").clicked() {
            request_weather(Location::default(), self.tx.clone());
            self.weather_request_in_progress = true;
        }
    }

    fn location_box(&mut self, ui: &mut Ui) {
        let text_box = ui.text_edit_singleline(&mut self.input.location_box_contents);

        // if box is unfocused and there is weather data avalible, then display the location
        // tied to that data
        if !text_box.has_focus() {
            match &self.weather_data {
                Some(data) => {
                    let place = &data.location.place_name;
                    let country = &data.location.country_name;
                    self.input.location_box_contents = format!("{place}, {country}")
                }
                None => {
                    self.input.location_box_contents = "enter location here!".to_string();
                }
            }
        }

        // when boxes data changes change query and send request (todo), when the box looses focus
        // keep query the same
        if text_box.has_focus() {
            self.input.location_box_query = self.input.location_box_contents.clone();
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
}

impl eframe::App for WeatherApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // if there is no weather request in progress and there is
        // no data present, start request for default location
        if self.weather_request_in_progress == false && self.weather_data.is_none() {
            request_weather(Location::default(), self.tx.clone());
            self.weather_request_in_progress = true;
        }

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
        egui::CentralPanel::default().show(ctx, |ui| {
            self.location_box(ui);

            // debug stuff
            if ui.button("call terst function!!!").clicked() {
                location_query("boob".to_string(), 10);
            }
            ui.heading("input");
            ui.label(format!("{:#?}", self.input));
            ui.heading("weather data");
            ui.label(format!("{:#?}", self.weather_data));
        });
    }
}
