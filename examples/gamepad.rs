use lotus_engine::*;

your_game!(
    WindowConfiguration::default(),
    setup,
    update
);

fn setup(context: &mut Context) {
    context.gamepad_listener.enable();
}

fn update(context: &mut Context) {
    let gamepad_input: GamepadInput = context.world.get_resource_cloned::<GamepadInput>().unwrap();

    for (id, gamepad) in gamepad_input.get_all_connected() {
        if gamepad.is_some_pressed() {
            println!("Gamepad {} - Some button has been pressed", id);
        }

        if gamepad.is_button_pressed(Start) {
            println!("Gamepad {} - Start is pressed", id);
        }

        if gamepad.is_buttons_pressed(vec![North, East]) {
            println!("Gamepad {} - Buttons pressed at the same time", id);
        }

        println!("Axis {}, Direction {}", gamepad.gamepad_axis.axis.is_stick(), gamepad.gamepad_axis.direction);
    }
}
