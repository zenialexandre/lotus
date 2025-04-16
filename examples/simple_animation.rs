//! Work in progress.

use lotus_engine::*;

your_game!(
    WindowConfiguration::default().title("Simple Animation".to_string()),
    setup,
    update
);

fn setup(context: &mut Context) {
    let scarfy_sprite_sheet: SpriteSheet = SpriteSheet::new(
        "textures/animations/scarfy.png".to_string(),
        Transform::default(),
        0.2,
        (24, 24),
        1,
        6,
        vec![0, 1, 2, 3, 4, 5]
    );

    context.commands.spawn(vec![Box::new(scarfy_sprite_sheet)]);
}

fn update(_context: &mut Context) {
    //..
}
