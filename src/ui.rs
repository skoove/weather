use crate::log_good;
use crate::weather::request_weather;
use crate::weather::WeatherResponse;
use catppuccin_egui::{set_theme, MOCHA};
use eframe::egui::Layout;
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
}

impl eframe::App for WeatherApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // to make things not take forever
        let s = self;

        egui::CentralPanel::default().show(ctx, |ui| {});

        // make a top bar for some buttons
        egui::TopBottomPanel::top("top bar").show(ctx, |ui| {
            // make buttons be side by side
            ui.horizontal(|ui| {
                // first group: left aligned
                ui.with_layout(Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.button("refresh");
                    ui.button("theme");
                });

                // second group: right aligned
                ui.with_layout(Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui.button("exit");
                });
            })
        });
    }
}
