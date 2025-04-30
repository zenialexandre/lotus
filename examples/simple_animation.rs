//! This example is a show off about the animation system of the engine.
//! Our entity have a static sprite component and a animation component.
//! The animation component doesn't have any dependency on other components.
//! The animation component store N sprite sheets, with a title as the key.
//! In this example, a looped animation can be controlled by the keyboard keys.

use lotus_engine::*;

your_game!(
    WindowConfiguration::default().title("Simple Animation".to_string()),
    setup,
    update
);

fn setup(context: &mut Context) {
    let dummy: Sprite = Sprite::new("textures/lotus_pink_128x128.png".to_string());
    let scarfy_sprite_sheet: SpriteSheet = SpriteSheet::new(
        "textures/animations/scarfy.png".to_string(),
        Transform::default(),
        LoopingState::Once,
        (124.0, 124.0),
        2.0,
        1,
        6,
        vec![0, 1, 2, 3, 4, 5]
    );

    let mut animation: Animation = Animation::default();
    animation.add_sprite_sheet("run".to_string(), scarfy_sprite_sheet);
    context.commands.spawn(vec![Box::new(dummy), Box::new(animation)]);
}

fn update(context: &mut Context) {
    let input: ResourceRefMut<'_, Input> = context.world.get_resource_mut::<Input>().unwrap();
    let mut query: Query = Query::new(&context.world).with::<Animation>();
    let result: Entity = query.entities_with_components().unwrap().first().unwrap().clone();
    let mut animation: ComponentRefMut<'_, Animation> = context.world.get_entity_component_mut::<Animation>(&result).unwrap();

    if input.is_key_pressed(KeyCode::KeyW) {
        animation.play("run".to_string());
    } else if input.is_key_pressed(KeyCode::KeyA) {
        animation.pause("run".to_string());
    } else if input.is_key_pressed(KeyCode::KeyS) {
        animation.resume("run".to_string());
    } else if input.is_key_pressed(KeyCode::KeyD) {
        animation.stop("run".to_string());
    }
}
