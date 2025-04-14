use lotus_engine::*;
use std::time::Duration;

your_game!(
    WindowConfiguration::default().title("Simple Animation".to_string()),
    setup,
    update
);

fn setup(context: &mut Context) {
    let potion: Sprite = Sprite::new("textures/animations/potion_idle_256x256.png".to_string());
    let animation: SpriteSheet = SpriteSheet::new(
        "textures/animations/potion_sprite_sheet".to_string(),
        Transform::default(),
        Timer::new(TimerType::Repeat, Duration::new(1, 0)),
        7,
        1,
        vec![0, 1, 2, 3, 4, 5, 6]
    );

    context.commands.spawn(vec![Box::new(potion)]);
    context.commands.spawn(vec![Box::new(animation)]);
}

fn update(context: &mut Context) {
    let mut query: Query = Query::new(&context.world).with::<Sprite>();
    let cyndaquil_entity: &Entity = query.entities_with_components().unwrap().first().unwrap();


}
