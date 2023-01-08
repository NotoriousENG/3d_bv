use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, CollisionEvent, Sensor};

use bevy_hanabi::ParticleEffect;

use crate::particles::Firework;
use crate::velocity::Velocity;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_bullet_events);
    }
}

#[derive(Component)]
pub struct Bullet;

pub fn create_bullet(
    mut commands: Commands,
    bullet_transform: Transform,
    direction: Vec3,
    speed: f32,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/Spaceship/bullet.gltf#Scene0"),
            transform: bullet_transform,
            ..default()
        },
        Velocity(direction * speed),
        Bullet,
        Collider::cuboid(0.494, 0.494, 2.144),
        Sensor,
    ));
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
