use bevy::prelude::*;
// use bevy_editor_pls::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::bullet::BulletPlugin;
use crate::constants::make_cam_entity;
use crate::enemy::EnemyPlugin;
use crate::events::EventPlugin;
use crate::level::LevelPlugin;
use crate::particles::ParticlePlugin;
use crate::player::PlayerPlugin;
use crate::velocity::VelocityPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_plugin(EventPlugin)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(VelocityPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(BulletPlugin)
            .add_plugin(ParticlePlugin)
            //.add_plugin(RapierDebugRenderPlugin::default()) // disable hdr to use
            .add_plugin(LevelPlugin)
            // .add_plugin(EditorPlugin)
            .run();
    }
}

fn setup(mut commands: Commands) {
    const HALF_SIZE: f32 = 1.0;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
    commands.spawn(make_cam_entity(Transform::IDENTITY));
}
