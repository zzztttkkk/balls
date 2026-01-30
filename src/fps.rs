use bevy::prelude::*;

#[derive(Component)]
struct FpsLabel;

#[derive(Resource, Default)]
struct FpsCounter {
    delta_sum: f32,
    frame_count: u32,
    last_fps: u32,
}

fn setup_fps(mut cmds: Commands) {
    cmds.spawn((
        FpsLabel,
        Text2d::new(""),
        Transform::from_xyz(0.0, 0.0, 100.0),
    ));
}

fn update_fps(
    mut query: Query<(&mut Text2d, &FpsLabel)>,
    time: Res<Time>,
    mut counter: ResMut<FpsCounter>,
) {
    let (mut text, _) = query.single_mut().unwrap();
    counter.delta_sum += time.delta_secs();
    counter.frame_count += 1;
    if counter.delta_sum >= 1.0 {
        counter.last_fps = counter.frame_count;
        counter.frame_count = 0;
        counter.delta_sum -= 1.0;
        text.0 = format!("FPS: {}", counter.last_fps);
    }
}

pub(crate) struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FpsCounter::default());
        app.add_systems(Startup, setup_fps)
            .add_systems(Update, update_fps);
    }
}
