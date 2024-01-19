use crate::liblists::{*, self};
use tokio::time::{sleep, Duration};
use serde::Deserialize;
use std::fs;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: ConfigToml = toml
    ::from_str(&fs::read_to_string("./config.toml").expect("Failed to read Cargo.toml file"))
    .expect("Failed to deserialize Cargo.toml");
}

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

fn get_config_options() -> Vec<(&'static str, usize)> {
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

// TODO: add functions to select inputs
pub async fn main_logic() {
    sleep(Duration::from_secs(1)).await;
    println!("{:#?}", get_config_options());
}

//
// TODO:
// - Get primary input list
// - Get secondary input list
// - Do things
//