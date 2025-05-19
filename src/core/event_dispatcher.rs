use std::{collections::VecDeque, mem::take};
use cgmath::Vector2;
use lotus_proc_macros::Resource;
use super::ecs::entity::Entity;

/// Enumerator to represent the different types of events.
#[derive(Clone, PartialEq)]
pub(crate) enum EventType {
    UpdatePixelatedPosition,
    UpdatePixelatedScale
}

/// Struct to represent an event to be dispatched.
#[derive(Clone, PartialEq)]
pub(crate) struct Event {
    pub(crate) entity: Entity,
    pub(crate) event_type: EventType,
    pub(crate) value: Vector2<f32>
}

impl Event {
    /// Create a new event struct.
    pub(crate) fn new(entity: Entity, event_type: EventType, value: Vector2<f32>) -> Self {
        return Self {
            entity,
            event_type,
            value
        };
    }
}

/// Struct to represent the event dispatcher.
#[derive(Clone, Resource)]
pub(crate) struct EventDispatcher {
    pub(crate) events: VecDeque<Event>
}

impl EventDispatcher {
    /// Create a new event dispatcher struct.
    pub(crate) fn new() -> Self {
        return Self {
            events: VecDeque::new()
        };
    }

    /// Send a event to be dispatched.
    pub(crate) fn send(&mut self, event: Event) {
        self.events.push_front(event);
    }

    /// Drain the events from the dispatching queue.
    pub(crate) fn drain(&mut self) -> VecDeque<Event> {
        return take(&mut self.events);
    }
}
