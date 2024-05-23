use bevy::prelude::*;

mod ui;
mod mouse;
mod enemy;
mod camera;
mod worldtile;
use crate::ui::UiPlugin;
use crate::enemy::EnemyPlugin;
use crate::camera::CameraPlugin;
use crate::worldtile::WorldTilePlugin;
use crate::mouse::MouseInteractionsPlugin;

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
// B draw the path
// Y spawn the entity that follow the path
// P print stopping points 

// PENDING 
// add perlin to generate trees etc cannot build there 
// day night cicle 
// vegetables can be ahrvested 1 tiem a day 
// gambling machine 
// crops can be harvested and selled 
// build turrets
// enemies give money too
// 

// turrets
// detect the closest enemy
// select the closest enemy and get the position 
// if there is no enemys in the range cant shoot


// enemy path following
// fix the enemy stopping points please when gets to the max, harcoded stopping points it stops

// change the part of the states (the hardcoded one)
// use the total_sideways multiplied by 2 and there you have the toal stoping points