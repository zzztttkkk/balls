use bevy::prelude::*;
use rand::Rng;
use std::cell::RefCell;
use std::collections::HashMap;

mod fps;

#[derive(Component, Debug, Clone)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component, Debug, Clone)]
struct ReadonlyProps {
    radius: i32,
    color: Color,
}

fn non_zero_rand<R: Rng>(rng: &mut R, min: f32, max: f32) -> f32 {
    let range = max - min;
    let mut val = rng.random_range(min..max);
    if val.abs() < 1.0 {
        val = if val > 0.0 { 1.0 } else { -1.0 } + rng.random_range(0.0..(range - 2.0));
        val = val.clamp(min, max);
    }
    val
}

fn gen_balls(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    winq: Query<&Window>,
) {
    cmds.spawn(Camera2d);

    let shapes = RefCell::new(HashMap::<i32, Handle<Mesh>>::default());

    let mut mkcircle = |radius: i32| -> Handle<Mesh> {
        {
            let binding = shapes.borrow();
            let prev = binding.get(&radius);
            if let Some(prev) = prev {
                return prev.clone();
            }
        }
        let handle = meshes.add(Circle::new(radius as f32));
        shapes.borrow_mut().insert(radius, handle.clone());
        return handle;
    };

    let (x, y) = query_half_win_size(&winq);
    let mut rng = rand::rng();
    let vx = x * 2.0 / 4.0;
    let vy = y * 2.0 / 4.0;

    for _ in 0..50000 {
        let pos = Vec2 {
            x: rng.random_range(-x..x),
            y: rng.random_range(-y..y),
        };
        let vel = Velocity {
            x: non_zero_rand(&mut rng, -vx, vx),
            y: non_zero_rand(&mut rng, -vy, vy),
        };
        let props = ReadonlyProps {
            radius: rng.random_range(5..20),
            color: Color::srgba(
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
                rng.random_range(0.5..1.0),
            ),
        };

        let shape = Mesh2d(mkcircle(props.radius));

        let material = MeshMaterial2d(materials.add(props.color));

        cmds.spawn((
            vel,
            props,
            shape,
            material,
            Transform::from_xyz(pos.x, pos.y, 0.0),
        ));
    }
}

fn query_half_win_size(winq: &Query<&Window>) -> (f32, f32) {
    let win = winq.single().unwrap();
    (win.width() / 2.0, win.height() / 2.0)
}

fn update_balls(
    winq: Query<&Window>,
    mut query: Query<(&mut Velocity, &mut Transform, &ReadonlyProps)>,
    time: Res<Time>,
) {
    let (hwx, hwy) = query_half_win_size(&winq);
    let delta = time.delta_secs();

    for (mut velocity, mut transform, props) in query.iter_mut() {
        let mut position = transform.translation.xy();
        position.x += velocity.x * delta;
        position.y += velocity.y * delta;

        let radius = props.radius as f32;

        if position.x + radius > hwx || position.x - radius < -hwx {
            velocity.x = -velocity.x;
        }

        if position.y + radius > hwy || position.y - radius < -hwy {
            velocity.y = -velocity.y;
        }
        transform.translation = Vec3::new(position.x, position.y, 0.0);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, gen_balls)
        .add_systems(Update, update_balls)
        .add_plugins(fps::FpsPlugin)
        .run();
}
