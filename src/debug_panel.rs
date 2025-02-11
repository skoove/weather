use crate::location::location_query;
use crate::ui::WeatherApp;
use eframe::egui::{self, Layout, Ui};

impl WeatherApp {
    pub fn debug_panel(&mut self, ui: &mut Ui) {
        ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
            if ui.button("geocoding test").clicked() {
                location_query("brisbane".to_string(), 10);
            }
            if ui.button("error test").clicked() {
                self.error("this is a test error".to_string());
            }
        });

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                ui.group(|ui| {
                    ui.heading("weather");
                    ui.label(format!("{:#?}", self.weather_data));
                });
            })
        });
    }
}
