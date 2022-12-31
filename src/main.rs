mod particles;
mod skybox;

use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::window::PresentMode;
use bevy_hanabi::ParticleEffect;
use rand::prelude::*;
use std::time::Duration;

use bevy::{core_pipeline::bloom::BloomSettings, prelude::*, render::camera::Projection};
use bevy_rapier3d::prelude::*;

use crate::particles::{Firework, ParticlePlugin};
use crate::skybox::SkyboxPlugin;

/// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;
const MAX_SPEED: f32 = 30.0;
const ROT_SPEED: f32 = 3.0;
const ACCELERATION: f32 = 0.75;
const BULLET_SPEED: f32 = 300.0;
const BOUNDS_POS: Vec3 = Vec3::new(15.0, 8.0, 300.0);
const ENEMY_SPEED: f32 = 100.0;
const ENEMY_SPAWN_TIME: u64 = 1;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Resource)]
struct EnemySpawnTime {
    timer: Timer,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for CrosshairMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CrosshairMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Fake Star Fox".to_string(),
                width: 800.,
                height: 600.,
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugin(RapierDebugRenderPlugin::default()) // disable hdr to use
        .add_plugin(ParticlePlugin)
        .add_plugin(MaterialPlugin::<CrosshairMaterial>::default())
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugin(SkyboxPlugin)
        .add_startup_system(setup)
        .add_system(move_player)
        .add_system(velocity_movement)
        .add_system(fire_bullet)
        .add_system(spawn_enemies)
        .add_system(display_events)
        .add_system(handle_bullet_events)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CrosshairMaterial>>,
    asset_server: Res<AssetServer>,
) {
    const HALF_SIZE: f32 = 1.0;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true, // disable to use rapier debug render pipeline
                ..default()
            },
            projection: Projection::Perspective(PerspectiveProjection {
                fov: deg_to_rad(70.0),
                near: 0.05,
                far: 300.0,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        BloomSettings {
            intensity: 0.05,
            ..default()
        },
    ));

    let quad_mesh_5 = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(5.0, 5.0))));
    let color_crosshair = Color::GREEN;

    commands
        .spawn((
            SceneBundle {
                scene: asset_server.load("models/Spaceship/player.gltf#Scene0"),
                transform: Transform::from_xyz(0.0, 0.0, -15.0),
                ..default()
            },
            Player,
            Velocity(Vec3::ZERO),
        ))
        .with_children(|parent| {
            // crosshair1
            parent.spawn(MaterialMeshBundle {
                mesh: quad_mesh_5.clone(),
                transform: Transform::from_xyz(0.0, 0.0, -60.0),
                material: materials.add(CrosshairMaterial {
                    color: color_crosshair,
                    color_texture: Some(asset_server.load("textures/crosshair1.png")),
                    alpha_mode: AlphaMode::Blend,
                }),
                ..default()
            });
            // crosshair2
            parent.spawn(MaterialMeshBundle {
                mesh: quad_mesh_5.clone(),
                transform: Transform::from_xyz(0.0, 0.0, -250.0),
                material: materials.add(CrosshairMaterial {
                    color: color_crosshair,
                    color_texture: Some(asset_server.load("textures/crosshair2.png")),
                    alpha_mode: AlphaMode::Blend,
                }),
                ..default()
            });
        });

    commands.insert_resource(EnemySpawnTime {
        timer: Timer::new(Duration::from_secs(ENEMY_SPAWN_TIME), TimerMode::Repeating),
    })
}
/// apply velocity to transform
fn velocity_movement(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &Velocity, Entity)>,
) {
    for (mut transform, velocity, entity) in query.iter_mut() {
        transform.translation += velocity.0 * TIME_STEP;

        // delete if too far away
        if transform.translation.z > BOUNDS_POS.z || transform.translation.z < -BOUNDS_POS.z {
            if let Some(entity_commands) = commands.get_entity(entity) {
                entity_commands.despawn_recursive();
            }
        }
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    let (mut player_transform, mut player_velocity) = query.single_mut();

    // store xy direction input in a vector based off WASD keys
    let x = keyboard_input.pressed(KeyCode::D) as i32 as f32
        - keyboard_input.pressed(KeyCode::A) as i32 as f32;
    let y = keyboard_input.pressed(KeyCode::W) as i32 as f32
        - keyboard_input.pressed(KeyCode::S) as i32 as f32;
    let input_movement_vector = Vec3::new(x, y, 0.0).normalize_or_zero();

    // apply input to velocity
    player_velocity.0 = move_toward(
        player_velocity.0,
        input_movement_vector * MAX_SPEED,
        ACCELERATION,
    );

    // clamp to bounds
    player_transform.translation.x = player_transform.translation.x.clamp(-15.0, 15.0);
    player_transform.translation.y = player_transform.translation.y.clamp(-8.0, 8.0);

    // rotation_degrees.z = move_toward(rotation_degrees.z, target_z_rot, ROTSPEED)
    let player_eulers = Vec3::from(player_transform.rotation.to_euler(EulerRot::XYZ));
    let target_z_rot = player_velocity.0.x * -2.0;
    let new_z_rot = move_toward_f32(player_eulers.z, deg_to_rad(target_z_rot), ROT_SPEED);

    player_transform.rotation = Quat::from_euler(
        EulerRot::XYZ,
        deg_to_rad(player_velocity.0.y / 2.0),
        deg_to_rad(player_velocity.0.x / -2.0),
        new_z_rot,
    );
}

fn fire_bullet(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    query: Query<&Transform, With<Player>>,
) {
    // if spacebar is pressed
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    let player_transform = query.single();

    let bullet_transform = Transform {
        translation: player_transform.translation + player_transform.forward() * 2.0,
        rotation: player_transform.rotation,
        ..default()
    };

    // spawn bullet
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/Spaceship/bullet.gltf#Scene0"),
            transform: bullet_transform,
            ..default()
        },
        Velocity(player_transform.forward() * BULLET_SPEED),
        Bullet,
        Collider::cuboid(0.494, 0.494, 2.144),
        Sensor,
    ));
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

