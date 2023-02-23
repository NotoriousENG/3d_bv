use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use crate::constants::BOUNDS_POS;
use crate::events::TeardownLevelEvent;
use crate::math::deg_to_rad;
use crate::velocity::Velocity;

const ENEMY_SPEED: f32 = 100.0;
const ENEMY_SPAWN_TIME: u64 = 1;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(spawn_enemies)
            .add_system(on_teardown);
    }
}

#[derive(Component)]
struct Enemy;
#[derive(Resource)]
struct EnemySpawnTime {
    timer: Timer,
}

fn setup(mut commands: Commands) {
    commands.insert_resource(EnemySpawnTime {
        timer: Timer::new(Duration::from_secs(ENEMY_SPAWN_TIME), TimerMode::Repeating),
    })
}

fn on_teardown(
    mut commands: Commands,
    query: Query<Entity, With<Enemy>>,
    mut ev_teardown: EventReader<TeardownLevelEvent>,
) {
    for _ in ev_teardown.iter() {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut spawn_timer: ResMut<EnemySpawnTime>,
) {
    spawn_timer.timer.tick(time.delta());

    let mut rng = rand::thread_rng();

    let x_spawn: f32 = rng.gen_range(-BOUNDS_POS.x..BOUNDS_POS.x);
    let y_spawn: f32 = rng.gen_range(-BOUNDS_POS.y..BOUNDS_POS.y);
    let vec_spawn = Vec3::new(x_spawn, y_spawn, -BOUNDS_POS.z + 1.0);

    let mut transform_spawn = Transform::from_translation(vec_spawn);
    transform_spawn.rotate_y(deg_to_rad(180.0));

    if spawn_timer.timer.finished() {
        commands.spawn((
            SceneBundle {
                scene: asset_server.load("models/Spaceship/enemy.gltf#Scene0"),
                transform: transform_spawn,
                ..default()
            },
            Enemy,
            Velocity(Vec3::Z * ENEMY_SPEED),
            Collider::cuboid(2.17, 1.45, 1.73),
            RigidBody::Dynamic,
            GravityScale(0.0),
            ActiveEvents::COLLISION_EVENTS,
        ));
    }
}
