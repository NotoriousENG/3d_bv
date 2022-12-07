use bevy::{prelude::*, core_pipeline::bloom::BloomSettings};
use bevy_atmosphere::prelude::*;
use bevy_spectator::*;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;
const MAX_SPEED: f32 = 20.0;
const ACCELERATION: f32 = 0.75;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec3);

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(AtmosphereModel::default()) // Default Atmosphere material, we can edit it to simulate another planet
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(AtmospherePlugin)
        // .add_plugin(SpectatorPlugin)
        .add_startup_system(setup)
        .add_system(move_player)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                hdr: true,
                ..default()
             },
            transform: Transform::from_xyz(0.7, 0.7, 15.0)
                .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
            ..default()
        },
        BloomSettings {
            intensity: 0.05,
            ..default()
        },
        AtmosphereCamera::default(),
        Spectator,
    ));
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/Spaceship/player.gltf#Scene0"),
            ..default()
        },
        Player,
        Velocity(Vec3::ZERO),
    ));
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
    player_velocity.0 = move_toward(player_velocity.0, input_movement_vector * MAX_SPEED, ACCELERATION);

    // move player based on velocity
    player_transform.translation += player_velocity.0 * TIME_STEP;

    // set rotation degrees in euler angles for x and y to velocity / 2
    player_transform.rotation = Quat::from_euler(EulerRot::XYZ, deg_to_rad(player_velocity.0.y / 2.0), deg_to_rad(player_velocity.0.x / 2.0), deg_to_rad(-player_velocity.0.x / 2.0));
}

// Moves from toward to by the delta value and returns a new vector
fn move_toward(from: Vec3, to: Vec3, delta: f32) -> Vec3  {
    let mut result = to - from;
    let length = result.length();
    if length <= delta || length == 0.0 {
        return to;
    }
    result *= delta / length;
    result += from;
    result
}

// convert from degrees to radians
fn deg_to_rad(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}