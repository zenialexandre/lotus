use gilrs::Gilrs;
use super::super::super::{
    super::ResourceRefMut,
    ecs::world::World,
    event::dispatcher::{EventDispatcher, Event, EventType, SubEventType}
};

/// Struct to manage the gamepad events.
pub struct GamepadListener {
    pub(crate) gilrs: Gilrs,
    pub enabled: bool
}

impl GamepadListener {
    /// Returns a new instance of GamepadListener.
    pub(crate) fn new() -> Self {
        return Self {
            gilrs: Gilrs::new().unwrap(),
            enabled: false
        };
    }

    /// Enable Gamepad functionalities.
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable Gamepad functionalities.
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Manage Gamepad events and send them to the dispatcher.
    pub(crate) fn manage(&mut self, world: &mut World) {
        let mut event_dispatcher: ResourceRefMut<'_, EventDispatcher> = world.get_resource_mut::<EventDispatcher>().unwrap();

        while let Some(gilrs::Event {id, event, ..}) = self.gilrs.next_event() {
            match event {
                gilrs::EventType::Connected => {
                    event_dispatcher.send(Event::new_with_dummy(EventType::Gamepad(SubEventType::GamepadConnected), id));
                },
                gilrs::EventType::Disconnected => {
                    event_dispatcher.send(Event::new_with_dummy(EventType::Gamepad(SubEventType::GamepadDisconnected), id));
                },
                gilrs::EventType::ButtonPressed(button, _code) => {
                    event_dispatcher.send(Event::new_with_dummy(EventType::Gamepad(SubEventType::GamepadButtonPressed), (id, button)));
                },
                gilrs::EventType::ButtonReleased(button, _code) => {
                    event_dispatcher.send(Event::new_with_dummy(EventType::Gamepad(SubEventType::GamepadButtonReleased), (id, button)));
                },
                gilrs::EventType::AxisChanged(axis, direction, _code) => {
                    event_dispatcher.send(Event::new_with_dummy(EventType::Gamepad(SubEventType::GamepadAxisChanged), (id, axis, direction)))
                }
                _ => ()
            }
        }
    }
}
