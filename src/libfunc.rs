use crate::liblists::*;
use crate::libconf::*;
use enigo::Key;
use tokio::time::{sleep, Duration};

use chrono::prelude::*;
use tts::*;
use screenshots::Screen;

fn get_weighted_index(list: Vec<(&'static str, usize)>) -> WeightedIndex<usize> {
    return WeightedIndex::new(list.iter().map(|item| item.1)).unwrap();
}

fn get_weighted_index_keys(list: Vec<(Key, usize)>) -> WeightedIndex<usize> {
    return WeightedIndex::new(list.iter().map(|item| item.1)).unwrap();
}

fn gamepad(enigo: Enigo) {
    
}

fn mouse(enigo: Enigo) {
    
}

fn quote(tts: Tts) {
    match 
}

fn screenshot(tts: Tts) {
    println("hahahahah i am going to screenshot everything");
    let screens = Screen::all().unwrap();

    for screen in screens {
        let mut image = screen.capture().unwrap();
        image
            .save(format!("screenshots/{}.png", Local::now()))
            .unwrap();

    }
}

// TODO: add functions to select inputs
pub async fn main_logic(options, tts, enigo) {
    let mut rng = thread_rng();
    sleep(Duration::from_secs(1)).await;
    
    match options[get_weighted_index(options).sample(&mut rng)].0 {
        "keyboard" => keyboard(enigo),
        "gamepad" => gamepad(enigo),
        "mouse" => mouse(enigo),
        "quote" => quote(tts),
        "screenshot" => screenshot(tts),
        _ => println!("We found an unexpected value. Check your config files maybe?"),
    }

    sleep(Duration::from_secs(1)).await;
}