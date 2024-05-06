use crate::pipes::Pipe;
use crate::player::Player;
use crate::states::AppState;
use bevy::{prelude::*, sprite::collide_aabb::collide};

#[derive(Component)]
pub struct Hitbox {
    pub scale: Vec2,
}

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                check_for_collisions_system.run_if(in_state(AppState::InGame)),
                check_for_ground_system.run_if(in_state(AppState::InGame)),
            ),
        );
    }
}

fn check_for_collisions_system(
    mut player_query: Query<(&Transform, &Hitbox), With<Player>>,
    pipes_query: Query<(&Transform, &Hitbox), With<Pipe>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let player_transform_result = player_query.get_single_mut();
    if let Ok((player_transform, player_hitbox)) = player_transform_result {
        for (pipe_transform, pipe_hitbox) in &pipes_query {
            let collision = collide(
                player_transform.translation,
                player_hitbox.scale,
                pipe_transform.translation,
                pipe_hitbox.scale,
            );
            if collision.is_some() {
                next_state.set(AppState::GameOver);
            }
        }
    }
}

fn check_for_ground_system(
    mut player_query: Query<&Transform, With<Player>>,
    window_query: Query<&Window>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let player_transform_result = player_query.get_single_mut();
    let window = window_query.single();
    match player_transform_result {
        Ok(player_transform) if player_transform.translation.y <= -window.height() / 2.0 => {
            next_state.set(AppState::GameOver);
        }
        _ => {}
    }
}
