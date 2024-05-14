use windows::Gaming::Input::GamepadButtons;
use windows::UI::Input::Preview::Injection::InjectedInputGamepadInfo;
use windows::UI::Input::Preview::Injection::InputInjector;
use std::collections::HashMap;

pub struct GamepadInjector {
    gamepad_state: InjectedInputGamepadInfo,
    injector: InputInjector,
    abs_buttons: HashMap<String, bool>
}

impl GamepadInjector {
    pub fn new() -> Self {
        let abs_buttons: HashMap<String, bool> = HashMap::from([
            ("A".to_string(), false),
            ("B".to_string(), false),
            ("DPadDown".to_string(), false),
            ("DPadLeft".to_string(), false),
            ("DPadRight".to_string(), false),
            ("DPadUp".to_string(), false),
            ("LeftShoulder".to_string(), false),
            ("LeftThumbstick".to_string(), false),
            ("Menu".to_string(), false),
            ("RightShoulder".to_string(), false),
            ("RightThumbstick".to_string(), false),
            ("View".to_string(), false),
            ("X".to_string(), false),
            ("Y".to_string(), false),
        ]);
        let injector: InputInjector = InputInjector::TryCreate().unwrap();
        injector.InitializeGamepadInjection().unwrap();
        let gamepad_state = InjectedInputGamepadInfo::new().unwrap();
        Self {
            gamepad_state,
            injector,
            abs_buttons
        }
    }

    pub fn buttons(&self) -> GamepadButtons {
        self.gamepad_state.Buttons().unwrap()
    }

    pub fn toggle_button(&mut self, button: &str) {
        if self.abs_buttons.contains_key(button) {
            if !self.abs_buttons.get(button).unwrap() {
                self.abs_buttons.insert(button.to_string(), true);
                let mut buttons = self.buttons();
                buttons |= get_value_of_button(button);
                self.update_buttons(buttons);
            } else {
                self.abs_buttons.insert(button.to_string(), false);
                let mut buttons = self.buttons();
                buttons |= get_value_of_button(button);
                self.update_buttons(buttons);
            }
        }
    }

    pub fn update_left_thumbstick(&mut self, left_thumbstick: (f64, f64)) {
        let _ = self.gamepad_state.SetLeftThumbstickX(left_thumbstick.0.clamp(-1.0, 1.0));
        let _ = self.gamepad_state.SetLeftThumbstickY(left_thumbstick.1.clamp(-1.0, 1.0));
    }
    pub fn update_right_thumbstick(&mut self, right_thumbstick: (f64, f64)) {
        let _ = self.gamepad_state.SetRightThumbstickX(right_thumbstick.0.clamp(-1.0, 1.0));
        let _ = self.gamepad_state.SetRightThumbstickY(right_thumbstick.1.clamp(-1.0, 1.0));
    }
    pub fn update_left_trigger(&mut self, left_trigger: f64) {
        let _ = self.gamepad_state.SetLeftTrigger(left_trigger.clamp(0.0, 1.0));
    }
    pub fn update_right_trigger(&mut self, right_trigger: f64) {
        let _ = self.gamepad_state.SetRightTrigger(right_trigger.clamp(0.0, 1.0));
    }
    pub fn update_buttons(&mut self, buttons: GamepadButtons) {
        let _ = self.gamepad_state.SetButtons(buttons);
    }

    pub fn inject(&mut self) {
        self.injector
            .InjectGamepadInput(&self.gamepad_state)
            .unwrap();
    }
}

impl Drop for GamepadInjector {
    fn drop(&mut self) {
        self.injector.UninitializeGamepadInjection().unwrap();
    }
}

fn get_value_of_button(button: &str) -> GamepadButtons {
    match button {
        "A" => GamepadButtons::A,
        "B" => GamepadButtons::B,
        "X" => GamepadButtons::X,
        "Y" => GamepadButtons::Y,
        "DPadUp" => GamepadButtons::DPadUp,
        "DPadDown" => GamepadButtons::DPadDown,
        "DPadLeft" => GamepadButtons::DPadLeft,
        "DPadRight" => GamepadButtons::DPadRight,
        "Menu" => GamepadButtons::Menu,
        "View" => GamepadButtons::View,
        "LeftThumbstick" => GamepadButtons::LeftThumbstick,
        "LeftShoulder" => GamepadButtons::LeftShoulder,
        "RightThumbstick" => GamepadButtons::RightThumbstick,
        "RightShoulder" => GamepadButtons::RightShoulder,
        _ => GamepadButtons(0u32)
    }
}