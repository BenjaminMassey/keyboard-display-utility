#[derive(serde::Deserialize, Clone)]
pub struct Window {
    pub width: f32,  // TODO: should be dynamic from keys.table row length
    pub height: f32, // TODO: should be dynamic from keys.table column length
    pub decorations: bool,
    pub resizable: bool,
}

#[derive(serde::Deserialize, Clone)]
pub struct Colors {
    pub background: String,
    pub primary: String,
    pub secondary: String,
    pub alternate: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct Keys {
    pub table: Vec<Vec<String>>,
}

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub window: Window,
    pub colors: Colors,
    pub keys: Keys,
}

pub fn get_settings() -> Settings {
    toml::from_str(&std::fs::read_to_string("./settings.toml").unwrap()).unwrap()
}
