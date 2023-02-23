use crate::constants::BOUNDS_POS;
use bevy::prelude::*;

pub struct VelocityPlugin;

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(velocity_movement);
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec3);

/// apply velocity to transform
fn velocity_movement(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &Velocity, Entity)>,
    cam_query: Query<&GlobalTransform, With<Camera>>,
    time: Res<Time>,
) {
    for (mut transform, velocity, entity) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();

        for cam_transform in cam_query.iter() {
            // delete if too far away
            let dist = Vec3::distance(transform.translation, cam_transform.translation());
            if dist.abs() >= BOUNDS_POS.z {
                if let Some(entity_commands) = commands.get_entity(entity) {
                    entity_commands.despawn_recursive();
                }
            }
            break;
        }
    }
}
