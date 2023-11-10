use crate::pipes::Pipe;
use crate::player::Player;
use bevy::{prelude::*, sprite::collide_aabb::collide};

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_for_collisions_system);
    }
}

fn check_for_collisions_system(
    mut player_query: Query<&Transform, With<Player>>,
    pipes_query: Query<&Transform, With<Pipe>>,
) {
    let player_transform = player_query.single_mut();
    for transform in &pipes_query {
        let collision = collide(
            player_transform.translation,
            player_transform.scale.truncate(),
            transform.translation,
            transform.scale.truncate(),
        );
        if collision.is_some() {
            // TODO: Game over state
            println!("You died bitch");
        }
    }
}
