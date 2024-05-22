use bevy::prelude::*;

mod ui;
mod mouse;
mod enemy;
mod camera;
mod worldtile;
use crate::enemy::EnemyPlugin;
use crate::mouse::MouseInteractionsPlugin;
use crate::ui::UiPlugin;
use crate::camera::CameraPlugin;
use crate::worldtile::WorldTilePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MouseInteractionsPlugin)
        .add_plugins(WorldTilePlugin)
        .add_plugins(EnemyPlugin)
        .run();
}

// PENDING 
// add perlin to generate trees etc cannot build there 
// 