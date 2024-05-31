#[cfg(target_os = "macos")]
use cocoa_foundation::base::id;
#[cfg(target_os = "macos")]
use cocoa_foundation::foundation::NSDefaultRunLoopMode;
#[cfg(target_os = "macos")]
use cocoa_foundation::foundation::NSRunLoop;
#[cfg(target_os = "macos")]
use objc::{class, msg_send, sel, sel_impl};

#[allow(clippy::wildcard_imports)]
use crate::{
    get_config, lists::*, Speed, SPEED_WEIGHTED_LISTS_FAST, SPEED_WEIGHTED_LISTS_NORMAL,
    SPEED_WEIGHTED_LISTS_SLOW,
};
#[cfg(all(feature = "advanced", target_os = "windows"))]
use crate::{GamepadInjector, PenInjector};
use cgisf_lib::{gen_sentence, SentenceConfigBuilder};
use chrono::{prelude::*, DateTime};
use enigo::{Axis, Button, Coordinate, Direction, Enigo, Key, Keyboard, Mouse};
use lazy_static::lazy_static;
use rand::{
    distributions::{Distribution, WeightedIndex},
    thread_rng, Rng,
};
use regex::Regex;
use screenshots::Screen;
use std::{fs::File, str::FromStr, sync::Mutex, thread};
use tokio::time::{sleep, Duration};
use tts::Tts;

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
    let kvalue = !*(match key {
        Key::Shift => IS_SHIFT_PRESSED.lock().unwrap(),
        Key::Control => IS_CTRL_PRESSED.lock().unwrap(),
        Key::Alt => IS_ALT_PRESSED.lock().unwrap(),
        Key::LShift => IS_LSHIFT_PRESSED.lock().unwrap(),
        Key::LControl => IS_LCTRL_PRESSED.lock().unwrap(),
        Key::RShift => IS_RSHIFT_PRESSED.lock().unwrap(),
        Key::RControl => IS_RCTRL_PRESSED.lock().unwrap(),
        _ => return,
    });
    if kvalue {
        let _ = enigo.key(key, Direction::Press);
    } else {
        let _ = enigo.key(key, Direction::Release);
    }
}

pub fn drag_mouse_abs(enigo: &mut Enigo, pos: (i32, i32), speed: Speed) {
    let (mouse_x, mouse_y) = enigo.location().expect("Unable to locate mouse position.");

    let delta_x = pos.0 - mouse_x;
    let delta_y = pos.1 - mouse_y;

    let delta_sum = (delta_x.pow(2) + delta_y.pow(2)) as f32;
    let distance = delta_sum.sqrt();

    let step_x = delta_x as f32 / distance;
    let step_y = delta_y as f32 / distance;

    let mut new_x = mouse_x as f32;
    let mut new_y = mouse_y as f32;
    let sleep_duration = Duration::from_micros(speed as u64);
    for _ in 0..distance as usize {
        new_x += step_x;
        new_y += step_y;
        enigo
            .move_mouse(new_x as i32, new_y as i32, Coordinate::Abs)
            .unwrap_or_else(|_| panic!("Unable to move mouse position to ({}, {}).", pos.0, pos.1));
        thread::sleep(sleep_duration);
    }
}

