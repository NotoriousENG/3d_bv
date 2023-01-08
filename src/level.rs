use bevy::prelude::*;

use crate::skybox::{set_skybox_texture, Cubemap, SkyboxPlugin, SkyboxState};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_plugin(SkyboxPlugin)
            .add_system_set(SystemSet::on_enter(SkyboxState::Loaded).with_system(load_skybox))
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 1.0,
            });
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((SceneBundle {
        scene: asset_server.load("models/LV1/lv_1.gltf#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    },));
}

// change the skybox image
fn load_skybox(cubemap: ResMut<Cubemap>, asset_server: Res<AssetServer>) {
    set_skybox_texture(cubemap, asset_server.load("textures/sky.png"));
}
