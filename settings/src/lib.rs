use serde_json::{Map, Value};
use std::fs::File;
use std::io::BufReader;

pub struct Settings {
    pub name: String
}

impl Settings {


    pub fn get_config(config_subset: &str) -> Option<Map<String, Value>> {

        let file = File::open("settings/src/settings.json").expect("Unable to open settings.json");
        let reader = BufReader::new(file);

        let data: Value = serde_json::from_reader(reader).expect("Unable to parse settings.json");

        println!("JSON данные: {:?}", data);

        data.get(config_subset)
            .and_then(|subset| subset.as_object().cloned())

    }



}