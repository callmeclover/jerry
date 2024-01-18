use enigo::{Key, KeyboardControllable};

// Alphanumeric (Letters, Numbers, Special ASCII Characters)
let alphanumeric_keys: Vec<Key> = {
    let alphanumeric_chars = "abcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()-=_+[]{}\\|;:'\",.<>/?`~";
    alphanumeric_chars
        .chars()
        .map(|c| Key::Layout(c))
        .filter(|&key| key != Key::Unknown)
        .collect()
};

// Function (F1-F35)
let function_keys: Vec<Key> = vec![
    Key::F1, Key::F2, Key::F3, Key::F4, Key::F5, Key::F6, Key::F7, Key::F8, Key::F9,
    Key::F10, Key::F11, Key::F12, Key::F13, Key::F14, Key::F15, Key::F16, Key::F17, Key::F18, Key::F19,
    Key::F20, Key::F21, Key::F22, Key::F23, Key::F24, Key::F25, Key::F26, Key::F27, Key::F28, Key::F29,
    Key::F30, Key::F31, Key::F32, Key::F33, Key::F34, Key::F35,
]

// Modifiers (Control, Option/Alt, Shift, Meta)
let modifier_keys: Vec<Key> = vec![
    Key::Control, Key::LControl, Key::RControl,
    Key::Alt,
    Key::Shift, Key::LShift, Key::RShift,
    Key::Meta,
];

// Special (Non-Alphanumeric Keys)
let special_keys: Vec<Key> = vec![
    Key::Backspace,
    Key::Begin,
    Key::Break,
    Key::CapsLock,
    Key::Clear,
    Key::Delete,
    Key::DownArrow,
    Key::End,
    Key::Escape,
    Key::Execute,
    Key::Find,
    Key::Help,
    Key::Home,
    Key::Insert,
    Key::Linefeed,
    Key::PageDown,
    Key::PageUp,
    Key::Return,
    Key::Space,
    Key::Tab,
    Key::UpArrow,
];

// Combined List
let combined: Vec<Vec<Key>> = vec![alphanumeric, function_keys, modifiers, special];
