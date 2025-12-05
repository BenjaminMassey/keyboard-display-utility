use eframe::egui;
use inputbot::KeybdKey;
use std::sync::{Arc, Mutex};
use strum::IntoEnumIterator;

mod map;

const PRINT: bool = false;

fn main() {
    let current_key = Arc::new(Mutex::new(String::new()));
    let ctx_holder: Arc<Mutex<Option<egui::Context>>> = Arc::new(Mutex::new(None));

    let key_for_gui = current_key.clone();
    let ctx_for_keys = ctx_holder.clone();
    let _ = std::thread::spawn(move || {
        key_presses(key_for_gui.clone(), ctx_for_keys);
    });

    let _ = gui_window(current_key.clone(), ctx_holder.clone());
}

fn key_presses(current_key: Arc<Mutex<String>>, ctx_holder: Arc<Mutex<Option<egui::Context>>>) {
    for key in KeybdKey::iter() {
        let current_key = current_key.clone();
        let ctx_holder = ctx_holder.clone();
        key.bind(move || {
            press(current_key.clone(), &key, ctx_holder.clone());
        });
    }

    inputbot::handle_input_events();
}

fn press(
    current_key: Arc<Mutex<String>>,
    key: &KeybdKey,
    ctx_holder: Arc<Mutex<Option<egui::Context>>>,
) {
    let key_string = map::key_to_string(key);
    if PRINT {
        println!("pressed \"{}\"", key_string);
    }
    {
        let mut current_key = current_key.lock().unwrap();
        *current_key = key_string.to_owned();
    }

    // Request repaint when key is pressed
    if let Some(ctx) = ctx_holder.lock().unwrap().as_ref() {
        ctx.request_repaint();
    }
}

fn gui_window(
    current_key: Arc<Mutex<String>>,
    ctx_holder: Arc<Mutex<Option<egui::Context>>>,
) -> eframe::Result {
    let gui_app = GuiApp {
        current_key: current_key.clone(),
        ctx_holder: ctx_holder.clone(),
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([100.0, 100.0])
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
    current_key: Arc<Mutex<String>>,
    ctx_holder: Arc<Mutex<Option<egui::Context>>>,
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.ctx_holder.lock().unwrap().is_none() {
            *self.ctx_holder.lock().unwrap() = Some(ctx.clone());
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.allocate_ui_with_layout(
                ui.available_size(),
                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| {
                    ui.label(
                        egui::RichText::new(format!("{}", self.current_key.lock().unwrap()))
                            .size(48.0),
                    );
                },
            );
        });
    }
}
