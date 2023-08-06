use bevy::prelude::Commands;
use bevy::prelude::*;
use plugins::player::player::PlayerPlugin;


mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PlayerPlugin) // prevents blurry sprites
        .run();
}

