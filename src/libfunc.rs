use crate::liblists::*;
use std::fs::*;
use enigo::*;
use rand::Rng;
use rand::distributions::{ WeightedIndex, Distribution };
use rand::thread_rng;
use tokio::time::{ sleep, Duration };
use regex::Regex;

use tts::*;
use chrono::prelude::*;
use chrono::DateTime;
use screenshots::Screen;

fn convert_mouse_action(input: &str) -> Option<MouseButton> {
    match input {
        "left" => Some(MouseButton::Left),
        "right" => Some(MouseButton::Right),
        "middle" => Some(MouseButton::Middle),
        "back" => Some(MouseButton::Back),
        "forward" => Some(MouseButton::Forward),
        "scrollup" => Some(MouseButton::ScrollUp),
        "scrolldown" => Some(MouseButton::ScrollDown),
        "scrollright" => Some(MouseButton::ScrollRight),
        "scrollleft" => Some(MouseButton::ScrollLeft),
        _ => { None }
    }
}

fn convert_to_human_readable(timestamp: &str) -> String {
    // Parse the timestamp
    let parsed_time = DateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M:%S%.f %:z").expect(
        "Invalid timestamp format"
    );

    // Format the parsed time in a human-readable format
    let formatted_time = parsed_time.format("%Y-%m-%d %H.%M.%S%.3f").to_string();

    formatted_time
}

fn keyboard(enigo: &mut Enigo, rng: &mut rand::rngs::ThreadRng) {
    let lists: Vec<(Vec<(Key, usize)>, usize)> = vec![
        (ALPHANUMERIC_KEYS.to_vec(), 5),
        (FUNCTION_KEYS.to_vec(), 3),
        (SPECIAL_KEYS.to_vec(), 1),
        (MODIFIER_KEYS.to_vec(), 2)
    ];
    let index: WeightedIndex<usize> = WeightedIndex::new(
        lists.iter().map(|item: &(Vec<(Key, usize)>, usize)| item.1)
    ).unwrap();
    let list: &Vec<(Key, usize)> = &lists[index.sample(rng)].0;
    let index2: WeightedIndex<usize> = WeightedIndex::new(
        list.iter().map(|item: &(Key, usize)| item.1)
    ).unwrap();
    enigo.key_click(list[index2.sample(rng)].0);
}

fn gamepad(enigo: &mut Enigo, rng: &mut rand::rngs::ThreadRng) {
    let lists: Vec<(Vec<(Key, usize)>, usize)> = vec![
        (GAMEPAD_BUTTONS.to_vec(), 5),
        (GAMEPAD_MOVE.to_vec(), 3),
        (GAMEPAD_SPECIAL.to_vec(), 1)
    ];
    let index: WeightedIndex<usize> = WeightedIndex::new(
        lists.iter().map(|item: &(Vec<(Key, usize)>, usize)| item.1)
    ).unwrap();
    let list: &Vec<(Key, usize)> = &lists[index.sample(rng)].0;
    let index2: WeightedIndex<usize> = WeightedIndex::new(
        list.iter().map(|item: &(Key, usize)| item.1)
    ).unwrap();
    enigo.key_click(list[index2.sample(rng)].0);
}

