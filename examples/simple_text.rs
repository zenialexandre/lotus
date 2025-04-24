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
        Font::new(Fonts::UnderdogRegular.get_path(), 80.0),
        Position::new(Vector2::new(0.0, 0.0), Strategy::Normalized),
        Color::BLACK,
        "Hello Text!".to_string()
    );
    context.commands.spawn(vec![Box::new(underdog_regular)]);

    let codystar_light: Text = Text::new(
        Font::new(Fonts::CodystarLight.get_path(), 80.0),
        Position::new(Vector2::new(0.0, 0.25), Strategy::Normalized),
        Color::BLUE,
        "Hello Text!".to_string()
    );
    context.commands.spawn(vec![Box::new(codystar_light)]);

    let codystar_regular: Text = Text::new(
        Font::new(Fonts::CodystarRegular.get_path(), 80.0),
        Position::new(Vector2::new(0.0, 0.45), Strategy::Normalized),
        Color::MAGENTA,
        "Hello Text!".to_string()
    );
    context.commands.spawn(vec![Box::new(codystar_regular)]);

    let roboto_mono: Text = Text::new(
        Font::new(Fonts::RobotoMono.get_path(), 80.0),
        Position::new(Vector2::new(0.0, 0.65), Strategy::Normalized),
        Color::BROWN,
        "Hello Text!".to_string()
    );
    context.commands.spawn(vec![Box::new(roboto_mono)]);

    let roboto_mono_italic: Text = Text::new(
        Font::new(Fonts::RobotoMonoItalic.get_path(), 80.0),
        Position::new(Vector2::new(0.0, 0.85), Strategy::Normalized),
        Color::RED,
        "Hello Text!".to_string()
    );
    context.commands.spawn(vec![Box::new(roboto_mono_italic)]);
}

fn update(_context: &mut Context) {}
