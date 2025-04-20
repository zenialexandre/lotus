//! This example is a simple show off of the engine basic template.
//! It will create a window with a white background and the 'Hello World!' message will be printed at each frame.

use lotus_engine::*;

your_game!(
    WindowConfiguration::default(),
    setup,
    update
);

fn setup(_context: &mut Context) {}

fn update(_context: &mut Context) {
    eprintln!("Hello World!");
}
