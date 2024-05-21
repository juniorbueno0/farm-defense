use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Resource, Debug)]
pub struct MyMouseCoords(pub Vec2);

#[derive(Component)]
pub struct MainCamera;

pub struct MouseInteractionsPlugin;

impl Plugin for MouseInteractionsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MyMouseCoords(Vec2 { x:0., y:0. }))
            .add_systems(Update, cursor_to_world_position);
    }
}

fn cursor_to_world_position(
    mut mycoords: ResMut<MyMouseCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position + Vec2::new(0.5, 0.5);
        // to int 
        let x_value: i32 = mycoords.0.x as i32;
        let y_value: i32 = mycoords.0.y as i32;
        // to f32 
        mycoords.0 = Vec2::new(x_value as f32, y_value as f32);
    }
}