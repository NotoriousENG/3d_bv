mod bullet;
mod constants;
mod enemy;
mod events;
mod game;
mod level;
mod materials;
mod math;
mod particles;
mod player;
mod skybox;
mod velocity;

use bevy::prelude::*;
use bevy::window::PresentMode;

use crate::game::GamePlugin;

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
        .insert_resource(Msaa { samples: 4 })
        .add_plugin(GamePlugin)
        .run();
}
