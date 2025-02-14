use crate::location::location_query;
use crate::location::Location;
use crate::weather::request_weather;
use crate::weather::WeatherResponse;
#[allow(unused_imports)]
use crate::{log_bad, log_good, log_info};
use catppuccin_egui::{set_theme, Theme, LATTE, MOCHA};
use eframe::egui::{self, Color32, Context, Frame, Layout, Margin, Ui};
use egui_autocomplete::AutoCompleteTextEdit;
use reqwest::Error;
use std::env;
use std::thread::JoinHandle;
use std::time::Instant;

#[derive(Debug)]
pub struct WeatherApp {
    pub weather_data: Option<WeatherResponse>,
    pub location_input: LocationInput,
    weather_handle: Option<JoinHandle<Result<WeatherResponse, Error>>>,
    pub location_handle: Option<JoinHandle<Result<Vec<Location>, Error>>>,
    last_error: Option<(Instant, String)>,
    debug_mode: bool,
    pub locations: Vec<Location>,
    theme: Theme,
}
#[derive(Debug)]
pub struct LocationInput {
    pub location_box_contents: String,
    location_box_inputs: Vec<String>,
}

/// checks if debug flag passed (-d or --debug)
fn parse_arguments() -> bool {
    let arguments: Vec<String> = env::args().collect();
    log_info!("arguments: {:?}", &arguments);
    if arguments.iter().any(|e| e == "-d" || e == "--debug") {
        log_info!("debug mode enabled!");
        true
    } else {
        false
    }
}

impl WeatherApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        log_good!("created app");
        let ctx = &cc.egui_ctx;

        let start_theme: Theme = MOCHA;

        set_theme(ctx, start_theme);

        ctx.style_mut(|theme| {
            theme.visuals.widgets.inactive.bg_stroke.color = Color32::TRANSPARENT
        });

        let input = LocationInput {
            location_box_contents: "enter location here!".to_string(),
            location_box_inputs: vec!["iasdfhusdf".to_string()],
        };

        Self {
            weather_data: None,
            weather_handle: None,
            location_handle: None,
            location_input: input,
            locations: Vec::new(),
            last_error: None,
            debug_mode: parse_arguments(),
            theme: start_theme,
        }
    }

    pub fn error(&mut self, error: String) {
        self.last_error = Some((Instant::now(), error))
    }

    fn locations_to_strings(&mut self) {
        let mut strings: Vec<String> = Vec::new();
        for location in self.locations.clone() {
            let string = format!("{}, {}", location.place_name, location.country_name);
            strings.push(string);
        }
        self.location_input.location_box_inputs = strings;
    }

    fn display_error(&mut self, ctx: &Context) {
        let Some((time, text)) = &self.last_error else {
            return;
        };
        if time.elapsed().as_secs() > 2 {
            self.last_error = None
        } else {
            let frame = Frame::default();
            let frame = frame.fill(self.theme.red);
            let frame = frame.inner_margin(Margin::same(10.0));
            egui::TopBottomPanel::bottom("error")
                .frame(frame)
                .show(ctx, |ui| {
                    ui.visuals_mut().override_text_color = Some(self.theme.base);
                    ui.heading(text);
                });
        }
    }

    pub fn request_weather_gui(&mut self) {
        self.weather_handle = Some(request_weather(self.locations[0].clone()));
    }

    fn try_recv_wdata(&mut self) {
        let Some(handle) = self.weather_handle.take_if(|h| h.is_finished()) else {
            return;
        };
        match handle.join().expect("expected to join thread!") {
            Ok(data) => self.weather_data = Some(data),
            Err(err) => {
                log_bad!("failed to retreive weather data! error:{}", err);
                self.error("failed to retrieve weather data!".to_string());
            }
        };
    }

    fn try_recv_location(&mut self) {
        let Some(handle) = self.location_handle.take_if(|h| h.is_finished()) else {
            return;
        };
        match handle.join().expect("expected to join thread!") {
            Ok(data) => self.locations = data,
            Err(err) => {
                log_bad!("failed to retreive location! error: {}", err);
                self.error("failed to retrieve location!".to_string());
            }
        };
    }

    pub fn weather_request_button(&mut self, ui: &mut Ui, text: &str) {
        if ui.button(text).clicked() {
            self.request_weather_gui();
        }
    }

    fn debug_button(&mut self, ui: &mut Ui) {
        let button = ui.button("🐞");
        if self.debug_mode {
            button.clone().highlight();
        }
        if button.clicked() {
            self.debug_mode = !self.debug_mode;
        }
    }

    fn set_theme(&self, ctx: &Context) {
        catppuccin_egui::set_theme(ctx, self.theme);
        ctx.style_mut(|theme| {
            theme.visuals.widgets.inactive.bg_stroke.color = Color32::TRANSPARENT
        });
    }

    fn toggle_theme(&mut self, ctx: &Context) {
        if self.theme == MOCHA {
            self.theme = LATTE;
            log_info!("toggled theme to latte");
        } else if self.theme == LATTE {
            self.theme = MOCHA;
            log_info!("toggled theme to mocha");
        }
        self.set_theme(ctx);
    }

    fn theme_button(&mut self, ui: &mut Ui, ctx: &Context) {
        let symbol = match self.theme {
            MOCHA => "☀",
            LATTE => "🌙",
            _ => "uh oh",
        };
        let button = ui.button(symbol);
        if button.clicked() {
            self.toggle_theme(ctx);
        }
    }

    pub fn location_box(&mut self, ui: &mut Ui) {
        let text_box = ui.add(
            AutoCompleteTextEdit::new(
                &mut self.location_input.location_box_contents,
                &self.location_input.location_box_inputs,
            )
            .highlight_matches(true),
        );

        if text_box.has_focus() {
            if self.location_handle.is_none() {
                self.location_handle = Some(location_query(
                    self.location_input.location_box_contents.clone(),
                    5,
                ));
            }
            self.locations_to_strings();
        }

        if text_box.lost_focus() {
            log_info!("location box lost focus");
            dbg!(&self.locations[0]);
            request_weather(self.locations[0].clone());
        }
    }
}

impl eframe::App for WeatherApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // if there is no weather request in progress and there is
        // no data present, start request for default location
        // if self.weather_handle.is_none() & self.weather_data.is_none() {
        //     self.weather_handle = Some(request_weather(Location::default()));
        // }

        self.try_recv_wdata();
        self.try_recv_location();
        self.display_error(ctx);

        // make a top bar for some buttons
        egui::TopBottomPanel::top("top bar")
            .min_height(30.0)
            .show(ctx, |ui| {
                // make buttons be side by side
                ui.horizontal_centered(|ui| {
                    // first group: left aligned
                    ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
                        self.weather_request_button(ui, "⟳");
                    });

                    // second group: right aligned
                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        self.theme_button(ui, ctx);
                        self.debug_button(ui);
                    });
                })
            });
        // centeral panel for main content
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("ujisdfgihu").clicked() {
                self.location_handle = Some(location_query("poop".to_string(), 5));
            };
            if self.debug_mode {
                self.debug_panel(ui);
            } else {
                self.location_box(ui);
            }
        });
    }
}
