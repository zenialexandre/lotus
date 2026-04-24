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

    if let Some((key, value)) = gamepad_input.instances.iter().next() {
        //println!("id = {}", key);

        if value.is_some_pressed() {
            println!("Some button has been pressed");
        }

        if value.is_button_pressed(Start) {
            println!("Start is pressed");
        }

        if value.is_buttons_pressed(vec![North, East]) {
            println!("Buttons pressed at the same time");
        }
    }
}
