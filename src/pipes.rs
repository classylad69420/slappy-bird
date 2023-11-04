use bevy::prelude::*;

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

pub struct PipePlugin;

#[derive(Component)]
struct Pipe {}

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                tick_spawn_timer_system,
                spawn_pipes_system,
                despawn_pipes_system,
                move_pipes_system,
            ),
        )
        .init_resource::<PipeSpawnTimer>();
    }
}

fn spawn_pipes_system(mut commands: Commands, spawn_timer: Res<PipeSpawnTimer>) {
    if spawn_timer.0.just_finished() {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(200.0, 0.0, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::new(20.0, 100.0)),
                    color: Color::rgba(0.25, 1.0, 0.25, 1.0),
                    ..default()
                },
                ..default()
            },
            Pipe {},
        ));
    }
}

fn despawn_pipes_system(mut commands: Commands, mut pipes: Query<(&mut Transform, &Pipe)>) {}

fn move_pipes_system(mut pipes: Query<(&mut Transform, &Pipe)>, time: Res<Time>) {
    for (mut transform, _) in &mut pipes {
        transform.translation.x = transform.translation.x - (50.0 * time.delta_seconds());
    }
}

fn tick_spawn_timer_system(mut spawn_timer: ResMut<PipeSpawnTimer>, time: Res<Time>) {
    spawn_timer.0.tick(time.delta());
}
