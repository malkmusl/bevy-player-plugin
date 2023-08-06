use bevy::prelude::*;

use super::player::{Player, PlayerDirection, PlayerMoving};

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,(
            spawn_camera,
        ));
        app.add_systems(Update, (
            character_movement,
            camera_follow.after(character_movement),
            ));
    }
}

fn spawn_camera(
    mut commands: Commands
){
    commands.spawn(Camera2dBundle::default());
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn character_movement(
    mut characters: Query<(
        &mut Transform,
        &mut Player,
    )>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in &mut characters {
        let mut movement_vector = Vec2::ZERO;

        if input.pressed(KeyCode::W) {
            movement_vector.y += 1.0;
            player.facing = PlayerDirection::Up;
        }else if input.pressed(KeyCode::S) {
            movement_vector.y -= 1.0;
            player.facing = PlayerDirection::Down;
        }else if input.pressed(KeyCode::D) {
            movement_vector.x += 1.0;
            player.facing = PlayerDirection::Right;
            
        }else if input.pressed(KeyCode::A) {
            movement_vector.x -= 1.0;
            player.facing = PlayerDirection::Left;
        }

        // Normalize the movement vector to ensure diagonal movement is not faster
        movement_vector = movement_vector.normalize_or_zero();

        // Update player movement state
        if movement_vector != Vec2::ZERO {
            player.is_moving = PlayerMoving::TRUE;
        }else{
            player.is_moving =  PlayerMoving::FALSE;
        }
        

        // Move the player
        let mut movement_amount = player.speed * time.delta_seconds();
        let multiplyer = 2 as f32;
        let new_movement_amount = movement_amount * multiplyer;

        if input.pressed(KeyCode::ShiftLeft) {
            movement_amount = new_movement_amount;
        }
        
        transform.translation.x += movement_vector.x * movement_amount;
        transform.translation.y += movement_vector.y * movement_amount;
    }
}