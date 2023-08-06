use bevy::prelude::*;

use super::player::{Player, PlayerDirection, PlayerMoving};

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
            animate_player_sprite
        );
    }
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);


fn animate_player_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut Player,
    )>,
    input: Res<Input<KeyCode>>,
) {
    for (_indices, mut timer, mut sprite, mut player) in &mut query {
        let mut first_frame = 0;
        let mut last_frame = 0;

        match player.facing {
            PlayerDirection::Down => {
                first_frame = 0;
                last_frame = 3;
            }
            PlayerDirection::Left => {
                first_frame = 4;
                last_frame = 7;
            }
            PlayerDirection::Right => {
                first_frame = 8;
                last_frame = 11;
            }
            PlayerDirection::Up => {
                first_frame = 12;
                last_frame = 15;
            }
        }

        // Reset the animation timer when the player changes direction
        if player.facing != player.prev_facing {
            timer.reset();
            sprite.index = first_frame; // Update the sprite index immediately
        }

        player.prev_facing = player.facing;

        // Update the animation only when the player is moving
        if player.is_moving == PlayerMoving::TRUE {
            timer.tick(time.delta());
            if timer.just_finished() {
                sprite.index = if sprite.index >= last_frame {
                    first_frame
                } else {
                    sprite.index + 1
                };
            }
        }else{
            sprite.index = first_frame;
        }
    }
}
