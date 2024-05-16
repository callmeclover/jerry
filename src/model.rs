use windows::Gaming::Input::GamepadButtons;
use windows::UI::Input::Preview::Injection::{
    InputInjector,
    InjectedInputGamepadInfo,
    InjectedInputPenInfo,
    InjectedInputPenButtons
};
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
                buttons |= model::GamepadInjector::get_value_of_button(button);
                self.update_buttons(buttons);
            } else {
                self.abs_buttons.insert(button.to_string(), false);
                let mut buttons = self.buttons();
                buttons |= model::GamepadInjector::get_value_of_button(button);
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
}

impl Drop for GamepadInjector {
    fn drop(&mut self) {
        self.injector.UninitializeGamepadInjection().unwrap();
    }
}

pub struct PenInjector {
    pen_state: InjectedInputPenInfo,
    injector: InputInjector,
    abs_buttons: HashMap<String, bool>
}

impl PenInjector {
    pub fn new() -> Self {
        let abs_buttons: HashMap<String, bool> = HashMap::from([
            ("Barrel".to_string(), false),
            ("Eraser".to_string(), false),
            ("Inverted".to_string(), false),
        ]);
        let injector: InputInjector = InputInjector::TryCreate().unwrap();
        injector.InitializePenInjection(Default::default()).unwrap();
        let pen_state = InjectedInputPenInfo::new().unwrap();
        Self {
            pen_state,
            injector,
            abs_buttons
        }
    }

    pub fn buttons(&self) -> InjectedInputPenButtons {
        self.pen_state.PenButtons().unwrap()
    }

    pub fn update_buttons(&mut self, buttons: InjectedInputPenButtons) {
        let _ = self.pen_state.SetPenButtons(buttons);
    }

    pub fn toggle_button(&mut self, button: &str) {
        if self.abs_buttons.contains_key(button) {
            if !self.abs_buttons.get(button).unwrap() {
                self.abs_buttons.insert(button.to_string(), true);
                let mut buttons = self.buttons();
                buttons |= model::PenInjector::get_value_of_button(button);
                self.update_buttons(buttons);
            } else {
                self.abs_buttons.insert(button.to_string(), false);
                let mut buttons = self.buttons();
                buttons |= model::PenInjector::get_value_of_button(button);
                self.update_buttons(buttons);
            }
        }
    }

    pub fn update_tilt(&mut self, tilt: (i32, i32)) {
        let _ = self.pen_state.SetTiltX(tilt.0.clamp(-90, 90));
        let _ = self.pen_state.SetTiltY(tilt.1.clamp(-90, 90));
    }

    pub fn update_rotation(&mut self, rotation: f64) {
        let _ = self.pen_state.SetRotation(rotation.clamp(0.0, 359.0));
    }

    pub fn update_pressure(&mut self, pressure: f64) {
        let _ = self.pen_state.SetPressure(pressure.clamp(0.0, 1024.0));
    }

    pub fn update_position(&mut self, position: (i32,i32)) {
        let mut info = self.pen_state.PointerInfo().unwrap();
        if !position.0==-1 { info.PixelLocation.PositionX = position.0; }
        if !position.1==-1 { info.PixelLocation.PositionY = position.1; }
        let _ = self.pen_state.SetPointerInfo(info);
    }

    pub fn inject(&mut self) {
        self.injector
            .InjectPenInput(&self.pen_state)
            .unwrap();
    }

    fn get_value_of_button(button: &str) -> InjectedInputPenButtons {
        match button {
            "Barrel" => InjectedInputPenButtons::Barrel,
            "Eraser" => InjectedInputPenButtons::Eraser,
            "Inverted" => InjectedInputPenButtons::Inverted,
            _ => InjectedInputPenButtons(0u32)
        }
    }
}

impl Drop for PenInjector {
    fn drop(&mut self) {
        self.injector.UninitializePenInjection().unwrap();
    }
}