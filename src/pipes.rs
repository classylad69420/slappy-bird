use bevy::prelude::*;

pub struct PipePlugin;

#[derive(Component)]
struct Pipe {}

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_pipes_system, move_pipes_system));
    }
}

fn spawn_pipes_system(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(200.0, 0.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(20.0, 100.0)),
                color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        Pipe {},
    ));
}

fn move_pipes_system(mut pipes: Query<(&mut Transform, With<Pipe>)>) {
    for (mut transform, _) in &mut pipes {
        transform.translation.x = transform.translation.x - 5.0;
    }
}
