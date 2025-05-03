use std::{collections::VecDeque, mem::take};
use cgmath::Vector2;
use lotus_proc_macros::Resource;
use super::ecs::entity::Entity;

#[derive(Clone, PartialEq)]
pub(crate) enum EventType {
    UpdatePixelatedPosition,
    UpdatePixelatedScale
}

#[derive(Clone, PartialEq)]
pub(crate) struct Event {
    pub(crate) entity: Entity,
    pub(crate) event_type: EventType,
    pub(crate) value: Vector2<f32>
}

impl Event {
    pub(crate) fn new(entity: Entity, event_type: EventType, value: Vector2<f32>) -> Self {
        return Self {
            entity,
            event_type,
            value
        };
    }
}

#[derive(Clone, Resource)]
pub(crate) struct EventDispatcher {
    pub(crate) events: VecDeque<Event>
}

impl EventDispatcher {
    pub(crate) fn new() -> Self {
        return Self {
            events: VecDeque::new()
        };
    }

    pub(crate) fn send(&mut self, event: Event) {
        self.events.push_front(event);
    }

    pub(crate) fn drain(&mut self) -> VecDeque<Event> {
        return take(&mut self.events);
    }
}
