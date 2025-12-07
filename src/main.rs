use eframe::egui;
use inputbot::KeybdKey;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod gui;
mod keys;
mod map;

type KeyStateMap = HashMap<KeybdKey, bool>; // key => down/up

fn main() {
    let chosen_keys: Vec<KeybdKey> = std::fs::read_to_string("keys.txt")
        .unwrap()
        .trim()
        .split(" ")
        .map(|s| map::string_to_key(s))
        .collect();

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

    let _ = gui::run(key_states.clone(), ctx_holder.clone(), &chosen_keys);
}
