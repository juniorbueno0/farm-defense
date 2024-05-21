use bevy::prelude::*;

use crate::mouse::MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, camera_movement);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            near: -1000.0,
            far: 1000.0,
            viewport_origin: Vec2::new(0.5, 0.5),
            scaling_mode: bevy::render::camera::ScalingMode::WindowSize(1.0),
            scale: 0.0332,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(
            0.,
            0.,
            1000.0,
        ))
        .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }).insert(MainCamera);
}

fn camera_movement(
    mut query: Query<&mut Transform, With<Camera2d>>,
    mut camera_query: Query<&mut OrthographicProjection, With<Camera2d>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    for mut p in query.iter_mut() {
        if input.pressed(KeyCode::KeyA) {
            if p.translation.x > 0. {
                p.translation += Vec3 { x:-10., y:0., z:0. } * time.delta_seconds();
            }
            println!("{:?}",p.translation);
        }
        if input.pressed(KeyCode::KeyD) {
            if p.translation.x < 19. {
                p.translation += Vec3 { x:10., y:0., z:0. } * time.delta_seconds();
            }
            println!("{:?}",p.translation);
        }
        if input.pressed(KeyCode::KeyW) {
            if p.translation.y < 10. {
                p.translation += Vec3 { x:0., y:10., z:0. } * time.delta_seconds();
            } 
            println!("{:?}",p.translation);
        }
        if input.pressed(KeyCode::KeyS) {
            if p.translation.y > 9. {
                p.translation += Vec3 { x:0., y:-10., z:0. } * time.delta_seconds();
            }
            println!("{:?}",p.translation);
        } 

        for mut c in camera_query.iter_mut() {
            if input.pressed(KeyCode::KeyE) {
                if c.scale < 0.0332 {
                    c.scale += 0.02 * time.delta_seconds();
                }
                println!("{:?}", c.scale);
            }
            if input.pressed(KeyCode::KeyQ) {
                if c.scale > 0.015 {
                    c.scale += -0.02 * time.delta_seconds();
                }
                println!("{:?}", c.scale);
            }   
        }

    }
}