use eframe::egui;
use inputbot::{KeybdKey, MouseButton};
use std::sync::{Arc, Mutex};
use strum::IntoEnumIterator;

use crate::{ButtonStateMap, KeyStateMap};

pub fn run(
    key_states: Arc<Mutex<KeyStateMap>>,
    button_states: Arc<Mutex<ButtonStateMap>>,
    ctx_holder: Arc<Mutex<Option<egui::Context>>>,
    chosen_keys: &[KeybdKey],
    chosen_buttons: &[MouseButton],
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

    for button in MouseButton::iter() {
        if !chosen_buttons.contains(&button) {
            continue;
        }
        let button_states = button_states.clone();
        let ctx_holder = ctx_holder.clone();
        button.bind(move || {
            click(button_states.clone(), &button, ctx_holder.clone());
        });
    }

    inputbot::handle_input_events();
}

fn press(
    key_states: Arc<Mutex<KeyStateMap>>,
    key: &KeybdKey,
    ctx_holder: Arc<Mutex<Option<egui::Context>>>,
) {
    while key.is_pressed() {
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
// TODO: these functions too similar
fn click(
    button_states: Arc<Mutex<ButtonStateMap>>,
    button: &MouseButton,
    ctx_holder: Arc<Mutex<Option<egui::Context>>>,
) {
    while button.is_pressed() {
        let mut button_states = button_states.lock().unwrap();
        if button_states.contains_key(button) {
            *button_states.get_mut(button).unwrap() = true;
        } else {
            button_states.insert(button.clone(), true);
        }
    }

    let mut button_states = button_states.lock().unwrap();
    if button_states.contains_key(button) {
        *button_states.get_mut(button).unwrap() = false;
    } else {
        // TODO: shouldn't be possible AFAIK
        button_states.insert(button.clone(), false);
    }

    if let Some(ctx) = ctx_holder.lock().unwrap().as_ref() {
        ctx.request_repaint();
    }
}
