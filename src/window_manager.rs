use wgpu::SurfaceError;
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
        WindowId
    }
};
use std::sync::Arc;

use crate::rendering_manager::State;

#[derive(Default)]
struct Application {
    window: Option<Arc<Window>>,
    state: Option<State>
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window: Arc<Window> = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());
        self.window = Some(window.clone());

        let state: State = pollster::block_on(State::new(window));
        self.state = Some(state);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, window_event: WindowEvent) {
        if let Some(state) = &mut self.state {
            match window_event {
                WindowEvent::CloseRequested => {
                    println!("Close button pressed.");
                    event_loop.exit();
                },
                WindowEvent::Resized(new_size) => {
                    state.resize(new_size);
                }
                WindowEvent::RedrawRequested => {
                    state.window().request_redraw();
                    state.update();

                    match state.render() {
                        Ok(_) => {}

                        Err(
                            SurfaceError::Lost | SurfaceError::Outdated
                        ) => state.resize(state.physical_size),

                        Err(
                            SurfaceError::OutOfMemory | SurfaceError::Other
                        ) => {
                            log::error!("Application OOMKilled.");
                            event_loop.exit();
                        }

                        Err(SurfaceError::Timeout) => {
                            log::warn!("Surface Timeout.")
                        }
                    }
                }
                _ => ()
            }
        }
    }
}

pub(crate) async fn open_default_window() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut application: Application = Application::default();
    let _ = event_loop.run_app(&mut application);
}
