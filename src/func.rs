use crate::lists::*;
#[cfg(feature = "advanced")]
use crate::{GamepadInjector, PenInjector};
use cgisf_lib::{gen_sentence, SentenceConfigBuilder};
use chrono::{prelude::*, DateTime};
use enigo::*;
use lazy_static::lazy_static;
use rand::{
    distributions::{Distribution, WeightedIndex},
    thread_rng, Rng,
};
use regex::Regex;
use rsautogui::{mouse, mouse::Button, mouse::ScrollAxis, mouse::Speed};
use screenshots::Screen;
use std::{fs::*, str::FromStr, sync::Mutex, thread};
use tokio::time::{sleep, Duration};
use tts::*;

lazy_static! {
    static ref IS_SHIFT_PRESSED: Mutex<bool> = Mutex::new(false);
    static ref IS_CTRL_PRESSED: Mutex<bool> = Mutex::new(false);
    static ref IS_ALT_PRESSED: Mutex<bool> = Mutex::new(false);
    static ref IS_LSHIFT_PRESSED: Mutex<bool> = Mutex::new(false);
    static ref IS_LCTRL_PRESSED: Mutex<bool> = Mutex::new(false);
    static ref IS_RSHIFT_PRESSED: Mutex<bool> = Mutex::new(false);
    static ref IS_RCTRL_PRESSED: Mutex<bool> = Mutex::new(false);
}

fn toggle_key_press(key: Key, enigo: &mut Enigo) {
    let kstr = match key {
        Key::Shift => IS_SHIFT_PRESSED.lock().unwrap(),
        Key::Control => IS_CTRL_PRESSED.lock().unwrap(),
        Key::Alt => IS_ALT_PRESSED.lock().unwrap(),
        Key::LShift => IS_LSHIFT_PRESSED.lock().unwrap(),
        Key::LControl => IS_LCTRL_PRESSED.lock().unwrap(),
        Key::RShift => IS_RSHIFT_PRESSED.lock().unwrap(),
        Key::RControl => IS_RCTRL_PRESSED.lock().unwrap(),
        _ => return,
    };
    let kvalue = *kstr;
    let kstr = !kvalue;
    if kstr {
        let _ = enigo.key(key, Direction::Press);
    } else {
        let _ = enigo.key(key, Direction::Release);
    }
}

fn convert_mouse_action(input: &str) -> Option<Button> {
    match input {
        "left" => Some(Button::Left),
        "right" => Some(Button::Right),
        "middle" => Some(Button::Middle),
        _ => None,
    }
}

fn convert_to_human_readable(timestamp: &str) -> String {
    // Parse the timestamp
    let parsed_time = DateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M:%S%.f %:z")
        .expect("Invalid timestamp format");

    // Format the parsed time in a human-readable format
    let formatted_time = parsed_time.format("%Y-%m-%d %H.%M.%S%.3f").to_string();

    formatted_time
}

fn keyboard(enigo: &mut Enigo, rng: &mut rand::rngs::ThreadRng) {
    let lists: Vec<(Vec<(Key, usize)>, usize)> = vec![
        (ALPHANUMERIC_KEYS.to_vec(), 5),
        (FUNCTION_KEYS.to_vec(), 3),
        (SPECIAL_KEYS.to_vec(), 1),
        (MODIFIER_KEYS.to_vec(), 2),
    ];
    let index: WeightedIndex<usize> =
        WeightedIndex::new(lists.iter().map(|item: &(Vec<(Key, usize)>, usize)| item.1)).unwrap();
    let list: &Vec<(Key, usize)> = &lists[index.sample(rng)].0;
    let index2: WeightedIndex<usize> =
        WeightedIndex::new(list.iter().map(|item: &(Key, usize)| item.1)).unwrap();
    if list.contains(&(Key::Shift, 1)) {
        toggle_key_press(list[index2.sample(rng)].0, enigo);
    } else {
        let _ = enigo.key(list[index2.sample(rng)].0, Direction::Click);
    }
}

