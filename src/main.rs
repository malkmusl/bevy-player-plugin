use bevy::prelude::Commands;
use bevy::prelude::*;
use player::player::PlayerPlugin;

mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PlayerPlugin) // prevents blurry sprites
        .run();
}

