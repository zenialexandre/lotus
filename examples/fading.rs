#![doc =
    r"This example is a simple show off of the engine basic template.
    It will create a window with a white background and the 'Hello World!' message will be printed at each frame."
]

use lotus_engine::*;

your_game!(
    WindowConfiguration::default(),
    setup,
    update
);

fn setup(context: &mut Context) {
    let sprite: Sprite = Sprite::new("textures/lotus_pink_256x256.png".to_string());

    context.commands.spawn(
        vec![
            Box::new(sprite),
            Box::new(Fade::default()),
            Box::new(Transform::new(
                Position::new(Vec2::new(0.0, 0.0), Strategy::Normalized),
                0.0,
                Vec2::new(0.25, 0.25)
            )),
        ]
    );
}

fn update(context: &mut Context) {
    let keyboard_input: ResourceRef<'_, KeyboardInput> = context.world.get_resource::<KeyboardInput>().unwrap();

    let mut query: Query = Query::new(&context.world).with::<Fade>();
    let entity: Entity = query.entities_with_components().unwrap().first().unwrap().clone();
    let mut fade_component: ComponentRefMut<'_, Fade> = context.world.get_entity_component_mut::<Fade>(&entity).unwrap();

    if keyboard_input.is_key_pressed(KeyboardKey::Enter) || crate::is_any_fading(vec![fade_component.as_any().downcast_ref::<Fade>().unwrap()]) {
        let color: ComponentRefMut<'_, Color> = context.world.get_entity_component_mut::<Color>(&entity).unwrap();

        fade_component.state = true;

        if fade_component.fade_out(vec![color], context.delta, 0.5) {
            eprintln!("to");
            fade_component.state = false;
        }
    }

    if keyboard_input.is_key_pressed(KeyboardKey::Space) {
        let fade: ComponentRefMut<'_, Fade> = context.world.get_entity_component_mut::<Fade>(&entity).unwrap();
        let color: ComponentRefMut<'_, Color> = context.world.get_entity_component_mut::<Color>(&entity).unwrap();

        fade.fade_in(vec![color], context.delta, 0.5);
    }
}
