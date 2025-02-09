use crate::location::location_query;
use crate::location::Location;
use crate::weather::request_weather;
use crate::weather::WeatherResponse;
#[allow(unused_imports)]
use crate::{log_bad, log_good, log_info};
use colored::Colorize;
use eframe::egui::Color32;
use eframe::egui::Context;
use eframe::egui::Frame;
use eframe::egui::Layout;
use eframe::egui::Margin;
use eframe::egui::Ui;
use eframe::{self, egui};
use reqwest::Error;
use std::thread::JoinHandle;
use std::time::Duration;
use std::time::Instant;

#[derive(Debug)]
pub struct WeatherApp {
    weather_data: Option<WeatherResponse>,
    input: Input,
    weather_handle: Option<JoinHandle<Result<WeatherResponse, Error>>>,
    last_error: Option<(Instant, String)>,
}
#[derive(Debug)]
struct Input {
    location_box_contents: String,
    location_box_query: String,
}

impl WeatherApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        log_good!("created app");

        let input = Input {
            location_box_contents: "enter location here!".to_string(),
            location_box_query: "".to_string(),
        };

        Self {
            weather_data: None,
            weather_handle: None,
            input,
            last_error: None,
        }
    }

    fn error(&mut self, error: String) {
        self.last_error = Some((Instant::now(), error))
    }

    fn display_error(&mut self, ctx: &Context) {
        let Some((time, text)) = &self.last_error else {
            return;
        };
        if time.elapsed().as_secs() > 2 {
            self.last_error = None
        } else {
            let frame = Frame::default();
            let frame = frame.fill(Color32::from_rgb(200, 120, 120));
            let frame = frame.inner_margin(Margin::same(10.0));
            egui::TopBottomPanel::bottom("error")
                .frame(frame)
                .show(ctx, |ui| {
                    ui.visuals_mut().override_text_color = Some(Color32::from_rgb(0, 0, 0));
                    ui.heading(format!("{}", text));
                });
        }
    }

    fn try_recv_wdata(&mut self) {
        let Some(handle) = self.weather_handle.take_if(|h| h.is_finished()) else {
            return;
        };
        match handle.join().expect("expected to join thread!") {
            Ok(data) => self.weather_data = Some(data),
            Err(err) => {
                log_bad!("failed to retreive weather data! error:\n{}", err);
                self.error("failed to retrieve weather data!".to_string());
            }
        };
    }

    fn refresh_button(&mut self, ui: &mut Ui) {
        if ui.button("↻").clicked() {
            self.weather_handle = Some(request_weather(Location::default()));
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
}

impl eframe::App for WeatherApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // if there is no weather request in progress and there is
        // no data present, start request for default location
        if self.weather_handle.is_none() & self.weather_data.is_none() {
            self.weather_handle = Some(request_weather(Location::default()));
        }

        self.try_recv_wdata();
        self.display_error(ctx);

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
            if ui.button("error test").clicked() {
                self.error("yopu clkiekd the button".to_string());
            }
            ui.heading("everything");
            ui.label(format!("{:#?}", self));
        });
    }
}
