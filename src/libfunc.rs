//use crate::liblists::*;
use crate::libconf::*;
use enigo::Enigo;
use enigo::Key;
use rand::distributions::{WeightedIndex, Distribution};
use rand::thread_rng;
use tokio::time::{sleep, Duration};

use chrono::prelude::*;
//use tts::*;
use screenshots::Screen;

fn get_weighted_index(list: &Vec<(&str, usize)>) -> WeightedIndex<usize> {
    return WeightedIndex::new(list.iter().map(|item| item.1)).unwrap();
}

fn get_weighted_index_keys(list: Vec<(Key, usize)>) -> WeightedIndex<usize> {
    return WeightedIndex::new(list.iter().map(|item| item.1)).unwrap();
}

fn keyboard(enigo: &Enigo) {
    println!("Keyboard");
}

fn gamepad(enigo: &Enigo) {
    println!("Gamepad");
}

fn mouse(enigo: &Enigo) {
    println!("Mouse");
}
/*
fn quote(tts: Tts) {
    let mut rng = thread_rng();
    let index = get_weighted_index(QUOTES_POSITIVE).sample(&mut rng);
    tts.speak(&QUOTES_POSITIVE[index]);
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
} */

// TODO: add functions to select inputs
pub async fn main_logic(options: &Vec<(&str, usize)>,/* tts,*/ enigo: &Enigo) {
    let mut rng = thread_rng();
    sleep(Duration::from_secs(1)).await;
    
    match options[get_weighted_index(&options).sample(&mut rng)].0 {
        "keyboard" => keyboard(enigo),
        "gamepad" => gamepad(enigo),
        "mouse" => mouse(enigo),
        "quote" => println!("quote"),//quote(/*tts */),
        "screenshot" => println!("screenshot"),//screenshot(/*tts */),
        _ => println!("We found an unexpected value. Check your config files maybe?"),
    }

    sleep(Duration::from_secs(1)).await;
}