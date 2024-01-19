use std::any::Any;
use enigo::{ Key, Key::Layout };

/* Keyboard lists */
pub static ALPHANUMERIC_KEYS: [(Key, usize); 47] = [
    (Key::A, 3),
    (Key::B, 3),
    (Key::C, 3),
    (Key::D, 3),
    (Key::E, 3),
    (Key::F, 3),
    (Key::G, 3),
    (Key::H, 3),
    (Key::I, 3),
    (Key::J, 3),
    (Key::K, 3),
    (Key::L, 3),
    (Key::M, 3),
    (Key::N, 3),
    (Key::O, 3),
    (Key::P, 3),
    (Key::Q, 3),
    (Key::R, 3),
    (Key::S, 3),
    (Key::T, 3),
    (Key::U, 3),
    (Key::V, 3),
    (Key::W, 3),
    (Key::X, 3),
    (Key::Y, 3),
    (Key::Z, 3),
    (Key::Num0, 2),
    (Key::Num1, 2),
    (Key::Num2, 2),
    (Key::Num3, 2),
    (Key::Num4, 2),
    (Key::Num5, 2),
    (Key::Num6, 2),
    (Key::Num7, 2),
    (Key::Num8, 2),
    (Key::Num9, 2),
    (Layout('`'), 1),
    (Layout('-'), 1),
    (Layout('='), 1),
    (Layout('['), 1),
    (Layout(']'), 1),
    (Layout('\\'), 1),
    (Layout(';'), 1),
    (Layout('\''), 1),
    (Layout(','), 1),
    (Layout('.'), 1),
    (Layout('/'), 1)
];

pub static FUNCTION_KEYS: [(Key, usize); 24] = [
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
    (Key::F24, 1)
];

pub static MODIFIER_KEYS: [(Key, usize); 8] = [
    (Key::Control, 1),
    (Key::LControl, 1),
    (Key::RControl, 1),
    (Key::Alt, 1),
    (Key::CapsLock, 1),
    (Key::Shift, 1),
    (Key::LShift, 1),
    (Key::RShift, 1)
];

pub static SPECIAL_KEYS: [(Key, usize); 36] = [
    (Key::Backspace, 1),
    (Key::Meta, 1),
    (Key::Clear, 1),
    (Key::Delete, 1),
    (Key::End, 1),
    (Key::Escape, 1),
    (Key::Execute, 1),
    (Key::Help, 1),
    (Key::Home, 1),
    (Key::Insert, 1),
    (Key::PageDown, 1),
    (Key::PageUp, 1),
    (Key::Return, 1),
    (Key::Space, 1),
    (Key::Tab, 1),
    (Key::UpArrow, 1),
    (Key::DownArrow, 1),
    (Key::LeftArrow, 1),
    (Key::RightArrow, 1),
    (Key::Numlock, 1),
    (Key::Numpad0, 1),
    (Key::Numpad1, 1),
    (Key::Numpad2, 1),
    (Key::Numpad3, 1),
    (Key::Numpad4, 1),
    (Key::Numpad5, 1),
    (Key::Numpad6, 1),
    (Key::Numpad7, 1),
    (Key::Numpad8, 1),
    (Key::Numpad9, 1),
    (Key::Add, 1),
    (Key::Decimal, 1),
    (Key::Divide, 1),
    (Key::Return, 1),
    (Key::Multiply, 1),
    (Key::Subtract, 1)
];

/* Mouse action lists */
pub static MOUSE_CLICKS: [(&str, usize); 27] = [
    ("mouse_down_left", 1),
    ("mouse_down_right", 1),
    ("mouse_down_middle", 1),
    ("mouse_down_back", 1),
    ("mouse_down_forward", 1),
    ("mouse_down_scrollup", 1),
    ("mouse_down_scrolldown", 1),
    ("mouse_down_scrollright", 1),
    ("mouse_down_scrollleft", 1),
    ("mouse_up_left", 1),
    ("mouse_up_right", 1),
    ("mouse_up_middle", 1),
    ("mouse_up_back", 1),
    ("mouse_up_forward", 1),
    ("mouse_up_scrollup", 1),
    ("mouse_up_scrolldown", 1),
    ("mouse_up_scrollright", 1),
    ("mouse_up_scrollleft", 1),
    ("mouse_click_left", 1),
    ("mouse_click_right", 1),
    ("mouse_click_middle", 1),
    ("mouse_click_back", 1),
    ("mouse_click_forward", 1),
    ("mouse_click_scrollup", 1),
    ("mouse_click_scrolldown", 1),
    ("mouse_click_scrollright", 1),
    ("mouse_click_scrollleft", 1)
];

pub static MOUSE_MOVE: [(&str, usize); 2] = [("mouse_move_abs", 1), ("mouse_move_rel", 2)];

pub static MOUSE_SCROLL: [(&str, usize); 3] = [
    ("mouse_scroll_x", 2),
    ("mouse_scroll_y", 2),
    ("mouse_scroll_xy", 1)
];

/* Quotes */
pub static QUOTES_NEGATIVE: [(&str, usize); 5] = [
    ("i don't like you", 1),
    ("you have no friends", 1),
    ("loser", 1),
    ("idiot", 1),
    ("you're a mess", 1)
];

pub static QUOTES_POSITIVE: [(&str, usize); 4] = [
    ("nice computer you have here", 1),
    ("you seem cool", 1),
    ("wowza", 1),
    ("you're doing great", 1)
];

pub static QUOTES_QUESTION: [(&str, usize); 4] = [
    ("what graphics card is this?", 1),
    ("what games you got?", 1),
    ("where's your father?", 1),
    ("how does this work?", 1)
];

pub static QUOTES_STATEMENT: [(&str, usize); 4] = [
    ("i'm bored", 1),
    ("i am chaos", 1),
    ("what a mess", 1),
    ("i'm going to break this computer", 1)
];

/* Gamepad action lists */
pub static GAMEPAD_BUTTONS: [(Key, usize); 12] = [
    (Key::GamepadA, 2),
    (Key::GamepadB, 2),
    (Key::GamepadDPadDown, 1),
    (Key::GamepadDPadLeft, 1),
    (Key::GamepadDPadRight, 1),
    (Key::GamepadDPadUp, 1),
    (Key::GamepadLeftShoulder, 1),
    (Key::GamepadLeftTrigger, 1),
    (Key::GamepadRightShoulder, 1),
    (Key::GamepadRightTrigger, 1),
    (Key::GamepadX, 2),
    (Key::GamepadY, 2)
];

pub static GAMEPAD_MOVE: [(Key, usize); 8] = [
    (Key::GamepadLeftThumbstickDown, 1),
    (Key::GamepadLeftThumbstickLeft, 1),
    (Key::GamepadLeftThumbstickRight, 1),
    (Key::GamepadLeftThumbstickUp, 1),
    (Key::GamepadRightThumbstickDown, 1),
    (Key::GamepadRightThumbstickLeft, 1),
    (Key::GamepadRightThumbstickRight, 1),
    (Key::GamepadRightThumbstickUp, 1)
];

pub static GAMEPAD_SPECIAL: [(Key, usize); 4] = [
    (Key::GamepadLeftThumbstickButton, 1),
    (Key::GamepadMenu, 1),
    (Key::GamepadRightThumbstickButton, 1),
    (Key::GamepadView, 1)
];
