use bevy::{math::vec2, prelude::*};

#[derive(Resource, Debug)]
struct Metas(Vec<Vec2>);

#[derive(Component)]
struct EnemyData {
    meta_reached: bool,
    actual_meta: Vec2,
    meta_state: i32
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Metas(vec![]))
            .add_systems(Startup, setup)
            .add_systems(Update, (spawn_enemies, movement));
    }
}

fn setup(mut metas: ResMut<Metas>) {
    metas.0.push(Vec2::new(0.,0.));
    metas.0.push(Vec2::new(10.,0.));
    metas.0.push(Vec2::new(10.,10.));
    println!("{:?}", metas.0[0]);
}

fn spawn_enemies(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::KeyY) {
        commands.spawn(
            SpriteBundle {
            sprite: Sprite{color:Color::Rgba{red:0.4,green:0.6,blue:0.7,alpha:1.},..default()},
            transform: Transform::from_xyz(20., 0., 1.),
            ..default()
        }).insert(EnemyData{meta_reached:true,actual_meta:vec2(0., 0.),meta_state:0});
    }
}

fn movement(
    mut query: Query<(&mut Transform, &mut EnemyData), With<EnemyData>>,
    metas: Res<Metas>,
    time: Res<Time>
) {
    for mut enemy in query.iter_mut() {
        if enemy.0.translation.x > enemy.1.actual_meta.x {
            println!("x--");
            enemy.0.translation.x -= 1.5 * time.delta_seconds(); 
        }
        if enemy.0.translation.x < enemy.1.actual_meta.x {
            println!("x++");
            enemy.0.translation.x += 1.5  * time.delta_seconds(); 
        }
        if enemy.0.translation.y > enemy.1.actual_meta.y {
            println!("y--");
            enemy.0.translation.y -= 1.5 * time.delta_seconds(); 
        }
        if enemy.0.translation.y < enemy.1.actual_meta.y {
            println!("y++");
            enemy.0.translation.y += 1.5  * time.delta_seconds(); 
        }
        
        if enemy.1.meta_state == 0 {
            enemy.1.actual_meta = metas.0[enemy.1.meta_state as usize];
        }
        if enemy.1.meta_state == 1 {
            enemy.1.actual_meta = metas.0[enemy.1.meta_state as usize];
        }
        if enemy.1.meta_state == 2 {
            enemy.1.actual_meta = metas.0[enemy.1.meta_state as usize];
        }

        if enemy.1.meta_reached {
            enemy.1.meta_state+=1;
            enemy.1.meta_reached = false;
        }

        if (enemy.0.translation.x > enemy.1.actual_meta.x - 0.5) && (enemy.0.translation.x < enemy.1.actual_meta.x + 0.5) {
            // enemy.1.meta_reached = true;
            if (enemy.0.translation.y > enemy.1.actual_meta.y - 0.5) && (enemy.0.translation.y < enemy.1.actual_meta.y + 0.5) {
                enemy.1.meta_reached = true;
                println!("meta reached");
            } 
            // println!("meta reached");
        } 
        // if (enemy.0.translation.y > enemy.1.actual_meta.y - 0.5) && (enemy.0.translation.y < enemy.1.actual_meta.y + 0.5) {
        //     enemy.1.meta_reached = true;
        //     println!("meta reached");
        // } 
    }
}