use gilrs::{EventType, Gilrs};
use super::super::super::{
    super::ResourceRefMut,
    context::Context,
    event::dispatcher::EventDispatcher
};

/// GamepadState struct to manage the gamepad events.
pub(crate) struct GamepadState {
    pub(crate) gilrs: Gilrs
}

impl GamepadState {
    /// Returns a new instance of GamepadState.
    pub(crate) fn new() -> Self {
        return Self {
            gilrs: Gilrs::new().unwrap()
        };
    }

    pub(crate) fn manage(&mut self, context: &mut Context) {
        let mut event_dispatcher: ResourceRefMut<'_, EventDispatcher> = context.world.get_resource_mut::<EventDispatcher>().unwrap();

        if let Some(ev) = self.gilrs.next_event() {
            match ev.event {
                EventType::Connected => {
                    todo!();
                },
                EventType::Disconnected => {
                    todo!();
                },
                EventType::ButtonPressed(button, code) => {
                    todo!();
                },
                EventType::ButtonReleased(button, code) => {
                    todo!();
                },
                _ => ()
            }
        }
    }
}
