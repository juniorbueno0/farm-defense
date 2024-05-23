use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct CropComponent {
    pub is_ready: bool,
    pub age: f32
}

pub struct CropPlugin;

impl Plugin for CropPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, grow_manager); 
    }
}

fn grow_manager(
    mut commands: Commands,
    c_query: Query<(&Transform, &CropComponent), With<CropComponent>>
) {
    for crop in c_query.iter() {
        println!("{:?}, {:?}", crop.1, crop.0.translation);      
    }
}