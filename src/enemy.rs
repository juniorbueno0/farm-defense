use bevy::{math::vec2, prelude::*};

// #[derive(Resource, Debug)]
// struct Metas(Vec<Vec2>);

#[derive(Component)]
struct EnemyData {
    meta_reached: bool,
    actual_meta: Vec2,
    meta_state: i32
}

use crate::worldtile::Metas;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Metas(vec![]))
            // .add_systems(Startup, setup)
            .add_systems(Update, (spawn_enemies, movement, print_metas));
    }
}

fn _setup() { }

fn spawn_enemies(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    metas: Res<Metas>,
) {
    if input.just_pressed(KeyCode::KeyY) {
        commands.spawn(
            SpriteBundle {
            sprite: Sprite{color:Color::Rgba{red:0.4,green:0.6,blue:0.7,alpha:1.},..default()},
            transform: Transform::from_xyz(metas.0[0].x, metas.0[0].y, 1.),
            ..default()
        }).insert(EnemyData{meta_reached:true,actual_meta:vec2(0., 0.),meta_state:0});
    }
}

fn print_metas(input: Res<ButtonInput<KeyCode>>, metas: Res<Metas>,mut commands: Commands,) {
    if input.just_pressed(KeyCode::KeyP) {
        for meta in metas.0.iter() {
            println!("{:?}", metas);
            commands.spawn(
                SpriteBundle {
                sprite: Sprite{color:Color::Rgba{red:0.4,green:0.8,blue:0.4,alpha:1.},..default()},
                transform: Transform::from_xyz(meta.x, meta.y, 2.),
                ..default()
            });
        }
    }
}

fn movement(
    mut query: Query<(&mut Transform, &mut EnemyData), With<EnemyData>>,
    metas: Res<Metas>,
    time: Res<Time>
) {
    for mut enemy in query.iter_mut() {
        
        if enemy.0.translation.x > enemy.1.actual_meta.x {
            enemy.0.translation.x -= 1.5 * time.delta_seconds(); 
        }
        if enemy.0.translation.x < enemy.1.actual_meta.x {
            enemy.0.translation.x += 1.5  * time.delta_seconds(); 
        }
        if enemy.0.translation.y > enemy.1.actual_meta.y {
            enemy.0.translation.y -= 1.5 * time.delta_seconds(); 
        }
        if enemy.0.translation.y < enemy.1.actual_meta.y {
            enemy.0.translation.y += 1.5  * time.delta_seconds(); 
        }
        
        if enemy.1.meta_state == 0 {
            enemy.1.actual_meta = metas.0[enemy.1.meta_state as usize];
        }
        if enemy.1.meta_state == 1 { enemy.1.actual_meta = metas.0[enemy.1.meta_state as usize]; }
        if enemy.1.meta_state == 2 { enemy.1.actual_meta = metas.0[enemy.1.meta_state as usize]; }
        if enemy.1.meta_state == 3 { enemy.1.actual_meta = metas.0[enemy.1.meta_state as usize]; }
        if enemy.1.meta_state == 4 { enemy.1.actual_meta = metas.0[enemy.1.meta_state as usize]; }
        if enemy.1.meta_state == 5 { enemy.1.actual_meta = metas.0[enemy.1.meta_state as usize]; }
        if enemy.1.meta_state == 6 { enemy.1.actual_meta = metas.0[enemy.1.meta_state as usize]; }
        if enemy.1.meta_state == 7 { enemy.1.actual_meta = metas.0[enemy.1.meta_state as usize]; }
        if enemy.1.meta_state == 8 { enemy.1.actual_meta = metas.0[enemy.1.meta_state as usize]; }
        if enemy.1.meta_state == 9 { enemy.1.actual_meta = metas.0[enemy.1.meta_state as usize]; }

        if (enemy.0.translation.y > enemy.1.actual_meta.y - 0.1) && (enemy.0.translation.y < enemy.1.actual_meta.y + 0.1) {
            if (enemy.0.translation.x > enemy.1.actual_meta.x - 0.1) && (enemy.0.translation.x < enemy.1.actual_meta.x + 0.1) {
                enemy.1.meta_reached = true;
                println!("meta reached");
            } 
        }
        
        if enemy.1.meta_reached {
            enemy.1.meta_state+=1;
            println!("new state: {:?}", enemy.1.meta_state);
            println!("actual position: {:?}", enemy.0.translation);
            println!("next position: {:?}", enemy.1.actual_meta);
            enemy.1.meta_reached = false;
        }
    }
}