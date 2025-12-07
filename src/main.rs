use eframe::egui;
use inputbot::KeybdKey;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod gui;
mod keys;
mod map;
mod settings;

type KeyStateMap = HashMap<KeybdKey, bool>; // key => down/up

fn main() {
    let settings = settings::get_settings();

    let mut chosen_keys: Vec<KeybdKey> = Vec::new();
    for row in &settings.keys.table {
        for item in row {
            chosen_keys.push(map::string_to_key(item));
        }
    }

    let mut states: KeyStateMap = HashMap::new();
    for key in &chosen_keys {
        states.insert(key.clone(), false);
    }

    let key_states: Arc<Mutex<KeyStateMap>> = Arc::new(Mutex::new(states));
    let ctx_holder: Arc<Mutex<Option<egui::Context>>> = Arc::new(Mutex::new(None));

    let key_for_gui = key_states.clone();
    let ctx_for_keys = ctx_holder.clone();
    let chosen_keys_copy = chosen_keys.clone();
    let _ = std::thread::spawn(move || {
        keys::run(key_for_gui.clone(), ctx_for_keys, &chosen_keys_copy);
    });

    let _ = gui::run(
        key_states.clone(),
        ctx_holder.clone(),
        &chosen_keys,
        &settings,
    );
}
