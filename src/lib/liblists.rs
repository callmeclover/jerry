use enigo::{Key, Key::Layout, KeyboardControllable};
use enigo::{Mouse, MouseControllable};
use rand::prelude::*;
use rand::distributions::WeightedIndex;

pub struct MainList {
    pub kb_list: (Vec<Vec<(Key, usize)>>, usize),
    pub ms_list: (Vec<Vec<(&str, usize)>>, usize),
    pub qu_list: (Vec<Vec<(&str, usize)>>, usize),
}

/* Keyboard lists */
let alphanumeric_keys: Vec<(Key, usize)> = vec![
    (Layout('a'), 3), (Layout('b'), 3), (Layout('c'), 3), (Layout('d'), 3), (Layout('e'), 3), (Layout('f'), 3), (Layout('g'), 3), (Layout('h'), 3), (Layout('i'), 3), (Layout('j'), 3), (Layout('k'), 3), (Layout('l'), 3), (Layout('m'), 3), (Layout('n'), 3), (Layout('o'), 3), (Layout('p'), 3), (Layout('q'), 3), (Layout('r'), 3), (Layout('s'), 3), (Layout('t'), 3), (Layout('u'), 3), (Layout('v'), 3), (Layout('w'), 3), (Layout('x'), 3), (Layout('y'), 3), (Layout('z'), 3),
    (Layout('0'), 2), (Layout('1'), 2), (Layout('2'), 2), (Layout('3'), 2), (Layout('4'), 2), (Layout('5'), 2), (Layout('6'), 2), (Layout('7'), 2), (Layout('8'), 2), (Layout('9'), 2),
    (Layout('`'), 1), (Layout('-'), 1), (Layout('='), 1), (Layout('['), 1), (Layout(']'), 1), (Layout('\\'), 1), (Layout(';'), 1), (Layout('\''), 1), (Layout(','), 1), (Layout('.'), 1), (Layout('/'), 1),
];

let function_keys: Vec<(Key, usize)> = vec![
    (Key::F1, 1),
    (Key::F2, 1),
    (Key::F3, 1),
    (Key::F4, 1),
    (Key::F5, 1),
    (Key::F6, 1),
    (Key::F7, 1),
    (Key::F8, 1),
    (Key::F9, 1),
    (Key::F10, 1),
    (Key::F11, 1),
    (Key::F12, 1),
    (Key::F13, 1),
    (Key::F14, 1),
    (Key::F15, 1),
    (Key::F16, 1),
    (Key::F17, 1),
    (Key::F18, 1),
    (Key::F19, 1),
    (Key::F20, 1),
    (Key::F21, 1),
    (Key::F22, 1),
    (Key::F23, 1),
    (Key::F24, 1),
    (Key::F25, 1),
    (Key::F26, 1),
    (Key::F27, 1),
    (Key::F28, 1),
    (Key::F29, 1),
    (Key::F30, 1),
    (Key::F31, 1),
    (Key::F32, 1),
    (Key::F33, 1),
    (Key::F34, 1),
    (Key::F35, 1),
];

let modifier_keys: Vec<(Key, usize)> = vec![
    (Key::Control, 1), (Key::LControl, 1), (Key::RControl, 1),
    (Key::Alt, 1),
    (Key::Shift, 2), (Key::LShift, 2), (Key::RShift, 2),
];

let special_keys: Vec<(Key, usize)> = vec![
    (Key::Backspace, 1),
    (Key::Begin, 1),
    (Key::Meta, 1),
    (Key::Break, 1),
    (Key::CapsLock, 1),
    (Key::Clear, 1),
    (Key::Delete, 1),
    (Key::DownArrow, 1),
    (Key::End, 1),
    (Key::Escape, 1),
    (Key::Execute, 1),
    (Key::Find, 1),
    (Key::Help, 1),
    (Key::Home, 1),
    (Key::Insert, 1),
    (Key::Linefeed, 1),
    (Key::PageDown, 1),
    (Key::PageUp, 1),
    (Key::Return, 1),
    (Key::Space, 1),
    (Key::Tab, 1),
    (Key::UpArrow, 1),
];

/* Mouse action lists */
let mouse_clicks: Vec<(&str, usize)> = vec![
    ("mouse_down_left", 1), ("mouse_down_right", 1), ("mouse_down_middle", 1), ("mouse_down_back", 1), ("mouse_down_forward", 1), ("mouse_down_scrollup", 1), ("mouse_down_scrolldown", 1), ("mouse_down_scrollright", 1), ("mouse_down_scrollleft", 1),
    ("mouse_up_left", 1), ("mouse_up_right", 1), ("mouse_up_middle", 1), ("mouse_up_back", 1), ("mouse_up_forward", 1), ("mouse_up_scrollup", 1), ("mouse_up_scrolldown", 1), ("mouse_up_scrollright", 1), ("mouse_up_scrollleft", 1),
    ("mouse_click_left", 1), ("mouse_click_right", 1), ("mouse_click_middle", 1), ("mouse_click_back", 1), ("mouse_click_forward", 1), ("mouse_click_scrollup", 1), ("mouse_click_scrolldown", 1), ("mouse_click_scrollright", 1), ("mouse_click_scrollleft", 1),
];

let mouse_move: Vec<(&str, usize)> = vec![
    ("mouse_move_abs", 1), ("mouse_move_rel", 2), 
];

let mouse_scroll: Vec<(&str, usize)> = vec![
    ("mouse_scroll_x", 2), ("mouse_scroll_y", 2), ("mouse_scroll_xy", 1)
];

/* Quotes */
let quotes_negative: Vec<(&str, usize)> = vec![
    ("i don't like you", 1),
    ("you have no friends", 1),
    ("loser", 1),
    ("idiot", 1),
    ("you're a mess", 1),
];

let quotes_positive: Vec<(&str, usize)> = vec![
    ("nice computer you have here", 1),
    ("you seem cool", 1),
    ("wowza", 1),
    ("you're doing great", 1),
];

let quotes_question: Vec<(&str, usize)> = vec![
    ("what graphics card is this?", 1),
    ("what games you got?", 1),
    ("where's your father?", 1),
    ("how does this work?", 1),
];

let quotes_statement: Vec<(&str, usize)> = vec![
    ("i'm bored", 1),
    ("i am chaos", 1),
    ("what a mess", 1),
    ("i'm going to break this computer", 1),
];

// Combined Lists
pub const kb_list: Vec<Vec<(Key, usize)>> = vec![alphanumeric_keys, function_keys, modifier_keys, special_keys];
pub const ms_list: Vec<Vec<(&str, usize)>> = vec![mouse_clicks, mouse_move, mouse_scroll];
pub const qu_list: Vec<Vec<(&str, usize)>> = vec![quotes_negative, quotes_positive, quotes_question, quotes_statement];

// Mega list
pub const main_list: MainList = MainList((kb_list, 7), (ms_list, 6), (qu_list, 1))