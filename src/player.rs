use bevy::prelude::*;

use crate::constants::BOUNDS_POS;
use crate::events::SpawnBulletEvent;
use crate::materials::ColorMaterial;
use crate::math::{deg_to_rad, move_toward, move_toward_f32};
use crate::velocity::Velocity;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(move_player)
            .add_system(fire_bullet)
            .add_system(move_camera)
            .add_plugin(MaterialPlugin::<ColorMaterial>::default());
    }
}

const MAX_SPEED: f32 = 30.0;
const ROT_SPEED: f32 = 3.0;
const ACCELERATION: f32 = 0.75;
const BULLET_SPEED: f32 = 300.0;

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
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
                material: materials.add(ColorMaterial {
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
                material: materials.add(ColorMaterial {
                    color: color_crosshair,
                    color_texture: Some(asset_server.load("textures/crosshair2.png")),
                    alpha_mode: AlphaMode::Blend,
                }),
                ..default()
            });
        });
}

fn move_camera(
    query: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let player_transform = query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.z = player_transform.translation.z + 15.0;
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

    player_velocity.0.z = -10.0;

    // clamp to bounds
    player_transform.translation.x = player_transform
        .translation
        .x
        .clamp(-BOUNDS_POS.x, BOUNDS_POS.x);
    player_transform.translation.y = player_transform
        .translation
        .y
        .clamp(-BOUNDS_POS.y, BOUNDS_POS.y);

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

fn fire_bullet(
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<&Transform, With<Player>>,
    mut ev_fire: EventWriter<SpawnBulletEvent>,
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

    ev_fire.send(SpawnBulletEvent {
        transform: bullet_transform,
        direction: player_transform.forward(),
        speed: BULLET_SPEED,
    });
}
