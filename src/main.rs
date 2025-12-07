use eframe::egui;
use inputbot::{KeybdKey, MouseButton};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod gui;
mod keys;
mod map;
mod settings;

type KeyStateMap = HashMap<KeybdKey, bool>; // key => down/up
type ButtonStateMap = HashMap<MouseButton, bool>; // mouse version

fn main() {
    let settings = settings::get_settings();

    let mut chosen_keys: Vec<KeybdKey> = Vec::new();
    let mut chosen_buttons: Vec<MouseButton> = Vec::new();
    for row in &settings.keys.table {
        for item in row {
            if item.is_empty() {
                continue;
            } // used for empty space
            if let Some(chosen_key) = map::string_to_key(item) {
                chosen_keys.push(chosen_key);
            } else if let Some(chosen_button) = map::string_to_mouse(item) {
                chosen_buttons.push(chosen_button);
            } else {
                panic!("Unknown settings.toml key.table string: {item}");
            }
        }
    }

    let mut key_states: KeyStateMap = HashMap::new();
    for key in &chosen_keys {
        key_states.insert(key.clone(), false);
    }

    let mut button_states: ButtonStateMap = HashMap::new();
    for button in &chosen_buttons {
        button_states.insert(button.clone(), false);
    }

    let key_states: Arc<Mutex<KeyStateMap>> = Arc::new(Mutex::new(key_states));
    let button_states: Arc<Mutex<ButtonStateMap>> = Arc::new(Mutex::new(button_states));
    let ctx_holder: Arc<Mutex<Option<egui::Context>>> = Arc::new(Mutex::new(None));

    let keys_for_gui = key_states.clone();
    let buttons_for_gui = button_states.clone();
    let ctx_for_keys = ctx_holder.clone();
    let chosen_keys_copy = chosen_keys.clone();
    let chosen_buttons_copy = chosen_buttons.clone();
    let _ = std::thread::spawn(move || {
        keys::run(
            keys_for_gui.clone(),
            buttons_for_gui.clone(),
            ctx_for_keys.clone(),
            &chosen_keys_copy,
            &chosen_buttons_copy,
        );
    });

    let _ = gui::run(
        key_states.clone(),
        button_states.clone(),
        ctx_holder.clone(),
        &settings,
    );
}
