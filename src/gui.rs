use eframe::egui;
use egui::{Color32, Frame, Stroke};
use std::sync::{Arc, Mutex};

use crate::{ButtonStateMap, KeyStateMap};

pub fn run(
    key_states: Arc<Mutex<KeyStateMap>>,
    button_states: Arc<Mutex<ButtonStateMap>>,
    ctx_holder: Arc<Mutex<Option<egui::Context>>>,
    settings: &crate::settings::Settings,
) -> eframe::Result {
    let gui_app = GuiApp {
        key_states: key_states.clone(),
        button_states: button_states.clone(),
        ctx_holder: ctx_holder.clone(),
        settings: settings.clone(),
    };
    let window_width = settings.keys.table.iter().map(|v| v.len()).max().unwrap() as f32 * 45f32;
    let window_height = settings.keys.table.len() as f32 * 32f32;
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([window_width, window_height])
            .with_resizable(settings.window.resizable)
            .with_decorations(settings.window.decorations),
        ..Default::default()
    };
    eframe::run_native(
        "keyboard-display-utility",
        options,
        Box::new(|_cc| Ok(Box::new(gui_app))),
    )
}

struct GuiApp {
    key_states: Arc<Mutex<KeyStateMap>>,
    button_states: Arc<Mutex<ButtonStateMap>>,
    ctx_holder: Arc<Mutex<Option<egui::Context>>>,
    settings: crate::settings::Settings,
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.ctx_holder.lock().unwrap().is_none() {
            *self.ctx_holder.lock().unwrap() = Some(ctx.clone());
        }
        let key_states = self.key_states.lock().unwrap().clone();
        let button_states = self.button_states.lock().unwrap().clone();
        egui::CentralPanel::default()
            .frame(
                Frame::default().fill(Color32::from_hex(&self.settings.colors.background).unwrap()),
            )
            .show(ctx, |ui| {
                egui::Grid::new("key_grid")
                    .spacing([5.0, 5.0])
                    .show(ui, |ui| {
                        for row in &self.settings.keys.table {
                            for item in row {
                                if !item.is_empty() {
                                    let pressed: bool;
                                    if let Some(key) = crate::map::string_to_key(item) {
                                        pressed = key_states[&key];
                                    } else if let Some(button) = crate::map::string_to_mouse(item) {
                                        pressed = button_states[&button];
                                    } else {
                                        panic!("Unknown settings.toml key.table string: {item}");
                                    }
                                    let color = if pressed {
                                        Color32::from_hex(&self.settings.colors.alternate).unwrap()
                                    } else {
                                        Color32::from_hex(&self.settings.colors.primary).unwrap()
                                    };
                                    ui.with_layout(
                                        egui::Layout::top_down(egui::Align::Center),
                                        |ui| {
                                            Frame::default()
                                                .fill(color)
                                                .stroke(Stroke::new(
                                                    2.0,
                                                    Color32::from_hex(
                                                        &self.settings.colors.secondary,
                                                    )
                                                    .unwrap(),
                                                ))
                                                .inner_margin(4.0)
                                                .show(ui, |ui| {
                                                    ui.label(item);
                                                });
                                        },
                                    );
                                } else {
                                    ui.label(""); // empty grid item
                                }
                            }
                            ui.end_row();
                        }
                    });
            });
        ctx.request_repaint();
    }
}
