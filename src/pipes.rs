use bevy::prelude::*;

pub struct PipePlugin;

#[derive(Component)]
struct Pipe {}

#[derive(Default, Resource)]
struct PipeArray(Vec<&Pipe>);

impl PipeArray {
    pub fn push_pipe(&mut self, pipe: &mut Pipe) {
        self.0.push(pipe);
    }
}

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PipeArray>()
            .add_systems(Update, spawn_pipes_system);
    }
}

fn spawn_pipes_system(mut commands: Commands, pipes: ResMut<PipeArray>) {
    let pipe = Pipe {};
    pipes.push_pipe(&pipe);
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(20.0, 100.0)),
                color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        pipe,
        Name::new("Pipe"),
    ));
}
