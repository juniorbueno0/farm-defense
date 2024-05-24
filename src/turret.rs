use std::time::Duration;

#[derive(Component, Debug)]
struct BulletComponent {
    life_time: f32,
    direction: Vec2
}

use bevy::prelude::*;

use crate::enemy::EnemyData;
use crate::worldtile::TurretComponent;

pub struct TurretPlugin;

impl Plugin for TurretPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (turret, bullet_manager));
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
                t.0.cooldown.tick(Duration::from_secs_f32(5. * time.delta_seconds_f64() as f32));
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
        b.2.translation += (Vec3::new(b.1.direction.x - a.x,b.1.direction.y - a.y,0.) * 10.) * time.delta_seconds_f64() as f32;
    }
}