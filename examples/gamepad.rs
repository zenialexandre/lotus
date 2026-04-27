//! This example aims to show off the usage of Gamepads.
//! First the listener needs to know if the user wants to hear Gamepad related events.
//! Then the user should set the deadzones of movement for the joysticks.
//! Inside of the GamepadInput struct, there are N GamepadInstances that are identified by a unique identifier.
//! Each GamepadInstance has its own events and can be queried individually.

use lotus_engine::*;

your_game!(
    WindowConfiguration::default(),
    setup,
    update
);

fn setup(context: &mut Context) {
    context.gamepad_listener.enable();

    let mut gamepad_input: ResourceRefMut<'_, GamepadInput> = context.world.get_resource_mut::<GamepadInput>().unwrap();
    gamepad_input.set_left_joystick_deadzone(0.15);
    gamepad_input.set_right_joystick_deadzone(0.15);
}

fn update(context: &mut Context) {
    let gamepad_input: GamepadInput = context.world.get_resource_cloned::<GamepadInput>().unwrap();

    for (id, gamepad) in gamepad_input.get_all_connected() {
        if gamepad.is_some_pressed() {
            println!("Gamepad {} - Some button has been pressed", id);
        }

        if gamepad.is_button_pressed(Button::Start) {
            println!("Gamepad {} - Start is pressed", id);
        }

        if gamepad.is_buttons_pressed(vec![Button::North, Button::East]) {
            println!("Gamepad {} - Buttons pressed at the same time", id);
        }

        if gamepad.is_left_joystick_moving(gamepad_input.left_joystick_deadzone) {
            println!("Gamepad {} - Left joystick is moving", id);
            println!("Gamepad {} - Left joystick x: {}", id, gamepad.left_joystick.x.current_direction);
            println!("Gamepad {} - Left joystick y: {}", id, gamepad.left_joystick.y.current_direction);
        }

        if gamepad.is_right_joystick_moving(gamepad_input.right_joystick_deadzone) {
            println!("Gamepad {} - Right joystick is moving", id);
            println!("Gamepad {} - Right joystick x: {}", id, gamepad.right_joystick.x.current_direction);
            println!("Gamepad {} - Right joystick y: {}", id, gamepad.right_joystick.y.current_direction);
        }
    }
}
