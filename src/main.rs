#![cfg_attr(
    all(feature = "invisibility", target_os = "windows"),
    windows_subsystem = "windows"
)]

// Begin allow clippy warnings

#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]

// End allow clippy warnings

mod config;
mod func;
mod lists;
mod model;

use config::{get_config, get_options, Config};
use enigo::{Enigo, Settings};
#[allow(clippy::wildcard_imports)]
use func::*;
#[allow(clippy::wildcard_imports)]
use model::*;
use std::fs::{create_dir, metadata};
use tts::Tts;

#[tokio::main]
/// Where this asshole starts.
async fn main() {
    #[cfg(all(target_os = "unix", feature = "invisibility"))]
    {
        let daemonize = daemonize::Daemonize::new();
        match daemonize.start() {
            Ok(_) => println!("Success, daemonized"),
            Err(e) => eprintln!("Error, {}", e),
        }
    }
    let config: Config = get_config();
    let options: Vec<(&str, usize)> = get_options(&config);

    println!("\nStarting Jerry...");
    println!("Jerry has been started!\n");

    if metadata("screenshots").is_err() {
        let _ = create_dir("screenshots");
    }
    let mut tts: Tts = match Tts::default() {
        Ok(tts) => tts,
        Err(err) => {
            panic!("Failed to create tts instance: {err:?}");
        }
    };

    let mut enigo: Enigo = Enigo::new(&Settings::default()).unwrap();

    #[cfg(all(feature = "advanced", target_os = "windows"))]
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
