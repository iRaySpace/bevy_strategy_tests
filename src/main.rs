use bevy::{prelude::*, sprite::collide_aabb::collide};

#[derive(Component, Debug)]
struct Troop {
    selected: bool,
}

fn startup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(30.0, 30.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|children| {
            children.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(2.0, 2.0)),
                    ..Default::default()
                },
                ..Default::default()
            });
        })
        .insert(Troop { selected: false });
}

fn point_click_system(
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut collider_query: Query<(Entity, &mut Troop, &Transform)>,
) {
    let win = windows.get_primary().expect("no primary window");
    if mouse_input.just_pressed(MouseButton::Left) {
        let size = Vec2::new(win.width() as f32, win.height() as f32);
        let default_orthographic_pos = size / 2.0;

        // convert mouse cursor position to world position
        let mut mouse_pos = win.cursor_position().unwrap();
        mouse_pos = mouse_pos - default_orthographic_pos;

        for (collider_entity, mut troop, transform) in collider_query.iter_mut() {
            let collision = collide(
                Vec3::new(mouse_pos.x, mouse_pos.y, 0.0),
                Vec2::new(25.0, 25.0),
                transform.translation,
                transform.scale.truncate(),
            );
            troop.selected = if collision.is_some() { true } else { false };
        }
    }
}

fn move_system(
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut troop_query: Query<(&Troop, &mut Transform)>,
) {
    let win = windows.get_primary().expect("no primary window");
    if mouse_input.just_pressed(MouseButton::Right) {
        let size = Vec2::new(win.width() as f32, win.height() as f32);
        let default_orthographic_pos = size / 2.0;

        // convert mouse cursor position to world position
        let mut mouse_pos = win.cursor_position().unwrap();
        mouse_pos = mouse_pos - default_orthographic_pos;

        for (troop, mut transform) in troop_query.iter_mut() {
            if troop.selected {
                transform.translation.x = mouse_pos.x;
                transform.translation.y = mouse_pos.y;
            }
        }
    }
}

fn selected_tint_system(
    troop_query: Query<(Entity, &Troop)>,
) {
    for (entity, troop) in troop_query.iter() {
        info!("{:?}", entity);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.4, 0.9)))
        .add_startup_system(startup)
        .add_system(point_click_system)
        .add_system(move_system)
        .add_system(selected_tint_system)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
