use enigo::{Key, Key::Unicode};

/* Keyboard lists */
pub static ALPHANUMERIC_KEYS: &[(Key, usize)] = &[
    (Unicode('A'), 4),
    (Unicode('B'), 4),
    (Unicode('C'), 4),
    (Unicode('D'), 4),
    (Unicode('E'), 4),
    (Unicode('F'), 4),
    (Unicode('G'), 4),
    (Unicode('H'), 4),
    (Unicode('I'), 4),
    (Unicode('J'), 4),
    (Unicode('K'), 4),
    (Unicode('L'), 4),
    (Unicode('M'), 4),
    (Unicode('N'), 8),
    (Unicode('O'), 4),
    (Unicode('P'), 4),
    (Unicode('Q'), 4),
    (Unicode('R'), 4),
    (Unicode('S'), 4),
    (Unicode('T'), 4),
    (Unicode('U'), 4),
    (Unicode('V'), 4),
    (Unicode('W'), 4),
    (Unicode('X'), 4),
    (Unicode('Y'), 4),
    (Unicode('Z'), 4),
    (Unicode('0'), 2),
    (Unicode('1'), 2),
    (Unicode('2'), 2),
    (Unicode('3'), 2),
    (Unicode('4'), 2),
    (Unicode('5'), 2),
    (Unicode('6'), 2),
    (Unicode('7'), 2),
    (Unicode('8'), 2),
    (Unicode('9'), 2),
    (Unicode('`'), 1),
    (Unicode('-'), 1),
    (Unicode('='), 1),
    (Unicode('['), 1),
    (Unicode(']'), 1),
    (Unicode('\\'), 1),
    (Unicode(';'), 1),
    (Unicode('\''), 1),
    (Unicode(','), 1),
    (Unicode('.'), 1),
    (Unicode('/'), 1),
];

pub static FUNCTION_KEYS: &[(Key, usize)] = &[
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
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    (Key::F21, 1),
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    (Key::F22, 1),
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    (Key::F23, 1),
    #[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
    (Key::F24, 1),
    #[cfg(all(unix, not(target_os = "macos")))]
    (Key::F25, 1),
    #[cfg(all(unix, not(target_os = "macos")))]
    (Key::F26, 1),
    #[cfg(all(unix, not(target_os = "macos")))]
    (Key::F27, 1),
    #[cfg(all(unix, not(target_os = "macos")))]
    (Key::F28, 1),
    #[cfg(all(unix, not(target_os = "macos")))]
    (Key::F29, 1),
    #[cfg(all(unix, not(target_os = "macos")))]
    (Key::F30, 1),
    #[cfg(all(unix, not(target_os = "macos")))]
    (Key::F31, 1),
    #[cfg(all(unix, not(target_os = "macos")))]
    (Key::F32, 1),
    #[cfg(all(unix, not(target_os = "macos")))]
    (Key::F33, 1),
    #[cfg(all(unix, not(target_os = "macos")))]
    (Key::F34, 1),
    #[cfg(all(unix, not(target_os = "macos")))]
    (Key::F35, 1),
];

pub static MODIFIER_KEYS: &[(Key, usize)] = &[
    (Key::Control, 1),
    (Key::LControl, 1),
    (Key::RControl, 1),
    (Key::Alt, 1),
    (Key::Shift, 1),
    (Key::LShift, 1),
    (Key::RShift, 1),
];

pub static SPECIAL_KEYS: &[(Key, usize)] = &[
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
    #[cfg(target_os = "windows")]
    (Key::Numlock, 1),
    #[cfg(target_os = "windows")]
    (Key::Numpad0, 1),
    #[cfg(target_os = "windows")]
    (Key::Numpad1, 1),
    #[cfg(target_os = "windows")]
    (Key::Numpad2, 1),
    #[cfg(target_os = "windows")]
    (Key::Numpad3, 1),
    #[cfg(target_os = "windows")]
    (Key::Numpad4, 1),
    #[cfg(target_os = "windows")]
    (Key::Numpad5, 1),
    #[cfg(target_os = "windows")]
    (Key::Numpad6, 1),
    #[cfg(target_os = "windows")]
    (Key::Numpad7, 1),
    #[cfg(target_os = "windows")]
    (Key::Numpad8, 1),
    #[cfg(target_os = "windows")]
    (Key::Numpad9, 1),
    #[cfg(target_os = "windows")]
    (Key::Decimal, 1),
    #[cfg(target_os = "windows")]
    (Key::Add, 1),
    #[cfg(target_os = "windows")]
    (Key::Divide, 1),
    #[cfg(target_os = "windows")]
    (Key::Multiply, 1),
    #[cfg(target_os = "windows")]
    (Key::Subtract, 1),
];

