use eframe::egui;
use egui::{Color32, Frame, Stroke};
use inputbot::KeybdKey;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use strum::IntoEnumIterator;
mod map;

type KeyStateMap = HashMap<KeybdKey, bool>; // key => down/up

fn main() {
    let key_states: Arc<Mutex<KeyStateMap>> = Arc::new(Mutex::new(HashMap::new()));
    let ctx_holder: Arc<Mutex<Option<egui::Context>>> = Arc::new(Mutex::new(None));

    let key_for_gui = key_states.clone();
    let ctx_for_keys = ctx_holder.clone();
    let _ = std::thread::spawn(move || {
        key_presses(key_for_gui.clone(), ctx_for_keys);
    });

    let _ = gui_window(key_states.clone(), ctx_holder.clone());
}

fn key_presses(key_states: Arc<Mutex<KeyStateMap>>, ctx_holder: Arc<Mutex<Option<egui::Context>>>) {
    for key in KeybdKey::iter() {
        let key_states = key_states.clone();
        let ctx_holder = ctx_holder.clone();
        key.bind(move || {
            press(key_states.clone(), &key, ctx_holder.clone());
        });
    }

    inputbot::handle_input_events();
}

fn press(
    key_states: Arc<Mutex<KeyStateMap>>,
    key: &KeybdKey,
    ctx_holder: Arc<Mutex<Option<egui::Context>>>,
) {
    while key.is_pressed() || key.is_toggled() {
        let mut key_states = key_states.lock().unwrap();
        if key_states.contains_key(key) {
            *key_states.get_mut(key).unwrap() = true;
        } else {
            key_states.insert(key.clone(), true);
        }
    }

    let mut key_states = key_states.lock().unwrap();
    if key_states.contains_key(key) {
        *key_states.get_mut(key).unwrap() = false;
    } else {
        // TODO: shouldn't be possible AFAIK
        key_states.insert(key.clone(), false);
    }

    if let Some(ctx) = ctx_holder.lock().unwrap().as_ref() {
        ctx.request_repaint();
    }
}

fn gui_window(
    key_states: Arc<Mutex<KeyStateMap>>,
    ctx_holder: Arc<Mutex<Option<egui::Context>>>,
) -> eframe::Result {
    let gui_app = GuiApp {
        key_states: key_states.clone(),
        ctx_holder: ctx_holder.clone(),
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([45.0, 1000.0])
            .with_resizable(false)
            .with_decorations(false),
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
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.ctx_holder.lock().unwrap().is_none() {
            *self.ctx_holder.lock().unwrap() = Some(ctx.clone());
        }
        let states = self.key_states.lock().unwrap().clone();
        egui::CentralPanel::default()
            .frame(Frame::default().fill(Color32::GREEN))
            .show(ctx, |ui| {
                for (key, &value) in states.iter() {
                    let color = if value {
                        Color32::WHITE
                    } else {
                        Color32::DARK_GRAY
                    };
                    Frame::default()
                        .fill(color)
                        .stroke(Stroke::new(2.0, Color32::LIGHT_GRAY))
                        .inner_margin(4.0)
                        .show(ui, |ui| {
                            ui.label(
                                egui::RichText::new(format!("{}", map::key_to_string(key)))
                                    .size(24.0),
                            );
                        });
                }
            });
        ctx.request_repaint();
        // TODO: shouldn't be necessary because of press(..) call, but in case for now
    }
}
