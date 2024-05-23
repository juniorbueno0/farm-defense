use std::time::Duration;

use bevy::math::vec3;
use bevy::{math::vec2, prelude::*};
use rand::prelude::*;

const WORLD_SIZE_X: i32 = 20;
const WORLD_SIZE_Y: i32 = 20;
const TOTAL_SIDEWAYS: i32 = 10;

#[derive(Component, Debug)]
struct WorldTile;

#[derive(Component, Debug)]
struct PathTile;

#[derive(Component)]
struct MouseSelection;

#[derive(Resource, Debug)]
pub struct Metas(pub Vec<Vec2>);

#[derive(Component, Debug, Clone)]
pub enum ObjectType {
    Potato,
    Empty,
    TurretA
}

#[derive(Component, Debug)]
struct CropComponent {
    is_ready: bool,
    age: f32
}

#[derive(Component, Debug)]
struct TurretComponent {
    cooldown: Timer,
    enemy_nearby: bool,
    enemy_dir: Vec2,
    range: i32
}

#[derive(Component, Debug)]
struct BulletComponent {
    life_time: f32,
    direction: Vec2
}

use crate::enemy::EnemyData;
use crate::mouse::MyMouseCoords;
use crate::ui::ObjectSelected; // object selected from the btn

pub struct WorldTilePlugin;

impl Plugin for WorldTilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_map)
            .add_systems(Update, (draw_path, mouse_highligth, spawn_object, turret, bullet_manager));
    }
}

fn draw_map(mut commands: Commands) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;   
    while WORLD_SIZE_Y > y {
        while WORLD_SIZE_X > x {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::Rgba {red:0.4,green:0.4,blue:0.8,alpha:1.},
                    custom_size: Some(Vec2 {x:1.,y:1.}),
                    ..default()
                },
                transform: Transform::from_xyz(x as f32, y as f32, 0.),
                ..default()
            }).insert(WorldTile);
            x += 1;
        }
        x = 0;
        y += 1;
    }

    // mouse pixel selected
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::Rgba {red:1.,green:1.,blue:1.,alpha:0.3},
            custom_size: Some(Vec2 {x:1.,y:1.}),
            ..default()
        },
        ..default()
    }).insert(MouseSelection);
}

fn draw_path(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<Entity, With<PathTile>>,
    mut metas: ResMut<Metas>
) {
    if input.just_pressed(KeyCode::KeyB) {    
        
        for tile in query.iter_mut() {
            commands.entity(tile).despawn();
        }

        // clear the ols stopping points - testing only 
        metas.0 = vec![];
 
        let mut rng = thread_rng();

        let mut y: i32 = 0; 
        let mut i: i32 = 0;
        let mut a: i32 = 0;

        let mut last_x_pos: i32 = 0;
        let max_fordward_tiles:i32 = WORLD_SIZE_Y / TOTAL_SIDEWAYS;

        while TOTAL_SIDEWAYS > i {
            let x: u32 = rng.gen_range(1..WORLD_SIZE_X as u32 - 2);
            let mut flag_b: bool = true;
            
            while max_fordward_tiles > y {
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::Rgba {red:0.4,green:0.4,blue:0.6,alpha:1.},
                        custom_size: Some(Vec2 {x:1.,y:1.}),
                        ..default()
                    },
                    transform: Transform::from_xyz(x as f32, a as f32, 1.),
                    ..default()
                }).insert(PathTile);

                a += 1;
                y += 1;
                
                if flag_b || (max_fordward_tiles == y) {  
                    if flag_b {
                        metas.0.push(vec2(x as f32, a as f32 - 1.));        
                    }else {
                        metas.0.push(vec2(x as f32, a as f32));
                    }
                }

                // connect the new path 
                if last_x_pos != 0 && flag_b {
                    let bridge_diference: i32 = last_x_pos - x as i32;
                    let mut n: i32 = 0;
                    let mut x_update:i32 = last_x_pos;
                    
                    while n < bridge_diference.abs() {
                        commands.spawn(SpriteBundle {
                            sprite: Sprite {
                                color: Color::Rgba {red:0.8,green:0.8,blue:0.6,alpha:1.},
                                custom_size: Some(Vec2 {x:1.,y:1.}),
                                ..default()
                            },
                            transform: Transform::from_xyz(x_update as f32, a as f32 - 1., 1.),
                            ..default()
                        }).insert(PathTile);
                        
                        if (x as i32) < last_x_pos { 
                            x_update-=1; 
                        }else { 
                            x_update+=1; 
                        }
                        n+=1;
                    }
                    flag_b = false;
                }
            }
            last_x_pos = x as i32;
            y = 0;
            i += 1;
        }
    }
}

