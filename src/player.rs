use bevy::prelude::*;

use crate::{collision::Hitbox, states::AppState};

const JUMP_SPEED: f32 = 160.0;

#[derive(Component)]
pub struct Player {
    fall_speed: f32,
}

const GRAVITY: f32 = -490.0;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_player)
            .add_systems(
                Update,
                (
                    player_movement_system.run_if(in_state(AppState::InGame)),
                    restart_game_system.run_if(in_state(AppState::GameOver)),
                ),
            );
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_query: Query<Entity, With<Player>>,
) {
    let player_entity_result = player_query.get_single_mut();
    if let Ok(player_entity) = player_entity_result {
        commands.entity(player_entity).despawn();
    }
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-75.0, 0.0, 0.0),
                ..default()
            },
            sprite: Sprite { ..default() },
            texture: asset_server.load("sprites/yellowbird-downflap.png"),
            ..default()
        },
        Player { fall_speed: 0.0 },
        Name::new("Player"),
        Hitbox {
            scale: Vec2::new(34.0, 24.0),
        },
    ));
}

fn player_movement_system(
    mut player_query: Query<(&mut Transform, &mut Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let timestep = time.delta_seconds();
    for (mut transform, mut player) in &mut player_query {
        if input.just_pressed(KeyCode::Space) {
            // TODO: bad evil constant
            player.fall_speed = JUMP_SPEED - (player.fall_speed * 0.15);
        }

        if input.pressed(KeyCode::Tab) {
            transform.translation.y = 0.0;
            player.fall_speed = 0.0;
        }

        // Framerate-independent constant acceleration calculation
        // https://stackoverflow.com/questions/43960217/framerate-independent-acceleration-decceleration (accessed 2/6/2024)
        transform.translation.y +=
            player.fall_speed * timestep + 0.5 * GRAVITY * timestep * timestep;
        player.fall_speed += GRAVITY * timestep;
    }
}

fn restart_game_system(input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if input.just_pressed(KeyCode::Tab) {
        next_state.set(AppState::InGame);
    }
}
