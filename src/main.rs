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
use bevy::window::{PresentMode, WindowResolution};

use crate::game::GamePlugin;

#[bevy_main]
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Window {
                    title: "Fake Star Fox".to_string(),
                    resolution: WindowResolution::new(800.0, 600.0),
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                }
                .into(),
                ..default()
            }),
        )
        .insert_resource(Msaa::Sample4)
        .add_plugin(GamePlugin)
        .run();
}
