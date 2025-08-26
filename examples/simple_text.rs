//! This example is a show off for the text rendering.
//! Inside the 'setup' function, each text component is spawned as an entity.
//! Each text component uses the default fonts available on the engine.
//! As the end-user you can bring your own fonts or make use of the available ones.

use lotus_engine::*;

your_game!(
    WindowConfiguration::default(),
    setup,
    update
);

fn setup(context: &mut Context) {
    let underdog_regular: Text = Text::new(
        &mut context.render_state,
        Font::new(Fonts::UnderdogRegular.get_path(), 200.0),
        Position::new(Vector2::new(0.0, 0.0), Strategy::Normalized),
        Color::BLACK,
        "A".to_string()
    );
    context.commands.spawn(vec![Box::new(underdog_regular)]);
}

fn update(_context: &mut Context) {}