pub fn drag_mouse_rel(enigo: &mut Enigo, pos: (i32, i32), speed: Speed) {
    let delta_sum = (pos.0.pow(2) + pos.1.pow(2)) as f32;
    let distance = delta_sum.sqrt();

    let step_x = pos.0 as f32 / distance;
    let step_y = pos.1 as f32 / distance;

    let (mouse_x, mouse_y) = enigo.location().expect("Unable to locate mouse position.");
    let mut new_x = mouse_x as f32;
    let mut new_y = mouse_y as f32;
    let sleep_duration = Duration::from_micros(speed as u64);
    for _ in 0..distance as usize {
        new_x += step_x;
        new_y += step_y;
        enigo
            .move_mouse(new_x as i32, new_y as i32, Coordinate::Abs)
            .unwrap_or_else(|_| panic!("Unable to move mouse position to ({}, {}).", pos.0, pos.1));
        thread::sleep(sleep_duration);
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
    let lists: Vec<(&'static [(Key, usize)], usize)> = vec![
        (ALPHANUMERIC_KEYS, 5),
        (FUNCTION_KEYS, 3),
        (SPECIAL_KEYS, 1),
        (MODIFIER_KEYS, 2),
    ];
    let index: WeightedIndex<usize> =
        WeightedIndex::new(lists.iter().map(|item: &(&[(Key, usize)], usize)| item.1)).unwrap();
    let list: &[(Key, usize)] = lists[index.sample(rng)].0;
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
        if get_config().extra.do_debugging {
            tracing::info!("mouse: holding button {:?}", typeclick)
        }
        let _ = enigo.button(typeclick, Direction::Press);
        thread::sleep(Duration::from_millis(rng.gen_range(0..=5000)));
        let _ = enigo.button(typeclick, Direction::Release);
    } else if Regex::new(r"mouse_click_.+").unwrap().is_match(click) {
        let typeclick: Button = convert_mouse_action(click.split('_').collect::<Vec<_>>()[2])
            .expect("cant convert mouse action");
        if get_config().extra.do_debugging {
            tracing::info!("mouse: clicking button {:?}", typeclick)
        }
        let _ = enigo.button(typeclick, Direction::Click);
    } else {
        match click {
            "mouse_move_abs" => enigo
                .move_mouse(
                    rng.gen_range(0..=enigo.main_display().unwrap().0),
                    rng.gen_range(0..=enigo.main_display().unwrap().1),
                    Coordinate::Abs,
                )
                .unwrap(),
            "mouse_move_rel" => enigo
                .move_mouse(
                    rng.gen_range(0..=enigo.main_display().unwrap().0),
                    rng.gen_range(0..=enigo.main_display().unwrap().1),
                    Coordinate::Rel,
                )
                .unwrap(),
            "mouse_drag_abs_std" => drag_mouse_abs(
                enigo,
                (
                    rng.gen_range(0..=enigo.main_display().unwrap().0),
                    rng.gen_range(0..=enigo.main_display().unwrap().1),
                ),
                SPEED_WEIGHTED_LISTS_NORMAL[WeightedIndex::new(
                    SPEED_WEIGHTED_LISTS_NORMAL
                        .iter()
                        .map(|item: &(Speed, usize)| item.1),
                )
                .unwrap()
                .sample(rng)]
                .0,
            ),
            "mouse_drag_rel_std" => drag_mouse_rel(
                enigo,
                (
                    rng.gen_range(0..=enigo.main_display().unwrap().0),
                    rng.gen_range(0..=enigo.main_display().unwrap().1),
                ),
                SPEED_WEIGHTED_LISTS_NORMAL[WeightedIndex::new(
                    SPEED_WEIGHTED_LISTS_NORMAL
                        .iter()
                        .map(|item: &(Speed, usize)| item.1),
                )
                .unwrap()
                .sample(rng)]
                .0,
            ),
            "mouse_drag_abs_fst" => drag_mouse_abs(
                enigo,
                (
                    rng.gen_range(0..=enigo.main_display().unwrap().0),
                    rng.gen_range(0..=enigo.main_display().unwrap().1),
                ),
                SPEED_WEIGHTED_LISTS_FAST[WeightedIndex::new(
                    SPEED_WEIGHTED_LISTS_FAST
                        .iter()
                        .map(|item: &(Speed, usize)| item.1),
                )
                .unwrap()
                .sample(rng)]
                .0,
            ),
            "mouse_drag_rel_fst" => drag_mouse_rel(
                enigo,
                (
                    rng.gen_range(0..=enigo.main_display().unwrap().0),
                    rng.gen_range(0..=enigo.main_display().unwrap().1),
                ),
                SPEED_WEIGHTED_LISTS_FAST[WeightedIndex::new(
                    SPEED_WEIGHTED_LISTS_FAST
                        .iter()
                        .map(|item: &(Speed, usize)| item.1),
                )
                .unwrap()
                .sample(rng)]
                .0,
            ),
            "mouse_drag_abs_slw" => drag_mouse_abs(
                enigo,
                (
                    rng.gen_range(0..=enigo.main_display().unwrap().0),
                    rng.gen_range(0..=enigo.main_display().unwrap().1),
                ),
                SPEED_WEIGHTED_LISTS_SLOW[WeightedIndex::new(
                    SPEED_WEIGHTED_LISTS_SLOW
                        .iter()
                        .map(|item: &(Speed, usize)| item.1),
                )
                .unwrap()
                .sample(rng)]
                .0,
            ),
            "mouse_drag_rel_slw" => drag_mouse_rel(
                enigo,
                (
                    rng.gen_range(0..=enigo.main_display().unwrap().0),
                    rng.gen_range(0..=enigo.main_display().unwrap().1),
                ),
                SPEED_WEIGHTED_LISTS_SLOW[WeightedIndex::new(
                    SPEED_WEIGHTED_LISTS_SLOW
                        .iter()
                        .map(|item: &(Speed, usize)| item.1),
                )
                .unwrap()
                .sample(rng)]
                .0,
            ),
            "mouse_scroll_x" => enigo
                .scroll(rng.gen_range(1..=200), Axis::Horizontal)
                .unwrap(),
            "mouse_scroll_y" => enigo
                .scroll(rng.gen_range(1..=175), Axis::Vertical)
                .unwrap(),
            "mouse_scroll_xy" => {
                enigo
                    .scroll(rng.gen_range(0..=200), Axis::Horizontal)
                    .unwrap();
                enigo
                    .scroll(rng.gen_range(0..=175), Axis::Vertical)
                    .unwrap();
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
    if get_config().extra.do_debugging {
        tracing::info!("quote: choosing random sentence")
    }
    let index: WeightedIndex<usize> = WeightedIndex::new(lists.iter().map(|item| item.1)).unwrap();
    let list: &Vec<(&str, usize)> = &lists[index.sample(rng)].0;
    let index2: WeightedIndex<usize> = WeightedIndex::new(list.iter().map(|item| item.1)).unwrap();
    let quote: &str = list[index2.sample(rng)].0;
    println!("{quote}");
    let _ = tts.speak(quote, true);
    #[cfg(target_os = "macos")]
    {
        let run_loop: id = unsafe { NSRunLoop::currentRunLoop() };
        unsafe {
            let date: id = msg_send![class!(NSDate), distantFuture];
            let _: () = msg_send![run_loop, runMode:NSDefaultRunLoopMode beforeDate:date];
        }
    }
}

fn quote_gen(tts: &mut Tts) {
    if get_config().extra.do_debugging {
        tracing::info!("quote_gen: generating sentence")
    }
    let quote: &str = &gen_sentence(SentenceConfigBuilder::random().build());
    println!("{quote}");
    let _ = tts.speak(quote, true);
    #[cfg(target_os = "macos")]
    {
        let run_loop: id = unsafe { NSRunLoop::currentRunLoop() };
        unsafe {
            let date: id = msg_send![class!(NSDate), distantFuture];
            let _: () = msg_send![run_loop, runMode:NSDefaultRunLoopMode beforeDate:date];
        }
    }
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
        if get_config().extra.do_debugging {
            tracing::info!("quote_gen_ext: internet check passed, sending request to sentence api")
        }
        let quote: &str = &reqwest::get("http://metaphorpsum.com/sentences/1/")
            .await
            .expect("could not get external sentence api")
            .text()
            .await
            .unwrap();
        println!("{quote}");
        let _ = tts.speak(quote, true);
        #[cfg(target_os = "macos")]
        {
            let run_loop: id = unsafe { NSRunLoop::currentRunLoop() };
            unsafe {
                let date: id = msg_send![class!(NSDate), distantFuture];
                let _: () = msg_send![run_loop, runMode:NSDefaultRunLoopMode beforeDate:date];
            }
        }
    }
}

fn screenshot(tts: &mut Tts) {
    println!("hahahahah i am going to screenshot everything");
    let _ = tts.speak("hahahahah i am going to screenshot everything", true);
    #[cfg(target_os = "macos")]
    {
        let run_loop: id = unsafe { NSRunLoop::currentRunLoop() };
        unsafe {
            let date: id = msg_send![class!(NSDate), distantFuture];
            let _: () = msg_send![run_loop, runMode:NSDefaultRunLoopMode beforeDate:date];
        }
    }
    let screens: Vec<Screen> = Screen::all().unwrap();

    for screen in screens {
        let time: String = convert_to_human_readable(Local::now().to_string().as_str());
        let _ = File::create(format!("screenshots/{time}.png"));
        let image: screenshots::image::ImageBuffer<screenshots::image::Rgba<u8>, Vec<u8>> =
            screen.capture().unwrap();
        image.save(format!("screenshots/{time}.png")).unwrap();
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
#[cfg(target_os = "windows")]
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
            let range = (rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0));
            gamepad.update_left_thumbstick(range);

            if get_config().extra.do_debugging {
                tracing::info!("gamepad: holding left thumbstick at position {:?}", range)
            }
            gamepad.inject();
            thread::sleep(Duration::from_millis(rng.gen_range(0..=5000)));
            gamepad.update_left_thumbstick((0.0, 0.0));
        }
        "RightThumbstickMove" => {
            let range = (rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0));
            gamepad.update_right_thumbstick(range);

            if get_config().extra.do_debugging {
                tracing::info!("gamepad: holding right thumbstick at position {:?}", range)
            }
            gamepad.inject();
            thread::sleep(Duration::from_millis(rng.gen_range(0..=5000)));
            gamepad.update_right_thumbstick((0.0, 0.0));
        }
        "LeftTrigger" => {
            let placement = rng.gen_range(0.0..=1.0);
            gamepad.update_left_trigger(placement);

            if get_config().extra.do_debugging {
                tracing::info!("gamepad: holding left trigger at position {}", placement)
            }
            gamepad.inject();
            thread::sleep(Duration::from_millis(rng.gen_range(0..=5000)));
            gamepad.update_left_trigger(0.0);
        }
        "RightTrigger" => {
            let placement = rng.gen_range(0.0..=1.0);
            gamepad.update_right_trigger(placement);

            if get_config().extra.do_debugging {
                tracing::info!("gamepad: holding right trigger at position {}", placement)
            }
            gamepad.inject();
            thread::sleep(Duration::from_millis(rng.gen_range(0..=5000)));
            gamepad.update_right_trigger(0.0);
        }
        _ => {
            if get_config().extra.do_debugging {
                tracing::info!("gamepad: toggling button {}", action)
            }
            gamepad.toggle_button(action);
        }
    }
    gamepad.inject();
}

