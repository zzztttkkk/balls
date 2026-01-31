use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

#[derive(Component)]
struct FpsText;

#[derive(Resource, Default)]
struct Control {
    hide: bool,
}

fn handle_keyboard_input(keys: Res<ButtonInput<KeyCode>>, mut control: ResMut<Control>) {
    if keys.just_pressed(KeyCode::F5) {
        control.hide = !control.hide;
    }
}

pub(crate) struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .insert_resource(Control::default())
            .add_systems(Startup, setup_fps)
            .add_systems(Update, update_fps)
            .add_systems(Update, handle_keyboard_input);
    }
}

fn setup_fps(mut cmds: Commands) {
    cmds.spawn((
        Text::new(""),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node { ..default() },
        FpsText,
    ));
}

fn update_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
    control: Res<Control>,
) {
    if control.hide {
        if let Ok(mut text) = query.single_mut() {
            **text = "".to_string();
        }
        return;
    }

    if let Some(fps) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|diag| diag.smoothed())
    {
        if let Ok(mut text) = query.single_mut() {
            **text = format!("FPS: {:.0}", fps);
        }
    }
}
