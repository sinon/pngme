#![warn(missing_docs)]
use std::path::PathBuf;

use eframe::egui;
use pngme_lib::{decode, encode, remove};
fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Pngme: Hide secret messages in PNG files",
        options,
        Box::new(|_cc| Ok(Box::<PngmeApp>::default())),
    )
}

struct PngmeApp {
    picked_path: Option<String>,
    secret_message: String,
    decoded_message: Option<String>,
    error_message: Option<String>,
}

impl Default for PngmeApp {
    fn default() -> Self {
        Self {
            picked_path: None,
            secret_message: "".to_owned(),
            decoded_message: None,
            error_message: None,
        }
    }
}

impl eframe::App for PngmeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Select png file to encode, decode, or remove a message from it.");
            ui.horizontal(|ui| {
                if ui.button("Open file…").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.picked_path = Some(path.display().to_string());
                    }
                }
                if self.picked_path.is_some() && ui.button("Clear chosen file").clicked() {
                    self.picked_path = None;
                }
            });
            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
                ui.label("Options:");
                ui.horizontal(|ui| {
                    if ui.button("Decode").clicked() {
                        let path = PathBuf::from(format!("{}", picked_path));
                        self.decoded_message = Some(decode(path, "ruSt".to_string()).unwrap());
                    }

                    if ui.button("Remove").clicked() {
                        let path = PathBuf::from(format!("{}", self.picked_path.as_ref().unwrap()));
                        let result = remove(path, "ruSt".to_string());
                        if let Err(e) = result {
                            self.error_message = Some(format!("Error: {}", e));
                        }
                        self.decoded_message = None;
                    }
                });
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.secret_message);
                    if ui.button("Encode").clicked() {
                        let path = PathBuf::from(format!("{}", self.picked_path.as_ref().unwrap()));
                        encode(path, "ruSt".to_string(), self.secret_message.to_string()).unwrap();
                    }
                });
            }

            if let Some(error_message) = &self.error_message {
                ui.label("Error:");
                ui.monospace(error_message);
            }

            if let Some(decoded_msg) = &self.decoded_message {
                ui.label("Decoded message:");
                ui.monospace(decoded_msg);
            }
        });
    }
}
