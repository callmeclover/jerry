use enigo::{ Key, Key::Layout };

/* Keyboard lists */
pub static ALPHANUMERIC_KEYS: [(Key, usize); 47] = [
    (Key::A, 4),
    (Key::B, 4),
    (Key::C, 4),
    (Key::D, 4),
    (Key::E, 4),
    (Key::F, 4),
    (Key::G, 4),
    (Key::H, 4),
    (Key::I, 4),
    (Key::J, 4),
    (Key::K, 4),
    (Key::L, 4),
    (Key::M, 4),
    (Key::N, 4),
    (Key::O, 4),
    (Key::P, 4),
    (Key::Q, 4),
    (Key::R, 4),
    (Key::S, 4),
    (Key::T, 4),
    (Key::U, 4),
    (Key::V, 4),
    (Key::W, 4),
    (Key::X, 4),
    (Key::Y, 4),
    (Key::Z, 4),
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

pub static MODIFIER_KEYS: [(Key, usize); 7] = [
    (Key::Control, 1),
    (Key::LControl, 1),
    (Key::RControl, 1),
    (Key::Alt, 1),
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
    (Key::CapsLock, 1),
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

    ("mouse_click_left", 5),
    ("mouse_click_right", 5),
    ("mouse_click_middle", 5),
    ("mouse_click_back", 5),
    ("mouse_click_forward", 5),
    ("mouse_click_scrollup", 5),
    ("mouse_click_scrolldown", 5),
    ("mouse_click_scrollright", 5),
    ("mouse_click_scrollleft", 5)
];

pub static MOUSE_MOVE: [(&str, usize); 8] = [
    ("mouse_move_abs", 1), 
    ("mouse_move_rel", 1), 
    ("mouse_drag_abs_std", 5), 
    ("mouse_drag_rel_std", 5), 
    ("mouse_drag_abs_fst", 2),
    ("mouse_drag_rel_fst", 3),
    ("mouse_drag_abs_slw", 2), 
    ("mouse_drag_rel_slw", 3)
];

pub static MOUSE_SCROLL: [(&str, usize); 3] = [
    ("mouse_scroll_x", 2),
    ("mouse_scroll_y", 2),
    ("mouse_scroll_xy", 1)
];

/* Quotes */
pub static QUOTES_NEGATIVE: [(&str, usize); 8] = [
    ("i don't like you", 1),
    ("you have no friends", 1),
    ("loser", 1),
    ("idiot", 1),
    ("you're a mess", 1),
    ("i hope you die in a fire", 1),
    ("i hate you", 1),
    ("you have a large ass forehead", 1)
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
    ("where's your father, huh?", 1),
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
    (Key::GamepadA, 3),
    (Key::GamepadB, 3),
    (Key::GamepadDPadDown, 1),
    (Key::GamepadDPadLeft, 1),
    (Key::GamepadDPadRight, 1),
    (Key::GamepadDPadUp, 1),
    (Key::GamepadLeftShoulder, 1),
    (Key::GamepadLeftTrigger, 2),
    (Key::GamepadRightShoulder, 1),
    (Key::GamepadRightTrigger, 2),
    (Key::GamepadX, 3),
    (Key::GamepadY, 3)
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