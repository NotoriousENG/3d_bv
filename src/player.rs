use bevy::prelude::*;

use crate::constants::{make_cam_entity, BOUNDS_POS};
use crate::events::{SpawnBulletEvent, SpawnPlayerEvent};
use crate::level::PlayerPath;
use crate::materials::ColorMaterial;
use crate::math::{deg_to_rad, move_toward, move_toward_f32};
use crate::velocity::Velocity;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(on_spawn_player)
            .add_system(move_player)
            .add_system(fire_bullet)
            .add_system(move_along_path)
            .add_plugin(MaterialPlugin::<ColorMaterial>::default());
    }
}

const MAX_SPEED: f32 = 30.0;
const ROT_SPEED: f32 = 3.0;
const ACCELERATION: f32 = 0.75;
const BULLET_SPEED: f32 = 300.0;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerRoot;

#[derive(Component)]
pub struct PathFollower {
    pub index: usize,
    pub distance_along_path: f32,
}

fn on_spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut ev_spawn: EventReader<SpawnPlayerEvent>,
    q: Query<Entity, With<Player>>,
    q_camera: Query<Entity, With<Camera>>,
) {
    for ev in ev_spawn.iter() {
        for entity in q.iter() {
            commands.entity(entity).despawn_recursive();
        }
        for entity in q_camera.iter() {
            commands.entity(entity).despawn_recursive();
        }

        let quad_mesh_5 = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(5.0, 5.0))));
        let color_crosshair = Color::GREEN;

        let offset = 15.0;

        let mut root_transform = ev.transform.clone();
        root_transform.translation += Vec3::Z * offset;

        let mut ship_transform = Transform::IDENTITY;
        ship_transform.translation -= Vec3::Z * offset;

        commands
            .spawn((
                PlayerRoot,
                PathFollower {
                    index: 0,
                    distance_along_path: 0.0,
                },
                Velocity(Vec3::ZERO),
                SceneBundle {
                    transform: root_transform.clone(),
                    ..default()
                },
            ))
            .with_children(|root| {
                root.spawn(make_cam_entity(Transform::IDENTITY));
                root.spawn((
                    SceneBundle {
                        scene: asset_server.load("models/Spaceship/player.gltf#Scene0"),
                        transform: ship_transform.clone(),
                        ..default()
                    },
                    Player,
                    Velocity(Vec3::ZERO),
                ))
                .with_children(|ship| {
                    // crosshair1
                    ship.spawn(MaterialMeshBundle {
                        mesh: quad_mesh_5.clone(),
                        transform: Transform::from_xyz(0.0, 0.0, -60.0),
                        material: materials.add(ColorMaterial {
                            color: color_crosshair,
                            color_texture: Some(asset_server.load("textures/crosshair1.png")),
                            alpha_mode: AlphaMode::Blend,
                        }),
                        ..default()
                    });
                    // crosshair2
                    ship.spawn(MaterialMeshBundle {
                        mesh: quad_mesh_5.clone(),
                        transform: Transform::from_xyz(0.0, 0.0, -250.0),
                        material: materials.add(ColorMaterial {
                            color: color_crosshair,
                            color_texture: Some(asset_server.load("textures/crosshair2.png")),
                            alpha_mode: AlphaMode::Blend,
                        }),
                        ..default()
                    });
                });
            });
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    for (mut player_transform, mut player_velocity) in query.iter_mut() {
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

        // // clamp to bounds
        // player_transform.translation.x = player_transform
        //     .translation
        //     .x
        //     .clamp(-BOUNDS_POS.x, BOUNDS_POS.x);
        // player_transform.translation.y = player_transform
        //     .translation
        //     .y
        //     .clamp(-BOUNDS_POS.y, BOUNDS_POS.y);

        if player_transform.translation.z < -BOUNDS_POS.z + 10.0 {
            player_transform.translation.z = -15.0;
        }

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
}

// move the player along the path, this is stored in a player path component
fn move_along_path(
    mut query: Query<(&mut Transform, &mut PathFollower), With<PlayerRoot>>,
    mut path_query: Query<&PlayerPath>,
    time: Res<Time>,
) {
    for (mut root_transform, mut path_follower) in query.iter_mut() {
        for path in path_query.iter_mut() {
            path_follower.distance_along_path = (path_follower.distance_along_path
                + 20.0 * time.delta_seconds())
                % path.path_length;
            let next_transform = path.lerp_next_transform(path_follower.distance_along_path);
            root_transform.translation = next_transform.translation;
            root_transform.rotation = next_transform.rotation;
        }
    }
}

fn fire_bullet(
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<&GlobalTransform, With<Player>>,
    mut ev_fire: EventWriter<SpawnBulletEvent>,
) {
    // if spacebar is pressed
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    for player_global_tranform in query.iter() {
        let bullet_transform = Transform {
            translation: player_global_tranform.translation()
                + player_global_tranform.forward() * 2.0,
            rotation: player_global_tranform.compute_transform().rotation,
            ..default()
        };

        ev_fire.send(SpawnBulletEvent {
            transform: bullet_transform,
            direction: player_global_tranform.forward(),
            speed: BULLET_SPEED,
        });
    }
}
