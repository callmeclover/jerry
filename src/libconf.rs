use serde::Deserialize;
use std::{ fs, path::Path };
use rand::prelude::*;
use rand::distributions::WeightedIndex;
use toml::*;

#[derive(Debug, Deserialize)]
pub struct ConfigToml {
    #[allow(dead_code)] // Disable dead code warning for the entire struct
    basic: Basic,
}

#[derive(Debug, Deserialize)]
struct Basic {
    #[allow(dead_code)]
    use_mouse: bool,
    #[allow(dead_code)]
    use_keyboard: bool,
    #[allow(dead_code)]
    use_controller: bool,
    #[allow(dead_code)]
    do_screenshots: bool,
    #[allow(dead_code)]
    do_tts: bool,
}

fn get_config_options(CONFIG) -> Vec<(&'static str, usize)> {
    let mut config: Vec<(&'static str, usize)> = vec![];

    if CONFIG.basic.use_mouse {
        config.push(("mouse", 7));
    }
    if CONFIG.basic.use_keyboard {
        config.push(("keyboard", 8));
    }
    if CONFIG.basic.use_controller {
        config.push(("gamepad", 8));
    }
    if CONFIG.basic.do_screenshots {
        config.push(("screenshot", 1));
    }
    if CONFIG.basic.do_tts {
        config.push(("quote", 4));
    }

    return config;
}

pub fn get_config() {
    let mut config;

    if Path::new("./config.toml").exists() {
    config = get_config_options(toml
    ::from_str(&fs::read_to_string("./config.toml").expect("Failed to read Cargo.toml file"))
    .expect("Failed to deserialize Cargo.toml"));
    }

    return config;
}