#[cfg(feature = "advanced")]
#[cfg(target_os = "windows")]
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
            let placement = rng.gen_range(0.0..=1024.0);

            if get_config().extra.do_debugging {
                tracing::info!("pen: updating pressure to {}", placement)
            }
            pen.update_pressure(placement);
        }
        "Rotation" => {
            let placement = rng.gen_range(0.0..=359.0);

            if get_config().extra.do_debugging {
                tracing::info!("pen: updating rotation to {}", placement)
            }
            pen.update_rotation(placement);
        }
        "Tilt" => {
            let placement = (rng.gen_range(-90..=90), rng.gen_range(-90..=90));

            if get_config().extra.do_debugging {
                tracing::info!("pen: updating tilt to {:?}", placement)
            }
            pen.update_tilt(placement);
        }
        "XY_Move" => {
            let display = Screen::from_point(0, 0).unwrap().display_info;
            let placement = (
                rng.gen_range(0..=display.width).try_into().unwrap(),
                rng.gen_range(0..=display.height).try_into().unwrap(),
            );

            if get_config().extra.do_debugging {
                tracing::info!("pen: updating x and y position to {:?}", placement)
            }
            pen.update_position(placement);
        }
        "X_Move" => {
            let display = Screen::from_point(0, 0).unwrap().display_info;
            let placement = rng.gen_range(0..=display.width).try_into().unwrap();

            if get_config().extra.do_debugging {
                tracing::info!("pen: updating x position to {}", placement)
            }
            pen.update_position((placement, -1));
        }
        "Y_Move" => {
            let display = Screen::from_point(0, 0).unwrap().display_info;
            let placement = rng.gen_range(0..=display.height).try_into().unwrap();

            if get_config().extra.do_debugging {
                tracing::info!("pen: updating y position to {}", placement)
            }
            pen.update_position((-1, placement));
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
        #[cfg(target_os = "windows")]
        "gamepad" => gamepad(gamepadobj, &mut rng),
        #[cfg(target_os = "windows")]
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
