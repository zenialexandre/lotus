use gilrs::Button;

/// Enumerator representing the gamepad button for a given button.
///
/// Analogue to the gilrs crate [`Button`](https://docs.rs/gilrs/latest/gilrs/enum.Button.html) enum.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum GamepadButton {
    South,
    East,
    West,
    North,
    LeftBumper,
    RightBumper,
    LeftTrigger,
    RightTrigger,
    LeftJoystick,
    RightJoystick,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    Start,
    Select,
    Mode
}

/// This macro implements the [`to_gilrs`](GamepadButton::to_gilrs) and [`from_gilrs`](GamepadButton::from_gilrs) methods for the [`GamepadButton`](GamepadButton) enum.
macro_rules! impl_translations {
    ($($variant:ident => $gilrs:ident),* $(,)?) => {
        impl GamepadButton {
            /// Translates this [`GamepadButton`] to the corresponding [`Button`] from the gilrs crate.
            pub fn to_gilrs(&self) -> Button {
                match self {
                    $(Self::$variant => Button::$gilrs,)*
                }
            }

            /// Translates this [`GamepadButton`] to the corresponding [`Button`] from the gilrs crate.
            pub fn from_gilrs(button: &Button) -> Self {
                match button {
                    $(Button::$gilrs => Self::$variant,)*
                    _ => panic!("Button not supported")
                }
            }
        }
    };
}

impl_translations!(
    South => South,
    East => East,
    West => West,
    North => North,
    Start => Start,
    Select => Select,
    Mode => Mode,
    DPadUp => DPadUp,
    DPadDown => DPadDown,
    DPadLeft => DPadLeft,
    DPadRight => DPadRight,
    LeftBumper => LeftTrigger,
    RightBumper => RightTrigger,
    LeftTrigger => LeftTrigger2,
    RightTrigger => RightTrigger2,
    LeftJoystick => LeftThumb,
    RightJoystick => RightThumb
);