fn mouse(enigo: &mut Enigo, rng: &mut rand::rngs::ThreadRng) {
    let lists: Vec<(Vec<(&str, usize)>, usize)> = vec![
        (MOUSE_MOVE.to_vec(), 5),
        (MOUSE_CLICKS.to_vec(), 3),
        (MOUSE_SCROLL.to_vec(), 1)
    ];
    let index: WeightedIndex<usize> = WeightedIndex::new(
        lists.iter().map(|item: &(Vec<(&str, usize)>, usize)| item.1)
    ).unwrap();
    let list: &Vec<(&str, usize)> = &lists[index.sample(rng)].0;
    let index2: WeightedIndex<usize> = WeightedIndex::new(
        list.iter().map(|item: &(&str, usize)| item.1)
    ).unwrap();
    let click: &str = list[index2.sample(rng)].0;

    if Regex::new(r"mouse_down_.+").unwrap().is_match(click) {
        let typeclick: &str = click.split("_").collect::<Vec<_>>()[2];
        enigo.mouse_down(convert_mouse_action(typeclick).unwrap());
    } else if Regex::new(r"mouse_up_.+").unwrap().is_match(click) {
        let typeclick: &str = click.split("_").collect::<Vec<_>>()[2];
        enigo.mouse_down(convert_mouse_action(typeclick).unwrap());
    } else if Regex::new(r"mouse_click_.+").unwrap().is_match(click) {
        let typeclick: &str = click.split("_").collect::<Vec<_>>()[2];
        enigo.mouse_down(convert_mouse_action(typeclick).unwrap());
    } else {
        match click {
            "mouse_move_abs" => enigo.mouse_move_to(rng.gen_range(0..=MouseControllable::main_display_size(enigo).0), rng.gen_range(0..=MouseControllable::main_display_size(enigo).1)),
            "mouse_move_rel" =>
                enigo.mouse_move_relative(
                    rng.gen_range(0..=MouseControllable::main_display_size(enigo).0),
                    rng.gen_range(0..=MouseControllable::main_display_size(enigo).1)
                ),
            "mouse_scroll_x" => enigo.mouse_scroll_x(rng.gen_range(1..=200)),
            "mouse_scroll_y" => enigo.mouse_scroll_y(rng.gen_range(1..=175)),
            "mouse_scroll_xy" => {
                enigo.mouse_scroll_x(rng.gen_range(0..=200));
                enigo.mouse_scroll_y(rng.gen_range(0..=175));
            }
            _ => {}
        }
    }
}

fn quote(tts: &mut Tts, rng: &mut rand::rngs::ThreadRng) {
    let lists: Vec<(Vec<(&str, usize)>, usize)> = vec![
        (QUOTES_NEGATIVE.to_vec(), 5),
        (QUOTES_POSITIVE.to_vec(), 3),
        (QUOTES_QUESTION.to_vec(), 1),
        (QUOTES_STATEMENT.to_vec(), 2)
    ];
    let index: WeightedIndex<usize> = WeightedIndex::new(lists.iter().map(|item| item.1)).unwrap();
    let list: &Vec<(&str, usize)> = &lists[index.sample(rng)].0;
    let index2: WeightedIndex<usize> = WeightedIndex::new(list.iter().map(|item| item.1)).unwrap();
    let quote: &str = list[index2.sample(rng)].0;
    println!("{}", quote);
    let _ = tts.speak(quote, true);
}

fn screenshot(tts: &mut Tts) {
    println!("hahahahah i am going to screenshot everything");
    let _ = tts.speak("hahahahah i am going to screenshot everything", true);
    let screens: Vec<Screen> = Screen::all().unwrap();

    for screen in screens {
        let time: String = convert_to_human_readable(Local::now().to_string().as_str());
        let _ = File::create(format!("screenshots/{}.png", time));
        let image: screenshots::image::ImageBuffer<screenshots::image::Rgba<u8>, Vec<u8>> = screen
            .capture()
            .unwrap();
        image.save(format!("screenshots/{}.png", time)).unwrap();
    }
}

// TODO: add functions to select inputs
pub async fn main_logic(options: &Vec<(&str, usize)>, tts: &mut Tts, mut enigo: &mut Enigo) {
    let mut rng: rand::prelude::ThreadRng = thread_rng();

    let index: WeightedIndex<usize> = WeightedIndex::new(options.iter().map(|item| item.1)).unwrap();
    match options[index.sample(&mut rng)].0 {
        "keyboard" => keyboard(&mut enigo, &mut rng),
        "gamepad" => gamepad(&mut enigo, &mut rng),
        "mouse" => mouse(&mut enigo, &mut rng),
        "quote" => quote(tts, &mut rng),
        "screenshot" => screenshot(tts),
        _ => println!("idk what you did but fix it"),
    }

    sleep(Duration::from_millis(1500)).await;
}
