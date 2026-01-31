use bevy::{prelude::*, window::PresentMode};
use rand::Rng;

mod fps;
mod utils;

#[derive(Component, Debug, Clone)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component, Debug, Clone)]
struct ReadonlyProps {
    radius: i32,
}

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

fn gen_balls(mut cmds: Commands, mut images: ResMut<Assets<Image>>, winq: Query<&Window>) {
    cmds.spawn(Camera2d);

    let circle = images.add(utils::create_circle_image(64));

    let (x, y) = query_half_win_size(&winq);
    let mut rng = rand::rng();
    let vx = x;
    let vy = y;

    for _ in 0..1000 {
        let pos = Vec2 {
            x: rng.random_range(-x..x),
            y: rng.random_range(-y..y),
        };
        let vel = Velocity {
            x: non_zero_rand(&mut rng, -vx, vx),
            y: non_zero_rand(&mut rng, -vy, vy),
        };
        let radius = rng.random_range(5..20);
        let props = ReadonlyProps { radius };
        let color = Color::srgba(
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
            rng.random_range(0.0..1.0),
            rng.random_range(0.5..1.0),
        );

        cmds.spawn((
            vel,
            props,
            Sprite {
                image: circle.clone(),
                color,
                custom_size: Some(Vec2::splat((radius * 2) as f32)),
                ..default()
            },
            Transform {
                translation: Vec3::new(pos.x, pos.y, 0.0),
                ..default()
            },
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

    query
        .par_iter_mut()
        .for_each(|(mut velocity, mut transform, props)| {
            let mut position = transform.translation.xy();
            position.x += velocity.x * delta;
            position.y += velocity.y * delta;

            let radius = props.radius as f32;
            let xdiff = hwx - radius;
            let ydiff = hwy - radius;

            if position.x < -xdiff {
                velocity.x = -velocity.x;
                position.x = radius - hwx + 1.0;
            } else if position.x > xdiff {
                velocity.x = -velocity.x;
                position.x = xdiff - 1.0;
            }

            if position.y < -ydiff {
                velocity.y = -velocity.y;
                position.y = radius - hwy;
            } else if position.y > ydiff {
                velocity.y = -velocity.y + 1.0;
                position.y = ydiff - 1.0;
            }

            transform.translation = Vec3::new(position.x, position.y, 0.0);
        });
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum AppState {
    #[default]
    Running,
    Paused,
}

fn handle_keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }

    if keys.just_pressed(KeyCode::F9) {
        match state.get() {
            AppState::Running => next_state.set(AppState::Paused),
            AppState::Paused => next_state.set(AppState::Running),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Immediate,
                mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .add_systems(Startup, gen_balls)
        .add_systems(Update, update_balls.run_if(in_state(AppState::Running)))
        .add_systems(Update, handle_keyboard_input)
        .add_plugins(fps::FpsPlugin)
        .run();
}
