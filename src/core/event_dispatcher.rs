use std::{any::Any, collections::VecDeque, mem::take};
use lotus_proc_macros::Resource;
use super::ecs::entity::Entity;

/// Enumerator to represent the different types of MACRO events.
#[derive(Clone, PartialEq)]
pub(crate) enum EventType {
    Transform(SubEventType),
    Text(SubEventType)
}

/// Enumerator to represent the different types of MICRO events.
#[derive(Clone, PartialEq)]
pub(crate) enum SubEventType {
    UpdatePixelatedPosition,
    UpdatePixelatedScale,
    UpdateTextFont,
    UpdateTextPosition,
    UpdateTextContent,
    UpdateTextColor
}

/// Struct to represent an event to be dispatched.
pub(crate) struct Event {
    pub(crate) entity: Entity,
    pub(crate) event_type: EventType,
    pub(crate) value: Box<dyn Any + Send + Sync>
}

impl Event {
    /// Create a new event struct.
    pub(crate) fn new<T: Any + Send + Sync>(entity: Entity, event_type: EventType, value: T) -> Self {
        return Self {
            entity,
            event_type,
            value: Box::new(value)
        };
    }

    pub(crate) fn get<T: Any>(&self) -> Option<&T> {
        self.value.downcast_ref::<T>()
    }
}

/// Struct to represent the event dispatcher.
#[derive(Resource)]
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
