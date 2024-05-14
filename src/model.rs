use windows::Gaming::Input::GamepadButtons;
use windows::UI::Input::Preview::Injection::InjectedInputGamepadInfo;
use windows::UI::Input::Preview::Injection::InputInjector;

pub struct GamepadInjector {
    gamepad_state: InjectedInputGamepadInfo,
    injector: InputInjector,
}

impl GamepadInjector {
    pub fn new() -> Self {
        let injector: InputInjector = InputInjector::TryCreate().unwrap();
        injector.InitializeGamepadInjection().unwrap();
        let gamepad_state = InjectedInputGamepadInfo::new().unwrap();
        Self {
            gamepad_state,
            injector,
        }
    }

    pub fn update(
        &mut self,
        buttons: GamepadButtons,
        left_thumbstick: (f64, f64),
        right_thumbstick: (f64, f64),
    ) {
        let _ = self.gamepad_state.SetButtons(buttons);
        let _ = self.gamepad_state.SetLeftThumbstickX(left_thumbstick.0);
        let _ = self.gamepad_state.SetLeftThumbstickY(left_thumbstick.1);
        let _ = self.gamepad_state.SetRightThumbstickX(right_thumbstick.0);
        let _ = self.gamepad_state.SetRightThumbstickY(right_thumbstick.1);
    }

    pub fn update_left_thumbstick(&mut self, left_thumbstick: (f64, f64)) {
        let _ = self.gamepad_state.SetLeftThumbstickX(left_thumbstick.0.clamp(-1.0, 1.0));
        let _ = self.gamepad_state.SetLeftThumbstickY(left_thumbstick.1.clamp(-1.0, 1.0));
    }
    pub fn update_right_thumbstick(&mut self, right_thumbstick: (f64, f64)) {
        let _ = self.gamepad_state.SetRightThumbstickX(right_thumbstick.0.clamp(-1.0, 1.0));
        let _ = self.gamepad_state.SetRightThumbstickY(right_thumbstick.1.clamp(-1.0, 1.0));
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