use std::time::Duration;

use bevy::input::mouse;
use bevy::{math::vec3, prelude::*};
use bevy_xpbd_2d::plugins::spatial_query::RayCaster;
use bevy_xpbd_2d::plugins::spatial_query::RayHits;

#[derive(Component, Debug)]
pub struct CropComponent {
    pub is_ready: bool, 
    pub hydrated: f32,
    pub grow: f32
}

use crate::worldtile::SprinklerComponent;
use crate::worldtile::RaycastInputClick;
use crate::mouse::MyMouseCoords;

pub struct CropPlugin;

impl Plugin for CropPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (grow_manager, handle_raycast_click, raycast_hits)); 
    }
}

fn grow_manager(
    _commands: Commands,
    mut c_query: Query<(&Transform, &mut CropComponent, &mut Sprite), With<CropComponent>>,
    mut s_query: Query<(&Transform, &mut SprinklerComponent), With<SprinklerComponent>>,
    time: Res<Time>
) {
    for mut s in s_query.iter_mut() {
        s.1.cooldown.tick(Duration::from_secs_f32(1. * time.delta_seconds_f64() as f32));

        for mut crop in c_query.iter_mut() {
            println!("{:?}", crop.1);

            if s.1.cooldown.finished() {
            
                let _a: Vec3 = s.0.translation - vec3(0.,1.,0.);
                let _b: Vec3 = s.0.translation + vec3(0.,1.,0.);
                let _c: Vec3 = s.0.translation - vec3(1.,0.,0.);
                let _d: Vec3 = s.0.translation + vec3(1.,0.,0.);
                
                crop.1.hydrated += match crop.0.translation {
                    _a => { 5. }
                    _b => { 5. }
                    _c => { 5. }
                    _d => { 5. }
                };
            }

            if crop.1.is_ready { return; }
    
            if crop.1.grow <= 0. {
                crop.1.is_ready = true;
                crop.2.color = Color::rgb(0.1,0.8,0.9);
            }else if crop.1.hydrated >= 0.0 {
                crop.1.grow -= 1.* time.delta_seconds_f64() as f32; 
                crop.1.hydrated -= 1.* time.delta_seconds_f64() as f32; 
            }
        }
    }

}

fn handle_raycast_click( // despawn the raycaster that check the crops on click
    time: Res<Time>,
    mut commands: Commands,
    mut u_query: Query<(Entity, &mut RaycastInputClick), With<RaycastInputClick>>
) {
    for mut e in u_query.iter_mut() {
        e.1.life_time.tick(Duration::from_secs_f32(1.* time.delta_seconds_f64() as f32) );
        if e.1.life_time.finished() {
            commands.entity(e.0).despawn();    
        }
    }
}

fn raycast_hits(
    mut commands: Commands,
    query: Query<(&RayCaster, &RayHits)>,
    mouse_coords: Res<MyMouseCoords>,
    mut c_query: Query<(Entity, &mut CropComponent, &Transform), With<CropComponent>>
) {
    for (ray, hits) in &query {
        //iter() iter_sorted()
        for hit in hits.iter() {
            println!(
                "Hit entity {:?} at {} with normal {}",
                hit.entity,
                ray.origin + *ray.direction * hit.time_of_impact,
                hit.normal,
            );

            for crop in c_query.iter_mut() {
                if crop.1.is_ready && (Vec2{x:crop.2.translation.x,y:crop.2.translation.y} == Vec2{x:mouse_coords.0.x,y:mouse_coords.0.y}){
                    commands.entity(crop.0).despawn();
                }
            }
        }
    }
}