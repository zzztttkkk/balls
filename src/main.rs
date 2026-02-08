use avian2d::prelude::*;
use bevy::prelude::*;
use rand::Rng;

mod fps;
mod utils;

fn non_zero_rand<R: Rng>(rng: &mut R, min: f32, max: f32) -> f32 {
    let val = rng.random_range(min..max);
    if val.abs() < 5.0 {
        if val < 0.0 {
            return rng.random_range(min..-5.0);
        }
        return rng.random_range(5.0..max);
    }
    val
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Wall;

type WallBundle = (Wall, Transform, RigidBody, Collider, Restitution, Friction);

fn mk_wall(pos: Vec2, size: Vec2) -> WallBundle {
    (
        Wall,
        Transform::from_xyz(pos.x, pos.y, 0.0),
        RigidBody::Static,
        Collider::rectangle(size.x, size.y),
        Restitution::PERFECTLY_ELASTIC,
        Friction::ZERO,
    )
}

fn gen_balls(mut cmds: Commands, mut images: ResMut<Assets<Image>>, winq: Query<&Window>) {
    cmds.spawn(Camera2d);

    let circle = images.add(utils::create_circle_image(64));

    let (x, y) = query_half_win_size(&winq);
    let mut rng = rand::rng();
    let vx = x * 1.2;
    let vy = y * 1.2;

    cmds.spawn(mk_wall(Vec2 { x: 0.0, y }, Vec2 { x: x * 2.0, y: 1.0 }));
    cmds.spawn(mk_wall(Vec2 { x: 0.0, y: -y }, Vec2 { x: x * 2.0, y: 1.0 }));
    cmds.spawn(mk_wall(Vec2 { x: -x, y: 0.0 }, Vec2 { x: 1.0, y: y * 2.0 }));
    cmds.spawn(mk_wall(Vec2 { x: x, y: 0.0 }, Vec2 { x: 1.0, y: y * 2.0 }));

    for _ in 0..200 {
        let pos = Vec2 {
            x: rng.random_range(-x..x),
            y: rng.random_range(-y..y),
        };
        let radius = rng.random_range(2..30);
        let color = Color::srgba(
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
            rng.random_range(0.5..1.0),
        );

        cmds.spawn((
            Ball,
            Sprite {
                image: circle.clone(),
                color,
                custom_size: Some(Vec2::splat((radius * 2) as f32)),
                ..default()
            },
            Transform::from_xyz(pos.x, pos.y, 0.0),
            Circle::new(radius as f32).collider(),
            RigidBody::Dynamic,
            Friction::ZERO,
            Restitution::PERFECTLY_ELASTIC,
            LockedAxes::ROTATION_LOCKED,
            LinearVelocity(Vec2::new(
                non_zero_rand(&mut rng, -vx, vx),
                non_zero_rand(&mut rng, -vy, vy),
            )),
            Mass(radius as f32),
        ));
    }
}

fn query_half_win_size(winq: &Query<&Window>) -> (f32, f32) {
    let win = winq.single().unwrap();
    (win.width() / 2.0, win.height() / 2.0)
}

fn handle_keyboard_input(keys: Res<ButtonInput<KeyCode>>, mut phytime: ResMut<Time<Physics>>) {
    if keys.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }

    if keys.just_pressed(KeyCode::F9) {
        if phytime.is_paused() {
            phytime.unpause();
        } else {
            phytime.pause();
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec2::new(0.0, 0.0)))
        .add_systems(PostStartup, gen_balls)
        .add_systems(Update, handle_keyboard_input)
        .add_plugins(fps::FpsPlugin)
        .run();
}