fn spawn_object(
    mut commands: Commands,
    mouse_coords: Res<MyMouseCoords>,
    input: Res<ButtonInput<MouseButton>>,
    mut object_selected: ResMut<ObjectSelected>,
) {
    if input.just_pressed(MouseButton::Left) {
        match object_selected.0 {
            ObjectType::Empty => { println!("empty"); },
            ObjectType::Potato => { 
                commands.spawn(SpriteBundle{sprite:Sprite{color:Color::Rgba{red:0.1, green:0.6,blue:0.6,alpha:1.},..default()},transform:Transform::from_xyz(mouse_coords.0.x, mouse_coords.0.y, 1.),..default()});
                object_selected.0 = ObjectType::Empty;
            },
            ObjectType::TurretA => { 
                commands.spawn(SpriteBundle{
                    sprite:Sprite{color:Color::Rgba{red:1., green:1.,blue:0.8,alpha:1.},..default()},
                    transform:Transform::from_xyz(mouse_coords.0.x, mouse_coords.0.y, 1.), ..default()}
                ).insert(TurretComponent{enemy_dir:vec2(0., 0.),range:4,enemy_nearby:true,cooldown:Timer::from_seconds(2., TimerMode::Repeating)});
                object_selected.0 = ObjectType::Empty;
             } 
        }
    }
}

fn turret(
    mut t_query: Query<(&mut TurretComponent, &Transform), With<TurretComponent>>,
    e_query: Query<&Transform, With<EnemyData>>,
    mut commands: Commands,
    time: Res<Time>
) {
    for mut t in t_query.iter_mut() {
        for e in e_query.iter() {

            let x_distance: f32 = e.translation.x - t.1.translation.x;
            let y_distance: f32 = e.translation.y - t.1.translation.y;

            if (x_distance <= t.0.range as f32) && (y_distance <= t.0.range as f32) {
                t.0.cooldown.tick(Duration::from_secs_f32(6. * time.delta_seconds_f64() as f32));
                if t.0.cooldown.finished() {
                    commands.spawn(SpriteBundle{
                        sprite:Sprite{color:Color::Rgba{red:1., green:1.,blue:0.8,alpha:1.},..default()},
                        transform:Transform::from_xyz(t.1.translation.x, t.1.translation.y, 3.), ..default()}
                    ).insert(
                        BulletComponent{life_time:0.025,direction:Vec2::new(e.translation.x, e.translation.y)}
                    );
                }
            }

        }
    }
    
}

fn bullet_manager(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut BulletComponent, &mut Transform), With<BulletComponent>>
) {
    for mut b in query.iter_mut() {
        // lifetime
        b.1.life_time -= 0.1 * time.delta_seconds_f64() as f32;
        if b.1.life_time <= 0. { 
            commands.entity(b.0).despawn(); 
        }
        let a = b.2.translation;
        // speed
        b.2.translation += (vec3(b.1.direction.x - a.x,b.1.direction.y - a.y,0.) * 10.) * time.delta_seconds_f64() as f32;
    }
}

fn mouse_highligth(
    mouse_coords: Res<MyMouseCoords>,
    mut query_mouse: Query<&mut Transform, With<MouseSelection>>
) {
    for mut pixel in query_mouse.iter_mut() {
        pixel.translation = Vec3::new(mouse_coords.0.x, mouse_coords.0.y,1.);
    }
}