fn mouse(enigo: &mut Enigo, rng: &mut rand::rngs::ThreadRng) {
    let lists: Vec<(Vec<(&str, usize)>, usize)> = vec![
        (MOUSE_MOVE.to_vec(), 5),
        (MOUSE_CLICKS.to_vec(), 3),
        (MOUSE_SCROLL.to_vec(), 1),
    ];
    let index: WeightedIndex<usize> = WeightedIndex::new(
        lists
            .iter()
            .map(|item: &(Vec<(&str, usize)>, usize)| item.1),
    )
    .unwrap();
    let list: &Vec<(&str, usize)> = &lists[index.sample(rng)].0;
    let index2: WeightedIndex<usize> =
        WeightedIndex::new(list.iter().map(|item: &(&str, usize)| item.1)).unwrap();
    let click: &str = list[index2.sample(rng)].0;

    if Regex::new(r"mouse_down_.+").unwrap().is_match(click) {
        let typeclick: Button = convert_mouse_action(click.split('_').collect::<Vec<_>>()[2])
            .expect("cant convert mouse action");
        mouse::down(typeclick);
        thread::sleep(Duration::from_millis(rng.gen_range(0..=5000)));
        mouse::up(typeclick);
    } else if Regex::new(r"mouse_click_.+").unwrap().is_match(click) {
        let typeclick: Button = convert_mouse_action(click.split('_').collect::<Vec<_>>()[2])
            .expect("cant convert mouse action");
        mouse::click(typeclick);
    } else {
        match click {
            "mouse_move_abs" => mouse::move_to(
                rng.gen_range(0..=Mouse::main_display(enigo).unwrap().0)
                    .try_into()
                    .unwrap(),
                rng.gen_range(0..=Mouse::main_display(enigo).unwrap().1)
                    .try_into()
                    .unwrap(),
            ),
            "mouse_move_rel" => mouse::move_rel(
                rng.gen_range(0..=Mouse::main_display(enigo).unwrap().0),
                rng.gen_range(0..=Mouse::main_display(enigo).unwrap().1),
            ),
            "mouse_drag_abs_std" => mouse::drag_to(
                rng.gen_range(0..=Mouse::main_display(enigo).unwrap().0)
                    .try_into()
                    .unwrap(),
                rng.gen_range(0..=Mouse::main_display(enigo).unwrap().1)
                    .try_into()
                    .unwrap(),
            ),
            "mouse_drag_rel_std" => mouse::drag_rel(
                rng.gen_range(0..=Mouse::main_display(enigo).unwrap().0),
                rng.gen_range(0..=Mouse::main_display(enigo).unwrap().1),
            ),
            "mouse_drag_abs_fst" => mouse::slow_drag_to(
                rng.gen_range(0..=Mouse::main_display(enigo).unwrap().0)
                    .try_into()
                    .unwrap(),
                rng.gen_range(0..=Mouse::main_display(enigo).unwrap().1)
                    .try_into()
                    .unwrap(),
                Speed::Fast,
            ),
            "mouse_drag_rel_fst" => mouse::slow_drag_rel(
                rng.gen_range(0..=Mouse::main_display(enigo).unwrap().0),
                rng.gen_range(0..=Mouse::main_display(enigo).unwrap().1),
                Speed::Fast,
            ),
            "mouse_drag_abs_slw" => mouse::slow_drag_to(
                rng.gen_range(0..=Mouse::main_display(enigo).unwrap().0)
                    .try_into()
                    .unwrap(),
                rng.gen_range(0..=Mouse::main_display(enigo).unwrap().1)
                    .try_into()
                    .unwrap(),
                Speed::Slow,
            ),
            "mouse_drag_rel_slw" => mouse::slow_drag_rel(
                rng.gen_range(0..=Mouse::main_display(enigo).unwrap().0),
                rng.gen_range(0..=Mouse::main_display(enigo).unwrap().1),
                Speed::Slow,
            ),
            "mouse_scroll_x" => mouse::scroll(ScrollAxis::X, rng.gen_range(1..=200)),
            "mouse_scroll_y" => mouse::scroll(ScrollAxis::Y, rng.gen_range(1..=175)),
            "mouse_scroll_xy" => {
                mouse::scroll(ScrollAxis::X, rng.gen_range(0..=200));
                mouse::scroll(ScrollAxis::Y, rng.gen_range(0..=175));
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
        (QUOTES_STATEMENT.to_vec(), 2),
    ];
    let index: WeightedIndex<usize> = WeightedIndex::new(lists.iter().map(|item| item.1)).unwrap();
    let list: &Vec<(&str, usize)> = &lists[index.sample(rng)].0;
    let index2: WeightedIndex<usize> = WeightedIndex::new(list.iter().map(|item| item.1)).unwrap();
    let quote: &str = list[index2.sample(rng)].0;
    println!("{}", quote);
    let _ = tts.speak(quote, true);
}

fn quote_gen(tts: &mut Tts) {
    let quote: &str = &gen_sentence(SentenceConfigBuilder::random().build());
    println!("{}", quote);
    let _ = tts.speak(quote, true);
}

async fn quote_gen_ext(tts: &mut Tts) {
    if ping::ping(
        std::net::IpAddr::from_str("8.8.8.8").unwrap(),
        None,
        None,
        None,
        None,
        None,
    )
    .is_ok()
    {
        let quote: &str = &reqwest::get("http://metaphorpsum.com/sentences/1/")
            .await
            .expect("could not get external sentence api")
            .text()
            .await
            .unwrap();
        println!("{}", quote);
        let _ = tts.speak(quote, true);
    }
}

fn screenshot(tts: &mut Tts) {
    println!("hahahahah i am going to screenshot everything");
    let _ = tts.speak("hahahahah i am going to screenshot everything", true);
    let screens: Vec<Screen> = Screen::all().unwrap();

    for screen in screens {
        let time: String = convert_to_human_readable(Local::now().to_string().as_str());
        let _ = File::create(format!("screenshots/{}.png", time));
        let image: screenshots::image::ImageBuffer<screenshots::image::Rgba<u8>, Vec<u8>> =
            screen.capture().unwrap();
        image.save(format!("screenshots/{}.png", time)).unwrap();
    }
}

pub async fn main_logic(options: &[(&str, usize)], tts: &mut Tts, enigo: &mut Enigo) {
    let mut rng: rand::prelude::ThreadRng = thread_rng();

    let index: WeightedIndex<usize> =
        WeightedIndex::new(options.iter().map(|item| item.1)).unwrap();
    match options[index.sample(&mut rng)].0 {
        "keyboard" => keyboard(enigo, &mut rng),
        "mouse" => mouse(enigo, &mut rng),
        "quote" => quote(tts, &mut rng),
        "screenshot" => screenshot(tts),
        "quote_gen" => quote_gen(tts),
        "quote_gen_ext" => quote_gen_ext(tts).await,
        _ => println!("idk what you did but fix it"),
    }

    sleep(Duration::from_millis(1500)).await;
}

#[cfg(feature = "advanced")]
fn gamepad(gamepad: &mut GamepadInjector, rng: &mut rand::rngs::ThreadRng) {
    let lists: Vec<(Vec<(&str, usize)>, usize)> = vec![
        (GAMEPAD_BUTTONS.to_vec(), 5),
        (GAMEPAD_MOVE.to_vec(), 3),
        (GAMEPAD_SPECIAL.to_vec(), 1),
    ];
    let index: WeightedIndex<usize> = WeightedIndex::new(
        lists
            .iter()
            .map(|item: &(Vec<(&str, usize)>, usize)| item.1),
    )
    .unwrap();
    let list: &Vec<(&str, usize)> = &lists[index.sample(rng)].0;
    let index2: WeightedIndex<usize> =
        WeightedIndex::new(list.iter().map(|item: &(&str, usize)| item.1)).unwrap();
    let action = list[index2.sample(rng)].0;
    match action {
        "LeftThumbstickMove" => {
            gamepad.update_left_thumbstick((rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0)));
            gamepad.inject();
            thread::sleep(Duration::from_millis(rng.gen_range(0..=5000)));
            gamepad.update_left_thumbstick((0.0, 0.0));
        }
        "RightThumbstickMove" => {
            gamepad.update_right_thumbstick((rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0)));
            gamepad.inject();
            thread::sleep(Duration::from_millis(rng.gen_range(0..=5000)));
            gamepad.update_right_thumbstick((0.0, 0.0));
        }
        "LeftTrigger" => {
            gamepad.update_left_trigger(rng.gen_range(0.0..=1.0));
            gamepad.inject();
            thread::sleep(Duration::from_millis(rng.gen_range(0..=5000)));
            gamepad.update_left_trigger(0.0);
        }
        "RightTrigger" => {
            gamepad.update_right_trigger(rng.gen_range(0.0..=1.0));
            gamepad.inject();
            thread::sleep(Duration::from_millis(rng.gen_range(0..=5000)));
            gamepad.update_right_trigger(0.0);
        }
        _ => {
            gamepad.toggle_button(action);
        }
    }
    gamepad.inject();
}

