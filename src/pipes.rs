use bevy::prelude::*;

use crate::{collision::Hitbox, states::AppState};

const PIPE_HEIGHT: f32 = 200.0;
const PIPE_OFFSET: f32 = 100.0;
const PIPE_GAP: f32 = 200.0;

#[derive(Resource)]
struct PipeSpawnTimer(Timer);

impl PipeSpawnTimer {
    pub fn new() -> Self {
        Self(Timer::from_seconds(3.0, TimerMode::Repeating))
    }
}

impl Default for PipeSpawnTimer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Component)]
pub struct Pipe;

#[derive(Component)]
pub struct ScoringZone;

pub struct PipePlugin;
impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                tick_spawn_timer_system,
                spawn_pipes_system,
                despawn_pipes_system,
                move_pipes_system,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(OnEnter(AppState::InGame), clear_pipes_system)
        .init_resource::<PipeSpawnTimer>();
    }
}

fn spawn_pipes_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    spawn_timer: Res<PipeSpawnTimer>,
) {
    if spawn_timer.0.just_finished() {
        let mut y_pos = rand::random::<f32>() * (PIPE_HEIGHT - PIPE_OFFSET);
        y_pos += PIPE_OFFSET;
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(200.0, y_pos, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    flip_y: true,
                    ..default()
                },
                texture: asset_server.load("sprites/pipe-green.png"),
                ..default()
            },
            Pipe {},
            Hitbox {
                scale: Vec2::new(52.0, 320.0),
            },
        ));
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(200.0, y_pos - (PIPE_HEIGHT + PIPE_GAP), 0.0),
                    ..default()
                },
                sprite: Sprite { ..default() },
                texture: asset_server.load("sprites/pipe-green.png"),
                ..default()
            },
            Pipe {},
            Hitbox {
                scale: Vec2::new(52.0, 320.0),
            },
        ));
    }
}

fn despawn_pipes_system(mut commands: Commands, pipes: Query<(Entity, &Transform), With<Pipe>>) {
    for (entity, transform) in &pipes {
        if transform.translation.x < -200.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn clear_pipes_system(mut commands: Commands, pipes: Query<Entity, With<Pipe>>) {
    for entity in &pipes {
        commands.entity(entity).despawn();
    }
}

fn move_pipes_system(mut pipes: Query<&mut Transform, With<Pipe>>, time: Res<Time>) {
    for mut transform in &mut pipes {
        // TODO: Nasty nasty magic number, change this!!!!!
        transform.translation.x -= 50.0 * time.delta_seconds();
    }
}

fn tick_spawn_timer_system(mut spawn_timer: ResMut<PipeSpawnTimer>, time: Res<Time>) {
    spawn_timer.0.tick(time.delta());
}
