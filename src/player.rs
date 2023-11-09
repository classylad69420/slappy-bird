use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
struct Player {
    fall_speed: f32,
}

const GRAVITY: f32 = 9.8;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement_system);
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-75.0, 0.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(20.0, 20.0)),
                color: Color::rgba(1.0, 1.0, 0.0, 1.0),
                ..default()
            },
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
            player.fall_speed = 200.0 - (player.fall_speed * 0.15);
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
