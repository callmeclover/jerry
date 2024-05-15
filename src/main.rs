#![cfg_attr(feature = "invisibility", windows_subsystem = "windows")]

mod config;
mod func;
mod lists;
#[cfg(feature = "advanced")]
mod model;
#[cfg(feature = "advanced")]
use model::*;
use std::fs::*;
use enigo::*;
use config::*;
use func::main_logic;
use tts::*;

#[tokio::main]
async fn main() {
    let options: Vec<(&str, usize)> = get_options(get_config().await).await;

    println!("\nStarting Jerry...");
    println!("Jerry has been started!\n");

    if metadata("screenshots").is_err() {
        let _ = create_dir("screenshots");
    }
    let mut tts: Tts = match Tts::default() {
        Ok(tts) => tts,
        Err(err) => {
            panic!("Failed to create tts instance: {:?}", err);
        }
    };

    let mut enigo: Enigo = Enigo::new(&Settings::default()).unwrap();
    let mut gamepad = None;
    if cfg!(feature = "advanced") {
        gamepad = Some(GamepadInjector::new());
    }

    loop {
        main_logic(&options, &mut tts, &mut enigo, &mut gamepad).await;
    }
}