/* Mouse action lists */
pub static MOUSE_CLICKS: [(&str, usize); 6] = [
    ("mouse_down_left", 1),
    ("mouse_down_right", 1),
    ("mouse_down_middle", 1),
    ("mouse_click_left", 5),
    ("mouse_click_right", 5),
    ("mouse_click_middle", 5),
];

pub static MOUSE_MOVE: [(&str, usize); 8] = [
    ("mouse_move_abs", 1),
    ("mouse_move_rel", 1),
    ("mouse_drag_abs_std", 5),
    ("mouse_drag_rel_std", 5),
    ("mouse_drag_abs_fst", 2),
    ("mouse_drag_rel_fst", 3),
    ("mouse_drag_abs_slw", 2),
    ("mouse_drag_rel_slw", 3),
];

pub static MOUSE_SCROLL: [(&str, usize); 3] = [
    ("mouse_scroll_x", 2),
    ("mouse_scroll_y", 2),
    ("mouse_scroll_xy", 1),
];

/* Quotes */
pub static QUOTES_NEGATIVE: [(&str, usize); 11] = [
    ("i don't like you", 1),
    ("you have no friends", 1),
    ("loser", 1),
    ("idiot", 1),
    ("you're a mess", 1),
    ("i hope you die in a fire", 1),
    ("i hate you", 1),
    ("you have a large ass forehead", 1),
    ("bozo", 1),
    ("you talk like a redditor", 1),
    ("are you caseoh junior, mr. fatass?", 1),
];

pub static QUOTES_POSITIVE: [(&str, usize); 5] = [
    ("nice computer you have here", 1),
    ("you seem cool", 1),
    ("wowza", 1),
    ("you're doing great", 1),
    ("nice wifi", 1),
];

pub static QUOTES_QUESTION: [(&str, usize); 7] = [
    ("what graphics card is this?", 1),
    ("what games you got?", 1),
    ("where's your father, huh?", 1),
    ("how does this work?", 1),
    ("were you also abandoned as a child?", 1),
    ("i've won, but at what cost?", 1),
    ("did you know that in terms of male human and female pokémon breeding, vaporeon is the most compatible Pokémon for humans?", 1)
];

pub static QUOTES_STATEMENT: [(&str, usize); 9] = [
    ("i'm bored", 1),
    ("i am chaos", 1),
    ("what a mess", 1),
    ("i'm going to break this computer", 1),
    ("it's wizard time. fireball!", 1),
    ("butter dog", 1),
    ("damn shawty", 1),
    ("WE'RE GONNA CRAAAAAAAAASH", 1),
    ("do a flip!", 1),
];

/* Gamepad action lists */
#[allow(dead_code)]
pub static GAMEPAD_BUTTONS: [(&str, usize); 12] = [
    ("A", 3),
    ("B", 3),
    ("X", 3),
    ("Y", 3),
    ("DPadDown", 1),
    ("DPadLeft", 1),
    ("DPadRight", 1),
    ("DPadUp", 1),
    ("LeftShoulder", 1),
    ("LeftTrigger", 3),
    ("RightShoulder", 1),
    ("RightTrigger", 3),
];

#[allow(dead_code)]
pub static GAMEPAD_MOVE: [(&str, usize); 2] =
    [("LeftThumbstickMove", 3), ("RightThumbstickMove", 1)];

#[allow(dead_code)]
pub static GAMEPAD_SPECIAL: [(&str, usize); 4] = [
    ("LeftThumbstick", 1),
    ("Menu", 1),
    ("RightThumbstick", 1),
    ("View", 1),
];

/* Pen action lists */
#[allow(dead_code)]
pub static PEN_BUTTONS: [(&str, usize); 3] = [("Barrel", 1), ("Eraser", 1), ("Inverted", 1)];

#[allow(dead_code)]
pub static PEN_MOVE: [(&str, usize); 3] = [("XY_Move", 2), ("X_Move", 1), ("Y_Move", 1)];

#[allow(dead_code)]
pub static PEN_SPECIAL: [(&str, usize); 3] = [("Pressure", 1), ("Tilt", 1), ("Rotation", 1)];
