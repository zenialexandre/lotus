use lotus_engine::*;

your_game!(
    WindowConfiguration::default(),
    setup,
    update
);

fn setup(context: &mut Context) {
    let underdog_regular: Text = Text::new(
        Font::new(Fonts::UnderdogRegular.get_bytes(), 80.0),
        Vector2::new(0.10, 0.10),
        Color::BLACK,
        "Hello Text!".to_string()
    );
    context.commands.spawn(vec![Box::new(underdog_regular)]);

    let codystar_light: Text = Text::new(
        Font::new(Fonts::CodystarLight.get_bytes(), 80.0),
        Vector2::new(0.10, 0.25),
        Color::BLUE,
        "Hello Text!".to_string()
    );
    context.commands.spawn(vec![Box::new(codystar_light)]);

    let codystar_regular: Text = Text::new(
        Font::new(Fonts::CodystarRegular.get_bytes(), 80.0),
        Vector2::new(0.10, 0.45),
        Color::MAGENTA,
        "Hello Text!".to_string()
    );
    context.commands.spawn(vec![Box::new(codystar_regular)]);

    let roboto_mono: Text = Text::new(
        Font::new(Fonts::RobotoMono.get_bytes(), 80.0),
        Vector2::new(0.10, 0.65),
        Color::BROWN,
        "Hello Text!".to_string()
    );
    context.commands.spawn(vec![Box::new(roboto_mono)]);

    let roboto_mono_italic: Text = Text::new(
        Font::new(Fonts::RobotoMonoItalic.get_bytes(), 80.0),
        Vector2::new(0.10, 0.85),
        Color::RED,
        "Hello Text!".to_string()
    );
    context.commands.spawn(vec![Box::new(roboto_mono_italic)]);
}

fn update(_context: &mut Context) {}
