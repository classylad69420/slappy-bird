use bevy::{prelude::*, render::camera::ScalingMode};
use collision::CollisionPlugin;
use pipes::PipePlugin;
use player::PlayerPlugin;

mod collision;
mod pipes;
mod player;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Slappy bird".into(),
                        resolution: (1200.0, 900.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins((PlayerPlugin, PipePlugin, CollisionPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera);
}
