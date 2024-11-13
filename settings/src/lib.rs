use serde_json::{Map, Value};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use winit::{window::Window,
            dpi::PhysicalSize
};
use winit::window::Fullscreen;

pub struct Settings {
    pub name: String
}

impl Settings {


    pub fn get_config(config_subset: &str) -> Option<Map<String, Value>> {

        let data = Self::get_json_data("data/settings.json");

        println!("JSON данные: {:?}", data);

        data.get(config_subset)
            .and_then(|subset| subset.as_object().cloned())

    }

    fn get_json_data(json_path: &str) -> Value {
        let file = File::open(&json_path).unwrap_or_else(|_| {
            panic!("Unable to open {}", &json_path);
        });
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).expect("Unable to parse")
    }





}

pub struct ApplySettings {
    pub name: String,
    pub framerate: u64,
}

impl ApplySettings {
    pub fn apply_window_settings(window: &Window, settings: Arc<Mutex<u64>>) {
        let config = Settings::get_config("window");
        println!("config: {:?}", config);

        if let Some(config_map) = config {
            if let (Some(x), Some(y)) = (
                config_map.get("resolution").and_then(|res| res.get("x")).and_then(Value::as_u64),
                config_map.get("resolution").and_then(|res| res.get("y")).and_then(Value::as_u64),
            ) {
                let window_size = PhysicalSize::new(x as u32, y as u32);
                window.set_inner_size(window_size);
            } else {
                eprintln!("Resolution settings not found or incorrect format in JSON.");
            }

            if let Some(window_mode) = config_map.get("window_mode").and_then(Value::as_str) {
                match window_mode {
                    "fullscreen" => window.set_fullscreen(Some(Fullscreen::Borderless(None))),
                    "windowed" => window.set_fullscreen(None),
                    _ => {}
                }
            }

            // Обновление частоты кадров в Arc<Mutex> для динамического изменения
            if let Some(fps) = config_map.get("framerate").and_then(Value::as_u64) {
                *settings.lock().unwrap() = fps;
            }
        } else {
            eprintln!("Settings not found in JSON.");
        }
    }
}

