use eframe::egui;
use inputbot::KeybdKey;
use std::sync::{Arc, Mutex};
use strum::IntoEnumIterator;

use crate::KeyStateMap;

pub fn run(
    key_states: Arc<Mutex<KeyStateMap>>,
    ctx_holder: Arc<Mutex<Option<egui::Context>>>,
    chosen_keys: &[KeybdKey],
) {
    for key in KeybdKey::iter() {
        if !chosen_keys.contains(&key) {
            continue;
        }
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
