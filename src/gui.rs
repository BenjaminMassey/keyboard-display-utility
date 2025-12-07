use eframe::egui;
use egui::{Color32, Frame, Stroke};
use inputbot::KeybdKey;
use std::sync::{Arc, Mutex};

use crate::KeyStateMap;

pub fn run(
    key_states: Arc<Mutex<KeyStateMap>>,
    ctx_holder: Arc<Mutex<Option<egui::Context>>>,
    chosen_keys: &[KeybdKey],
    settings: &crate::settings::Settings,
) -> eframe::Result {
    let gui_app = GuiApp {
        key_states: key_states.clone(),
        ctx_holder: ctx_holder.clone(),
        chosen_keys: chosen_keys.to_vec(),
        settings: settings.clone(),
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([settings.window.width, settings.window.height])
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
    ctx_holder: Arc<Mutex<Option<egui::Context>>>,
    chosen_keys: Vec<KeybdKey>,
    settings: crate::settings::Settings,
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.ctx_holder.lock().unwrap().is_none() {
            *self.ctx_holder.lock().unwrap() = Some(ctx.clone());
        }
        let states = self.key_states.lock().unwrap().clone();
        egui::CentralPanel::default()
            .frame(
                Frame::default().fill(Color32::from_hex(&self.settings.colors.background).unwrap()),
            )
            .show(ctx, |ui| {
                egui::Grid::new("key_grid")
                    .num_columns(self.settings.keys.table.len())
                    .spacing([5.0, 5.0])
                    .show(ui, |ui| {
                        for row in &self.settings.keys.table {
                            for item in row {
                                if !item.is_empty() {
                                    let key = crate::map::string_to_key(&item);
                                    let color = if states[&key] {
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
                                                    ui.label(
                                                        egui::RichText::new(format!("{item}",))
                                                            .size(24.0),
                                                    );
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
