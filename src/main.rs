use bevy::prelude::*;

mod ui;
mod mouse;
mod camera;
mod worldtile;
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
        .run();
}