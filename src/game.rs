use bevy::{core_pipeline::bloom::BloomSettings, prelude::*, render::camera::Projection};
use bevy_rapier3d::prelude::*;

use crate::bullet::BulletPlugin;
use crate::enemy::EnemyPlugin;
use crate::level::LevelPlugin;
use crate::math::deg_to_rad;
use crate::particles::ParticlePlugin;
use crate::player::PlayerPlugin;
use crate::velocity::VelocityPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(VelocityPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(BulletPlugin)
            .add_plugin(ParticlePlugin)
            //.add_plugin(RapierDebugRenderPlugin::default()) // disable hdr to use
            .add_plugin(LevelPlugin)
            .run();
    }
}

fn setup(mut commands: Commands) {
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
}
