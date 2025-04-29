use wgpu::PresentMode;
use winit::{
    application::ApplicationHandler,
    event::{WindowEvent, ElementState},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId, WindowButtons, Icon},
    dpi::{Size, Position, LogicalSize, LogicalPosition}
};
use std::sync::Arc;

use super::super::{
    rendering::manager::RenderState,
    super::{
        asset_loader::AssetLoader,
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
    //pub visible: bool,
    pub active: bool,
    pub enabled_buttons: WindowButtons,
    pub present_mode: PresentMode
}

impl Default for WindowConfiguration {
    /// Returns a default configuration.
    fn default() -> Self {
        return Self {
            icon_path: "".to_string(),
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
            //visible: false,
            active: true,
            enabled_buttons: WindowButtons::all(),
            present_mode: PresentMode::AutoNoVsync
        };
    }
}

impl WindowConfiguration {
    /// Returns the window configuration with the icon as bytes.
    pub fn icon_path(self, icon_path: String) -> Self {
        return Self {
            icon_path,
            ..self
        };
    }

    /// Returns the window configuration with the title.
    pub fn title(self, title: String) -> Self {
        return Self {
            title,
            ..self
        };
    }

    /// Returns the window configuration with the background color.
    pub fn background_color(self, background_color: Option<Color>) -> Self {
        return Self {
            background_color,
            ..self
        };
    }

    /// Returns the window configuration with the background image.
    pub fn background_image_path(self, background_image_path: Option<String>) -> Self {
        return Self {
            background_image_path,
            ..self
        };
    }

    /// Returns the window configuration with the width.
    pub fn width(self, width: f64) -> Self {
        return Self {
            width,
            ..self
        };
    }

    /// Returns the window configuration with the height.
    pub fn height(self, height: f64) -> Self {
        return Self {
            height,
            ..self
        };
    }

    /// Returns the window configuration with the position x.
    pub fn position_x(self, position_x: f64) -> Self {
        return Self {
            position_x,
            ..self
        };
    }

    /// Returns the window configuration with the position y.
    pub fn position_y(self, position_y: f64) -> Self {
        return Self {
            position_y,
            ..self
        };
    }

    /// Returns the window configuration with the resizable.
    pub fn resizable(self, resizable: bool) -> Self {
        return Self {
            resizable,
            ..self
        };
    }

    /// Returns the window configuration with the decorations.
    pub fn decorations(self, decorations: bool) -> Self {
        return Self {
            decorations,
            ..self
        };
    }

    /// Returns the window configuration with the transparent.
    pub fn transparent(self, transparent: bool) -> Self {
        return Self {
            transparent,
            ..self
        };
    }

    /// Returns the window configuration with the visible.
    pub(crate) fn _visible(self, _visible: bool) -> Self {
        return Self {
            //visible,
            ..self
        };
    }

    /// Returns the window configuration with the active.
    pub fn active(self, active: bool) -> Self {
        return Self {
            active,
            ..self
        };
    }

    /// Returns the window configuration with the enabled buttons.
    pub fn enabled_buttons(self, enabled_buttons: WindowButtons) -> Self {
        return Self {
            enabled_buttons,
            ..self
        };
    }

    /// Returns the window configuration with the present mode.
    pub fn present_mode(self, present_mode: PresentMode) -> Self {
        return Self {
            present_mode,
            ..self
        };
    }

    /// Returns a icon by its relative bytes.
    pub fn get_icon_by_bytes(icon_as_bytes: Vec<u8>) -> Option<Icon> {
        if let Ok(image) = image::load_from_memory(&icon_as_bytes) {
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
        let mut present_mode: PresentMode = PresentMode::AutoNoVsync;

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
            window_attributes.visible = false; // Trick to appear the icon on the taskbar.
            window_attributes.active = window_configuration.active;
            window_attributes.enabled_buttons = window_configuration.enabled_buttons;
            present_mode = window_configuration.present_mode;

            if window_configuration.icon_path.is_empty() {
                window_attributes.window_icon = WindowConfiguration::get_icon_by_bytes(
                    include_bytes!("../../../../assets/textures/lotus_pink_256x256_aligned.png").to_vec()
                );
            } else {
                window_attributes.window_icon = WindowConfiguration::get_icon_by_bytes(
                    AssetLoader::load_bytes(&window_configuration.icon_path).ok().unwrap()
                );
            }

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
        self.window.as_ref().unwrap().focus_window();

        let world: World = World::new();
        let mut render_state: RenderState = pollster::block_on(RenderState::new(window, present_mode));
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

        if !render_state.input(&window_event) {
            match window_event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                },
                WindowEvent::Resized(new_size) => {
                    let camera2d: ResourceRef<'_, Camera2d> = context.world.get_resource::<Camera2d>().unwrap();
                    render_state.resize(new_size, &camera2d, &context.world.text_renderers);
                },
                WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => {
                    let mut input: ResourceRefMut<'_, Input> = context.world.get_resource_mut::<Input>().unwrap();

                    if ElementState::Pressed == event.state {
                        input.pressed_keys.insert(event.physical_key);
                    } else if ElementState::Released == event.state {
                        input.pressed_keys.remove(&event.physical_key);
                    }
                },
                WindowEvent::CursorMoved { device_id: _, position } => {
                    let mut input: ResourceRefMut<'_, Input> = context.world.get_resource_mut::<Input>().unwrap();
                    input.mouse_position.x = position.x as f32;
                    input.mouse_position.y = position.y as f32;
                },
                WindowEvent::MouseInput { device_id: _, state, button } => {
                    let mut input: ResourceRefMut<'_, Input> = context.world.get_resource_mut::<Input>().unwrap();

                    if ElementState::Pressed == state {
                        input.pressed_mouse_buttons.insert(button);
                    } else if ElementState::Released == state {
                        input.pressed_mouse_buttons.remove(&button);
                    }
                },
                WindowEvent::RedrawRequested => {
                    render_state.window().request_redraw();
                    self.game_loop.render(&mut context.render_state, &mut context.world, event_loop);
                }
                _ => ()
            }
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.window.as_mut().unwrap().set_visible(true);
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
