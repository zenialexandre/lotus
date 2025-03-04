pub mod core;

use core::windowing_manager::{initialize_application, WindowConfiguration};

pub fn initialize() {
    pollster::block_on(initialize_application(None));
}

pub fn initialize_with_configuration(window_configuration: WindowConfiguration) {
    pollster::block_on(initialize_application(Some(window_configuration)));
}
