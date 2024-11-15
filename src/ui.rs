use crate::location::Location;
use crate::log_good;
use crate::weather::request_weather;
use crate::weather::WeatherResponse;
use catppuccin_egui::{set_theme, MOCHA};
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

        cc.egui_ctx.set_pixels_per_point(1.0);
        set_theme(&cc.egui_ctx, MOCHA);

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
        }
    }

    fn exit_button(&mut self, ui: &mut Ui) {
        if ui.button("❌").clicked() {
            panic!("closed app");
        }
    }

    fn try_recv_wdata(&mut self) {
        if let Ok(weather_data) = self.rx.try_recv() {
            self.weather_data = Some(weather_data.unwrap());
            self.weather_request_in_progress = false;
        }
    }
}

impl eframe::App for WeatherApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let s = self;

        if s.weather_request_in_progress == true {
            s.try_recv_wdata();
        }

        egui::CentralPanel::default().show(ctx, |ui| {});

        // make a top bar for some buttons
        egui::TopBottomPanel::top("top bar").show(ctx, |ui| {
            // make buttons be side by side
            ui.horizontal(|ui| {
                // first group: left aligned
                ui.with_layout(Layout::left_to_right(egui::Align::TOP), |ui| {
                    s.refresh_button(ui);
                });

                // second group: right aligned
                ui.with_layout(Layout::right_to_left(egui::Align::TOP), |ui| {
                    s.exit_button(ui);
                });
            })
        });
    }
}
