use winit::{
    application::ApplicationHandler,
    event::{WindowEvent, ElementState},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId, WindowButtons, Icon},
    dpi::{Size, Position, LogicalSize, LogicalPosition}
};
use std::{path::Path, sync::Arc};

use super::{
    rendering_manager::RenderState,
    super::{
        camera::camera2d::Camera2d,
        ecs::{world::World, resource::{ResourceRef, ResourceRefMut}},
        color::Color,
        game_loop::GameLoop,
        engine::Context,
        input::Input
    }
};

/// Struct to facilitate the window configuration by the end-user.
#[derive(Clone)]
pub struct WindowConfiguration {
    pub icon_path: String,
    pub title: String,
    pub background_color: Option<Color>,
    pub background_image_path: Option<String>,
    pub width: f64,
    pub height: f64,
    pub position_x: f64,
    pub position_y: f64,
    pub resizable: bool,
    pub decorations: bool,
    pub transparent: bool,
    pub visible: bool,
    pub active: bool,
    pub enabled_buttons: WindowButtons
}

impl Default for WindowConfiguration {
    /// Returns a default configuration.
    fn default() -> Self {
        return Self {
            icon_path: "assets/textures/lotus_pink_256x256.png".to_string(),
            title: "New Game!".to_string(),
            background_color: Some(Color::WHITE),
            background_image_path: None,
            width: 800.0,
            height: 600.0,
            position_x: 100.0,
            position_y: 100.0,
            resizable: true,
            decorations: true,
            transparent: true,
            visible: true,
            active: true,
            enabled_buttons: WindowButtons::all()
        };
    }
}

impl WindowConfiguration {
    /// Returns a icon by its relative file path.
    pub fn get_icon_by_path(icon_path: String) -> Option<Icon> {
        if let Ok(image) = image::open(Path::new(icon_path.as_str())) {
            let icon_image = image.into_rgba8();
            let (icon_width, icon_height): (u32, u32) = icon_image.dimensions();
            let icon_rgba: Vec<u8> = icon_image.into_raw();
            let icon: Icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
            return Some(icon);
        };
        return None;
    }
}

struct Application {
    window: Option<Arc<Window>>,
    window_configuration: Option<WindowConfiguration>,
    context: Option<Context>,
    game_loop: GameLoop
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut color: Option<Color> = None;
        let mut background_image_path: Option<String> = None;

        let window: Arc<Window> = if let Some(window_configuration) = &self.window_configuration {
            let mut window_attributes: WindowAttributes = Window::default_attributes();
            window_attributes.title = window_configuration.title.clone();
            window_attributes.inner_size =  Some(Size::Logical(LogicalSize::new(
                window_configuration.width,
                window_configuration.height,
            )));
            window_attributes.position = Some(Position::Logical(LogicalPosition::new(
                window_configuration.position_x,
                window_configuration.position_y
            )));
            window_attributes.resizable = window_configuration.resizable;
            window_attributes.decorations = window_configuration.decorations;
            window_attributes.transparent = window_configuration.transparent;
            window_attributes.visible = window_configuration.visible;
            window_attributes.active = window_configuration.active;
            window_attributes.enabled_buttons = window_configuration.enabled_buttons;
            window_attributes.window_icon = WindowConfiguration::get_icon_by_path(window_configuration.icon_path.clone());

            if let Some(background_color) = window_configuration.background_color {
                color = Some(background_color);
            }

            if let Some(background_image_path_unwrapped) = &window_configuration.background_image_path {
                background_image_path = Some(background_image_path_unwrapped.to_string());
            }
            Arc::new(event_loop.create_window(window_attributes).unwrap())
        } else {
            Arc::new(event_loop.create_window(Window::default_attributes()).unwrap())
        };
        self.window = Some(window.clone());

        let world: World = World::new();
        let mut render_state: RenderState = pollster::block_on(RenderState::new(window));
        render_state.color = color;
        render_state.background_image_path = background_image_path;

        self.context = Some(Context::new(
            render_state,
            world,
            self.window_configuration.as_ref().unwrap().clone(),
            0.
        ));
        (self.game_loop.setup)(self.context.as_mut().unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, window_event: WindowEvent) {
        let context: &mut Context = &mut self.context.as_mut().unwrap();
        let render_state: &mut RenderState = &mut context.render_state;
        let camera2d: ResourceRef<'_, Camera2d> = context.world.get_resource::<Camera2d>().unwrap();
        let mut input: ResourceRefMut<'_, Input> = context.world.get_resource_mut::<Input>().unwrap();

        if !render_state.input(&window_event) {
            match window_event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                },
                WindowEvent::Resized(new_size) => {
                    render_state.resize(new_size, &camera2d);
                },
                WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => {
                    if ElementState::Pressed == event.state {
                        input.pressed_keys.insert(event.physical_key);
                    } else if ElementState::Released == event.state {
                        input.pressed_keys.remove(&event.physical_key);
                    }
                },
                WindowEvent::MouseInput { device_id: _, state, button } => {
                    if ElementState::Pressed == state {
                        input.pressed_mouse_buttons.insert(button);
                    } else if ElementState::Released == state {
                        input.pressed_mouse_buttons.remove(&button);
                    }
                },
                WindowEvent::CursorMoved { device_id: _, position } => {
                    input.mouse_position.0 = position.x as f32;
                    input.mouse_position.1 = position.y as f32;
                },
                WindowEvent::RedrawRequested => {
                    render_state.window().request_redraw();
                    self.game_loop.render(&mut context.render_state, &context.world, event_loop);
                }
                _ => ()
            }
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.game_loop.run(self.context.as_mut().unwrap(), event_loop);
    }
}

/// Initialize the engine/application asynchronously.
pub async fn initialize_application(
    window_configuration: Option<WindowConfiguration>,
    setup: fn(context: &mut Context),
    update: fn(context: &mut Context)
) {
    env_logger::init();
    let event_loop: EventLoop<()> = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut application: Application = if let Some(window_configuration_unwrapped) = window_configuration {
        Application {
            window: None,
            context: None,
            window_configuration: Some(window_configuration_unwrapped),
            game_loop: GameLoop::new(setup, update)
        }
    } else {
        Application {
            window: None,
            context: None,
            window_configuration: None,
            game_loop: GameLoop::new(setup, update)
        }
    };
    let _ = event_loop.run_app(&mut application);
}
