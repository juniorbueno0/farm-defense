use bevy::prelude::*;
use bevy_xpbd_2d::plugins::PhysicsPlugins;

mod ui;
mod crop;
mod mouse;
mod enemy;
mod turret;
mod camera;
mod worldtile;
use crate::ui::UiPlugin;
use crate::crop::CropPlugin;
use crate::turret::TurretPlugin;
use crate::enemy::EnemyPlugin;
use crate::camera::CameraPlugin;
use crate::worldtile::WorldTilePlugin;
use crate::mouse::MouseInteractionsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(UiPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MouseInteractionsPlugin)
        .add_plugins(WorldTilePlugin)
        .add_plugins(CropPlugin)
        .add_plugins(TurretPlugin)
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

// turrets
// detect the closest enemy
// select the closest enemy and get the position 
// if there is no enemys in the range cant shoot

// change the part of the states (the hardcoded one)
// use the total_sideways multiplied by 2 and there you have the toal stoping points

// fix the hitboxes (sizes of the colliders)  

// fix the direction of the bullet

// crops 
// hydration keeps getting added FIXIT