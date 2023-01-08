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
    time: Res<Time>,
) {
    for (mut transform, velocity, entity) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();

        // delete if too far away
        if transform.translation.z > BOUNDS_POS.z || transform.translation.z < -BOUNDS_POS.z {
            if let Some(entity_commands) = commands.get_entity(entity) {
                entity_commands.despawn_recursive();
            }
        }
    }
}
