use config_file::FromConfigFile;
use serde::Deserialize;
use lazy_static::lazy_static;


const CONFIG_FILE_PATH: &str = "/etc/{{project_name}}/config.json";

#[derive(Deserialize, Default)]
pub struct Config {
    pub example_value_1: u64,
    pub example_value_2: Option<String>
}

impl Config {
    fn new() -> Self {
        match Config::from_config_file(CONFIG_FILE_PATH) {
            Ok(v) => v,
            Err(e) => {
                println!("Unable to load config file: {}, {:?} -> using defaults", CONFIG_FILE_PATH, e);
                Config { example_value_1: 1, ..Default::default() }
            }
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

