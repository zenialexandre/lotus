//! This example is a show off for the text management.
//! Inside the 'setup' function, each text component is spawned as an entity.
//! Inside the 'update' function, each text component is mutated in a certain way.

use lotus_engine::*;
use rand::{RngExt, rngs::ThreadRng};
use uuid::Uuid;

const TEXT_SIZE: f32 = 65.0;
const TEXT_POSITION_X: f32 = -0.40;

your_game!(
    WindowConfiguration::default(),
    setup,
    update
);

fn setup(context: &mut Context) {
    let underdog_regular: Text = Text::new(
        &mut context.render_state,
        Font::new(Fonts::UnderdogRegular.get_path(), TEXT_SIZE),
        Position::new(Vector2::new(TEXT_POSITION_X, 0.0), Strategy::Normalized),
        Color::by_option(ColorOption::Black),
        "Hello Text!".to_string()
    );
    context.commands.spawn(vec![Box::new(underdog_regular)]);

    let codystar_light: Text = Text::new(
        &mut context.render_state,
        Font::new(Fonts::CodystarLight.get_path(), TEXT_SIZE),
        Position::new(Vector2::new(TEXT_POSITION_X, 0.35), Strategy::Normalized),
        Color::by_option(ColorOption::Blue),
        "Hello Text!".to_string()
    );
    context.commands.spawn(vec![Box::new(codystar_light)]);

    let codystar_regular: Text = Text::new(
        &mut context.render_state,
        Font::new(Fonts::CodystarRegular.get_path(), TEXT_SIZE),
        Position::new(Vector2::new(TEXT_POSITION_X, 0.65), Strategy::Normalized),
        Color::by_option(ColorOption::Magenta),
        "Hello Text!".to_string()
    );
    context.commands.spawn(vec![Box::new(codystar_regular)]);

    let roboto_mono: Text = Text::new(
        &mut context.render_state,
        Font::new(Fonts::RobotoMono.get_path(), TEXT_SIZE),
        Position::new(Vector2::new(TEXT_POSITION_X, 0.95), Strategy::Normalized),
        Color::by_option(ColorOption::Brown),
        "Hello Text!".to_string()
    );
    context.commands.spawn(vec![Box::new(roboto_mono)]);
}

fn update(context: &mut Context) {
    let thread_rng: ThreadRng = rand::rng();
    let keyboard_input: KeyboardInput = context.world.get_resource_cloned::<KeyboardInput>().unwrap();
    let mut query: Query = Query::new(&context.world).with::<Text>();
    let entities: Vec<Entity> = query.entities_with_components().unwrap();

    if keyboard_input.is_key_pressed(KeyCode::Space) {
        for entity in entities {
            let text: ComponentRef<'_, Text> = context.world.get_entity_component::<Text>(&entity).unwrap();

            text.font(&context.world, entity, randomize_font(thread_rng.clone()));
            text.content(&context.world, entity, Uuid::new_v4().to_string());
            text.color(&context.world, entity, randomize_color(thread_rng.clone()));
        }
    }
}

fn randomize_font(mut thread_rng: ThreadRng) -> Font {
    let font_enum: Fonts = match thread_rng.random_range(0..4) {
        0 => Fonts::CodystarRegular,
        1 => Fonts::CodystarLight,
        2 => Fonts::RobotoMono,
        3 => Fonts::RobotoMonoItalic,
        _ => Fonts::UnderdogRegular,
    };
    return Font::new(font_enum.get_path(), TEXT_SIZE);
}

fn randomize_color(mut thread_rng: ThreadRng) -> Color {
    return match thread_rng.random_range(0..26) {
        0 => Color::by_option(ColorOption::Black),
        1 => Color::by_option(ColorOption::White),
        2 => Color::by_option(ColorOption::Red),
        3 => Color::by_option(ColorOption::Green),
        4 => Color::by_option(ColorOption::Blue),
        5 => Color::by_option(ColorOption::Yellow),
        6 => Color::by_option(ColorOption::Cyan),
        7 => Color::by_option(ColorOption::Magenta),
        8 => Color::by_option(ColorOption::Orange),
        9 => Color::by_option(ColorOption::Purple),
        10 => Color::by_option(ColorOption::Pink),
        11 => Color::by_option(ColorOption::Brown),
        12 => Color::by_option(ColorOption::Lightgray),
        13 => Color::by_option(ColorOption::Gray),
        14 => Color::by_option(ColorOption::Darkgray),
        15 => Color::by_option(ColorOption::Gold),
        16 => Color::by_option(ColorOption::Silver),
        17 => Color::by_option(ColorOption::Turquoise),
        18 => Color::by_option(ColorOption::Violet),
        19 => Color::by_option(ColorOption::Limegreen),
        20 => Color::by_option(ColorOption::Lavender),
        21 => Color::by_option(ColorOption::Salmon),
        22 => Color::by_option(ColorOption::Peach),
        23 => Color::by_option(ColorOption::Mossgreen),
        24 => Color::by_option(ColorOption::Navyblue),
        _ => Color::by_option(ColorOption::Burgundy),
    };
}