#[cfg(feature = "advanced")]
fn pen(pen: &mut PenInjector, rng: &mut rand::rngs::ThreadRng) {
    let lists: Vec<(Vec<(&str, usize)>, usize)> = vec![
        (PEN_BUTTONS.to_vec(), 3),
        (PEN_MOVE.to_vec(), 5),
        (PEN_SPECIAL.to_vec(), 1),
    ];
    let index: WeightedIndex<usize> = WeightedIndex::new(
        lists
            .iter()
            .map(|item: &(Vec<(&str, usize)>, usize)| item.1),
    )
    .unwrap();
    let list: &Vec<(&str, usize)> = &lists[index.sample(rng)].0;
    let index2: WeightedIndex<usize> =
        WeightedIndex::new(list.iter().map(|item: &(&str, usize)| item.1)).unwrap();
    let action = list[index2.sample(rng)].0;
    match action {
        "Pressure" => {
            pen.update_pressure(rng.gen_range(0.0..=1024.0));
        }
        "Rotation" => {
            pen.update_rotation(rng.gen_range(0.0..=359.0));
        }
        "Tilt" => {
            pen.update_tilt((rng.gen_range(-90..=90), rng.gen_range(-90..=90)));
        }
        "XY_Move" => {
            let display = Screen::from_point(0, 0).unwrap().display_info;
            pen.update_position((
                rng.gen_range(0..=display.width).try_into().unwrap(),
                rng.gen_range(0..=display.height).try_into().unwrap(),
            ));
        }
        "X_Move" => {
            let display = Screen::from_point(0, 0).unwrap().display_info;
            pen.update_position((rng.gen_range(0..=display.width).try_into().unwrap(), -1));
        }
        "Y_Move" => {
            let display = Screen::from_point(0, 0).unwrap().display_info;
            pen.update_position((-1, rng.gen_range(0..=display.height).try_into().unwrap()));
        }
        _ => {
            pen.toggle_button(action);
        }
    }
    pen.inject();
}

#[cfg(feature = "advanced")]
pub async fn main_logic_adv(
    options: &[(&str, usize)],
    tts: &mut Tts,
    enigo: &mut Enigo,
    gamepadobj: &mut GamepadInjector,
    penobj: &mut PenInjector,
) {
    let mut rng: rand::prelude::ThreadRng = thread_rng();

    let index: WeightedIndex<usize> =
        WeightedIndex::new(options.iter().map(|item| item.1)).unwrap();
    match options[index.sample(&mut rng)].0 {
        "keyboard" => keyboard(enigo, &mut rng),
        "gamepad" => gamepad(gamepadobj, &mut rng),
        "pen" => pen(penobj, &mut rng),
        "mouse" => mouse(enigo, &mut rng),
        "quote" => quote(tts, &mut rng),
        "screenshot" => screenshot(tts),
        "quote_gen" => quote_gen(tts),
        "quote_gen_ext" => quote_gen_ext(tts).await,
        _ => println!("idk what you did but fix it"),
    }

    sleep(Duration::from_millis(1500)).await;
}
