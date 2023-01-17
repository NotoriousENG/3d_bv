use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, CollisionEvent, Sensor};

use crate::{
    events::{ExplosionEvent, SpawnBulletEvent},
    velocity::Velocity,
};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_collision_events)
            .add_system(create_bullet);
    }
}

#[derive(Component)]
pub struct Bullet;

fn create_bullet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ev_bullet: EventReader<SpawnBulletEvent>,
) {
    for ev in ev_bullet.iter() {
        commands.spawn((
            SceneBundle {
                scene: asset_server.load("models/Spaceship/bullet.gltf#Scene0"),
                transform: ev.transform.clone(),
                ..default()
            },
            Velocity(ev.direction * ev.speed),
            Bullet,
            Collider::cuboid(0.494, 0.494, 2.144),
            Sensor,
        ));
    }
}

fn handle_collision_events(
    query_bullet: Query<(Entity, &Transform), With<Bullet>>,
    mut contact_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    mut ev_explosion: EventWriter<ExplosionEvent>,
) {
    for contact_event in contact_events.iter() {
        for (bullet_entity, bullet_transform) in query_bullet.iter() {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                if h1 == &bullet_entity || h2 == &bullet_entity {
                    ev_explosion.send(ExplosionEvent(bullet_transform.clone()));

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
