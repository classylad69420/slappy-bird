use bevy::prelude::*;

use crate::states::AppState;

const JUMP_SPEED: f32 = 175.0;

#[derive(Component)]
pub struct Player {
    fall_speed: f32,
}

const GRAVITY: f32 = 9.8;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_player)
            .add_systems(
                Update,
                player_movement_system.run_if(in_state(AppState::InGame)),
            );
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-75.0, 0.0, 0.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            sprite: Sprite { ..default() },
            texture: asset_server.load("sprites/yellowbird-downflap.png"),
            ..default()
        },
        Player { fall_speed: 0.0 },
        Name::new("Player"),
    ));
}

fn player_movement_system(
    mut characters: Query<(&mut Transform, &mut Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in &mut characters {
        if input.just_pressed(KeyCode::Space) {
            // TODO: bad evil constant
            player.fall_speed = JUMP_SPEED - (player.fall_speed * 0.15);
        }

        if input.pressed(KeyCode::Tab) {
            transform.translation.y = 0.0;
            player.fall_speed = 0.0;
        }

        let movement_amount = player.fall_speed * time.delta_seconds();
        player.fall_speed = player.fall_speed - GRAVITY;
        transform.translation.y += movement_amount;
    }
}
