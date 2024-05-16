#![cfg_attr(feature = "invisibility", windows_subsystem = "windows")]

mod config;
mod func;
mod lists;
#[cfg(feature = "advanced")]
mod model;
use config::*;
use enigo::*;
use func::*;
#[cfg(feature = "advanced")]
use model::*;
use std::fs::*;
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

    #[cfg(feature = "advanced")]
    {
        let mut gamepad = GamepadInjector::new();
        let mut pen = PenInjector::new();
        loop {
            main_logic_adv(&options, &mut tts, &mut enigo, &mut gamepad, &mut pen).await;
        }
    }
    #[allow(unreachable_code)]
    loop {
        main_logic(&options, &mut tts, &mut enigo).await;
    }
}
