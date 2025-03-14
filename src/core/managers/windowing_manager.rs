use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{
        ActiveEventLoop,
        ControlFlow,
        EventLoop
    },
    window::{
        Window,
        WindowAttributes,
        WindowId,
        WindowButtons,
        Icon
    },
    dpi::{
        Size,
        Position,
        LogicalSize,
        LogicalPosition
    }
};
use std::{
    path::Path,
    sync::Arc
};

use crate::core::engine::EngineContext;

use super::super::{
    ecs::world::World,
    color::Color,
    game_loop::GameLoop
};
use super::rendering_manager::RenderState;

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
    pub enabled_buttons: WindowButtons
}

impl Default for WindowConfiguration {
    fn default() -> Self {
        return Self {
            icon_path: "assets/textures/lotus_pink_256x256.png".to_string(),
            title: "New Game!".to_string(),
            background_color: Some(Color::WHITE),
            background_image_path: None,
            width: 800.,
            height: 600.,
            position_x: 100.,
            position_y: 100.,
            resizable: true,
            decorations: true,
            transparent: true,
            visible: true,
            enabled_buttons: WindowButtons::all()
        };
    }
}

impl WindowConfiguration {
    pub fn get_icon_by_path(icon_path: String) -> Option<Icon> {
        if let Ok(image) = image::open(Path::new(icon_path.as_str())) {
            let icon_image = image.into_rgba8();
            let (icon_width, icon_height) = icon_image.dimensions();
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
    engine_context: Option<EngineContext>,
    game_loop: GameLoop
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut color: Option<Color> = None;
        let mut background_image_path: String = String::new();
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
            window_attributes.enabled_buttons = window_configuration.enabled_buttons;
            window_attributes.window_icon = WindowConfiguration::get_icon_by_path(window_configuration.icon_path.clone());
            
            if let Some(background_color) = window_configuration.background_color {
                color = Some(background_color);
            }

            if let Some(background_image_path_unwrapped) = &window_configuration.background_image_path {
                background_image_path = background_image_path_unwrapped.to_string();
            }
            Arc::new(event_loop.create_window(window_attributes).unwrap())
        } else {
            Arc::new(event_loop.create_window(Window::default_attributes()).unwrap())
        };
        self.window = Some(window.clone());

        let mut render_state: RenderState = pollster::block_on(RenderState::new(window));
        render_state.color = color;
        render_state.background_image_path = Some(background_image_path);

        let world: World = World::new();
        self.engine_context = Some(EngineContext::new(render_state, world, 0.0));

        (self.game_loop.setup)(self.engine_context.as_mut().unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, window_event: WindowEvent) {
        let render_state: &mut RenderState = &mut self.engine_context.as_mut().unwrap().render_state;

        if !render_state.input(&window_event) {
            match window_event {
                WindowEvent::CloseRequested => {
                    println!("Close button pressed.");
                    event_loop.exit();
                },
                WindowEvent::Resized(new_size) => {
                    render_state.resize(new_size);
                }
                WindowEvent::RedrawRequested => {
                    render_state.window().request_redraw();
                }
                _ => ()
            }
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.game_loop.run(self.engine_context.as_mut().unwrap(), event_loop);
    }
}

pub async fn initialize_application(
    window_configuration: Option<WindowConfiguration>,
    setup: fn(engine_context: &mut EngineContext),
    update: fn(engine_context: &mut EngineContext)
) {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut application: Application = if let Some(window_configuration_unwrapped) = window_configuration {
        Application {
            window: None,
            engine_context: None,
            window_configuration: Some(window_configuration_unwrapped),
            game_loop: GameLoop::new(setup, update)
        }
    } else {
        Application {
            window: None,
            engine_context: None,
            window_configuration: None,
            game_loop: GameLoop::new(setup, update)
        }
    };
    let _ = event_loop.run_app(&mut application);
}
