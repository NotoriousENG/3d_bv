use bevy::{prelude::*, core_pipeline::bloom::BloomSettings};
use bevy_atmosphere::prelude::*;
use bevy_spectator::*;

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;
const PLAYER_SPEED: f32 = 10.0;

#[derive(Component)]
struct Player;

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
    ));
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = query.single_mut();
    let mut direction = Vec3::ZERO;

    // store xy direction in a vector based off WASD keys
    let x = keyboard_input.pressed(KeyCode::D) as i32 as f32
        - keyboard_input.pressed(KeyCode::A) as i32 as f32;
    let y = keyboard_input.pressed(KeyCode::W) as i32 as f32
        - keyboard_input.pressed(KeyCode::S) as i32 as f32;
    direction += Vec3::new(x, y, 0.0).normalize_or_zero();

    // Calculate the new horizontal paddle position based on player input
    let next_player_position = player_transform.translation + direction * PLAYER_SPEED * TIME_STEP;

    player_transform.translation = next_player_position;
}
