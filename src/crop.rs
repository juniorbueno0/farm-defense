use std::time::Duration;

use bevy::{math::vec3, prelude::*};

#[derive(Component, Debug)]
pub struct CropComponent {
    pub is_ready: bool, 
    pub hydrated: f32,
    pub grow: f32
}

use crate::worldtile::SprinklerComponent;

pub struct CropPlugin;

impl Plugin for CropPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, grow_manager); 
    }
}

fn grow_manager(
    _commands: Commands,
    mut c_query: Query<(&Transform, &mut CropComponent), With<CropComponent>>,
    mut s_query: Query<(&Transform, &mut SprinklerComponent), With<SprinklerComponent>>,
    time: Res<Time>
) {
    for mut crop in c_query.iter_mut() {
        if crop.1.is_ready { return; }
        

        for mut s in s_query.iter_mut() {
            s.1.cooldown.tick(Duration::from_secs_f32(1. * time.delta_seconds_f64() as f32));
            if s.1.cooldown.finished() {
                if s.0.translation + vec3(1.,0.,0.) == crop.0.translation {
                    crop.1.hydrated += 5.;
                    println!("{:?}", crop.1);
                }
                if s.0.translation - vec3(1.,0.,0.) == crop.0.translation {
                    crop.1.hydrated += 5.;
                    println!("{:?}", crop.1);
                }
                if s.0.translation + vec3(0.,1.,0.) == crop.0.translation {
                    crop.1.hydrated += 5. ;
                    println!("{:?}", crop.1);
                }
                if s.0.translation - vec3(0.,1.,0.) == crop.0.translation {
                    crop.1.hydrated += 5.;
                    println!("{:?}", crop.1);
                }
                
            }
        }

        if crop.1.hydrated >= 0.0 {
            crop.1.grow -= 1.* time.delta_seconds_f64() as f32; 
            crop.1.hydrated -= 1.* time.delta_seconds_f64() as f32; 
        }

        if crop.1.grow <= 0. {
            crop.1.is_ready = true;
            println!("{:?}", crop.1);
        }
    }    


}


// if s.0.translation + vec3(1.,0.,0.) == crop.0.translation {
//     crop.1.hydrated += 10. * time.delta_seconds_f64() as f32;
// }
// if s.0.translation - vec3(1.,0.,0.) == crop.0.translation {
//     crop.1.hydrated += 10. * time.delta_seconds_f64() as f32;
// }
// if s.0.translation + vec3(0.,1.,0.) == crop.0.translation {
//     crop.1.hydrated += 10. * time.delta_seconds_f64() as f32;
// }
// if s.0.translation - vec3(0.,1.,0.) == crop.0.translation {
//     crop.1.hydrated += 10. * time.delta_seconds_f64() as f32;
// }