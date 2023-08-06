use bevy::prelude::*;

use crate::plugins::player::player_animation::{AnimationIndices, AnimationTimer};

use super::player_controller::PlayerControllerPlugin;
use super::player_animation::PlayerAnimationPlugin;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PlayerControllerPlugin,
            PlayerAnimationPlugin
        ));
        app.add_systems(Startup, spawn_player);
    }
}

#[derive(Clone, Copy)]
pub enum PlayerGender {
    MALE,
    FEMALE
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum PlayerDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component,Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlayerMoving {
    TRUE,
    FALSE,
}

#[derive(Component)]
pub struct Player{
    pub speed: f32,
    pub gender: PlayerGender,
    pub facing: PlayerDirection,
    pub prev_facing: PlayerDirection,
    pub is_moving: PlayerMoving
}

fn spawn_player(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    
    let gender = PlayerGender::FEMALE; // Replace this with the actual gender of the player
    
    let texture_handle = match gender {
        PlayerGender::MALE => asset_server.load("male-run-test.png"),
        PlayerGender::FEMALE => asset_server.load("female-run-test.png"),
    };

    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(40.0, 42.0), 16, 1, Some(Vec2 { x: 0.1, y: 0.0 }), Some(Vec2 { x: 0.0, y: 0.0 }));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    //? Player animation 0-3 Down, 4-7 Left, 8-11 Right, 12 - 15 Up  
    let animation_indices = AnimationIndices { first: 0, last: 15 };
    info!("AnimationIndices: {} - {}", animation_indices.first, animation_indices. last);
    
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        Player { 
            speed: 200.0,
            gender: PlayerGender::FEMALE,
            facing: PlayerDirection::Down,
            prev_facing: PlayerDirection::Down,
            is_moving: PlayerMoving::FALSE
        },
    ));
}