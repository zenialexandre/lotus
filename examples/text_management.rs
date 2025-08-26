//! This example is a show off for the text management.
//! Inside the 'setup' function, each text component is spawned as an entity.
//! Inside the 'update' function, each text component is mutated in a certain way.

use lotus_engine::*;
use rand::{rngs::ThreadRng, Rng};
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
        Color::BLACK,
        "Hello Text!".to_string()
    );
    context.commands.spawn(vec![Box::new(underdog_regular)]);

    let codystar_light: Text = Text::new(
        &mut context.render_state,
        Font::new(Fonts::CodystarLight.get_path(), TEXT_SIZE),
        Position::new(Vector2::new(TEXT_POSITION_X, 0.35), Strategy::Normalized),
        Color::BLUE,
        "Hello Text!".to_string()
    );
    context.commands.spawn(vec![Box::new(codystar_light)]);

    let codystar_regular: Text = Text::new(
        &mut context.render_state,
        Font::new(Fonts::CodystarRegular.get_path(), TEXT_SIZE),
        Position::new(Vector2::new(TEXT_POSITION_X, 0.65), Strategy::Normalized),
        Color::MAGENTA,
        "Hello Text!".to_string()
    );
    context.commands.spawn(vec![Box::new(codystar_regular)]);

    let roboto_mono: Text = Text::new(
        &mut context.render_state,
        Font::new(Fonts::RobotoMono.get_path(), TEXT_SIZE),
        Position::new(Vector2::new(TEXT_POSITION_X, 0.95), Strategy::Normalized),
        Color::BROWN,
        "Hello Text!".to_string()
    );
    context.commands.spawn(vec![Box::new(roboto_mono)]);
}

fn update(context: &mut Context) {
    let thread_rng: ThreadRng = rand::rng();
    let input: Input = {
        let input_ref: ResourceRef<'_, Input> = context.world.get_resource::<Input>().unwrap();
        input_ref.clone()
    };
    let mut query: Query = Query::new(&context.world).with::<Text>();
    let entities: Vec<Entity> = query.entities_with_components().unwrap();

    if input.is_key_pressed(KeyCode::Space) {
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
        0 => Color::BLACK,
        1 => Color::WHITE,
        2 => Color::RED,
        3 => Color::GREEN,
        4 => Color::BLUE,
        5 => Color::YELLOW,
        6 => Color::CYAN,
        7 => Color::MAGENTA,
        8 => Color::ORANGE,
        9 => Color::PURPLE,
        10 => Color::PINK,
        11 => Color::BROWN,
        12 => Color::LIGHTGRAY,
        13 => Color::GRAY,
        14 => Color::DARKGRAY,
        15 => Color::GOLD,
        16 => Color::SILVER,
        17 => Color::TURQUOISE,
        18 => Color::VIOLET,
        19 => Color::LIMEGREEN,
        20 => Color::LAVENDER,
        21 => Color::SALMON,
        22 => Color::PEACH,
        23 => Color::MOSSGREEN,
        24 => Color::NAVYBLUE,
        _ => Color::BURGUNDY,
    };
}
