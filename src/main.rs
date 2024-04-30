mod libfunc;
mod libconf;
mod liblists;
use ansi_term::Color;
use libconf::*;
use libfunc::main_logic;

use std::fs::*;
use tts::*;
use enigo::*;

#[tokio::main]
async fn main() {
    let options: Vec<(&str, usize)> = get_options(get_config().await).await;

    println!("\n{} Starting Jerry...", Color::Blue.paint("[INFO]:"));
    println!("{} Jerry has been started!\n", Color::Green.paint("[OK]:"));

    if !metadata("screenshots").is_ok() {
        let _ = create_dir("screenshots");
    }

    let mut tts: Tts = match Tts::default() {
        Ok(tts) => tts,
        Err(err) => {
            panic!("Failed to create tts instance: {:?}", err);
        }
    };

    let mut enigo: Enigo = Enigo::new(&Settings::default()).unwrap();
    
    loop {
        main_logic(&options, &mut tts, &mut enigo).await;    
    }
}
