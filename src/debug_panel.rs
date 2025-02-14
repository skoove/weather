use crate::location::location_query;
use crate::ui::WeatherApp;
use eframe::egui::{self, Grid, Layout, Ui};

impl WeatherApp {
    pub fn debug_panel(&mut self, ui: &mut Ui) {
        ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
            if ui.button("geocoding test").clicked() {
                location_query("brisbane".to_string(), 5);
            }
            if ui.button("error test").clicked() {
                self.error("this is a test error".to_string());
            }
        });

        ui.set_width(ui.available_width());

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.label(format!("{}", ui.available_width()));
            let max_col_width = (ui.available_width() / 3.0) - 5.0;
            Grid::new("debug panels")
                .max_col_width(max_col_width)
                .show(ui, |ui| {
                    self.weather(ui);
                    self.location(ui);
                    self.whole_app(ui);
                });
        });
    }

    // debug windows
    fn weather(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("weather");
                ui.separator();
                self.weather_request_button(ui, "request weather");
                ui.separator();
                ui.label(format!("{:#?}", self.weather_data));
            });
        });
    }

    fn location(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("location");
                ui.separator();
                ui.horizontal(|ui| {
                    self.location_box(ui);
                });
                ui.separator();
                ui.label(format!("{:#?}", self.location_handle));
                ui.label(format!("{:#?}", self.location_input));
                ui.label(format!("{:#?}", self.locations));
            });
        });
    }

    fn whole_app(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading("everything");
                ui.separator();
                ui.label(format!("{:#?}", self));
            });
        });
    }

    // debug buttons
}
