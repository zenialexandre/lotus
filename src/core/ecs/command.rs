use super::{
    world::World,
    entity::Entity,
    component::Component,
    resource::Resource,
    super::{color::Color, managers::rendering::manager::RenderState}
};

/// Enumerator that store the mutable commands allowed in the world.
pub enum Command {
    Spawn(Vec<Box<dyn Component>>),
    Despawn(Entity),
    AddResource(Box<dyn Resource>),
    AddResources(Vec<Box<dyn Resource>>),
    ShowFps(u32, Color)
}

/// Struct to represent the mutable commands to be made on the world.
pub struct Commands {
    pub commands: Vec<Command>
}

impl Commands {
    /// Create a new command struct.
    pub fn new() -> Self {
        return Self {
            commands: Vec::new()
        };
    }

    /// Spawn a new entity on the world.
    ///
    /// The entity will be rendered as its type demands.
    pub fn spawn(&mut self, components: Vec<Box<dyn Component>>) {
        self.commands.push(Command::Spawn(components));
    }

    /// Despawn a specific entity from the world.
    ///
    /// The entity is removed from the rendering flow and its related cached data is cleaned.
    pub fn despawn(&mut self, entity: Entity) {
        self.commands.push(Command::Despawn(entity));
    }

    /// Add a new resource to the world.
    pub fn add_resource(&mut self, resource: Box<dyn Resource>) {
        self.commands.push(Command::AddResource(resource));
    }

    /// Add a list of resources to the world.
    pub fn add_resources(&mut self, resources: Vec<Box<dyn Resource>>) {
        self.commands.push(Command::AddResources(resources));
    }

    /// Show the current FPS value.
    pub fn show_fps(&mut self, current_fps: u32, color: Color) {
        self.commands.push(Command::ShowFps(current_fps, color));
    }

    /// Take the commands memory reference.
    pub(crate) fn _take_commands(&mut self) -> Vec<Command> {
        return std::mem::take(&mut self.commands);
    }

    /// Flush the commands inside the buffer.
    pub fn flush_commands(&mut self, world: &mut World, render_state: &mut RenderState) {
        for command in self.commands.drain(..) {
            match command {
                Command::Spawn(components) => {
                    world.spawn(render_state, components);
                },
                Command::Despawn(entity) => {
                    if world.is_entity_alive(entity) {
                        world.despawn(render_state, &entity);
                    }
                },
                Command::AddResource(resource) => {
                    world.add_resource(resource);   
                },
                Command::AddResources(resources) => {
                    world.add_resources(resources);
                },
                Command::ShowFps(current_fps, color) => {
                    world.show_fps(render_state, current_fps, color);
                }
            }
        }
    }
}