/// A system that displays the Collision events
fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}

fn handle_bullet_events(
    query_bullet: Query<(Entity, &Transform), With<Bullet>>,
    mut contact_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    mut query_effect: Query<
        (&mut ParticleEffect, &mut Transform),
        (With<Firework>, Without<Bullet>),
    >,
) {
    let (mut firework_fx, mut firework_transform) = query_effect.single_mut();

    for contact_event in contact_events.iter() {
        for (bullet_entity, bullet_transform) in query_bullet.iter() {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                if h1 == &bullet_entity || h2 == &bullet_entity {
                    firework_transform.translation = bullet_transform.translation;
                    firework_fx.maybe_spawner().unwrap().reset();

                    if let Some(entity_commands) = commands.get_entity(*h1) {
                        entity_commands.despawn_recursive();
                    }
                    if let Some(entity_commands) = commands.get_entity(*h2) {
                        entity_commands.despawn_recursive();
                    }
                }
            }
        }
    }
}

/// Moves from toward to by the delta value and returns a new vector
fn move_toward(from: Vec3, to: Vec3, delta: f32) -> Vec3 {
    let mut result = to - from;
    let length = result.length();
    if length <= delta || length == 0.0 {
        return to;
    }
    result *= delta / length;
    result += from;
    result
}

// same as move toward but for f32
fn move_toward_f32(from: f32, to: f32, delta: f32) -> f32 {
    let mut result = to - from;
    let length = result.abs();
    if length <= delta || length == 0.0 {
        return to;
    }
    result *= delta / length;
    result += from;
    result
}

/// convert from degrees to radians
fn deg_to_rad(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}
