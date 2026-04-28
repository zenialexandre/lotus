#[cfg(test)]
pub mod tests {
    use lotus_engine::*;

    mod keyboard {
        use super::*;
        use winit::keyboard::*;

        #[test]
        fn to_winit_test() {
            let key_code: KeyCode = KeyboardKey::to_winit(&KeyboardKey::Abort);
            assert_eq!(key_code, KeyCode::Abort);
        }

        #[test]
        fn from_winit_test() {
            let keyboard_key: KeyboardKey = KeyboardKey::from_winit(&KeyCode::Abort);
            assert_eq!(keyboard_key, KeyboardKey::Abort);
        }
    }

    mod gamepad {
        use super::*;
        use gilrs::*;

        #[test]
        fn to_gilrs_test() {
            let button: Button = GamepadButton::to_gilrs(&GamepadButton::RightBumper);
            assert_eq!(button, Button::RightTrigger);
        }

        #[test]
        fn from_gilrs_test() {
            let gamepad_button: GamepadButton = GamepadButton::from_gilrs(&Button::RightTrigger);
            assert_eq!(gamepad_button, GamepadButton::RightBumper);
        }
    }
}
