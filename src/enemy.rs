use bevy::{math::vec2, prelude::*};
use bevy_xpbd_2d::{components::RigidBody, plugins::collision::Collider};

const ENEMY_SPEED: f32 = 2.0;

#[derive(Component)]
pub struct EnemyData {
    meta_reached: bool,
    actual_meta: Vec2,
    meta_state: i32,
    pub health: f32
}

use crate::worldtile::Metas;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Metas(vec![]))
            .add_systems(Update, (spawn_enemies, movement, print_metas));
    }
}

fn spawn_enemies(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    metas: Res<Metas>,
) {
    if input.just_pressed(KeyCode::KeyY) {
        commands.spawn((
            RigidBody::Static,
            Collider::rectangle(2.5, 2.5),
            SpriteBundle {
            sprite: Sprite{color:Color::Rgba{red:0.4,green:0.6,blue:0.7,alpha:1.},..default()},
            transform: Transform::from_xyz(metas.0[0].x, metas.0[0].y, 2.),
            ..default()}
        )).insert(EnemyData{meta_reached:true,actual_meta:vec2(0., 0.),meta_state:0,health:100.});
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

// need refactorization
fn movement(
    mut query: Query<(&mut Transform, &mut EnemyData, Entity), With<EnemyData>>,
    mut commands: Commands,
    metas: Res<Metas>,
    time: Res<Time>
) {
    for mut enemy in query.iter_mut() {
        
        if enemy.0.translation.x > enemy.1.actual_meta.x { enemy.0.translation.x-=ENEMY_SPEED*time.delta_seconds(); }
        if enemy.0.translation.x < enemy.1.actual_meta.x { enemy.0.translation.x+=ENEMY_SPEED*time.delta_seconds(); }
        if enemy.0.translation.y > enemy.1.actual_meta.y { enemy.0.translation.y-=ENEMY_SPEED*time.delta_seconds(); }
        if enemy.0.translation.y < enemy.1.actual_meta.y { enemy.0.translation.y+=ENEMY_SPEED*time.delta_seconds(); }

        enemy.1.actual_meta = match enemy.1.meta_state {
            0 => metas.0[0],
            1 => metas.0[1],
            2 => metas.0[2],
            3 => metas.0[3],
            4 => metas.0[4],
            5 => metas.0[5],
            6 => metas.0[6],
            7 => metas.0[7],
            8 => metas.0[8],
            9 => metas.0[9],
            10 => metas.0[10],
            11 => metas.0[11],
            12 => metas.0[12],
            13 => metas.0[13],
            14 => metas.0[14],
            15 => metas.0[15],
            16 => metas.0[16],
            17 => metas.0[17],
            18 => metas.0[18],
            19 => metas.0[19],
            _ => {
                commands.entity(enemy.2).despawn();
                Default::default()
            }
        };

        if (enemy.0.translation.y > enemy.1.actual_meta.y - 0.05) && 
            (enemy.0.translation.y < enemy.1.actual_meta.y + 0.05) {
            if (enemy.0.translation.x > enemy.1.actual_meta.x - 0.05) && 
                (enemy.0.translation.x < enemy.1.actual_meta.x + 0.05) {
                    enemy.1.meta_reached = true;
            } 
        }
        
        if enemy.1.meta_reached && (enemy.1.meta_state <= 19) {
            enemy.1.meta_state += 1;
            enemy.1.meta_reached = false;
            println!("new state: {:?}", enemy.1.meta_state);
            println!("actual position: {:?}", enemy.0.translation);
            println!("next position: {:?}", enemy.1.actual_meta);
        }

    }
}