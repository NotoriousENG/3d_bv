use bevy::prelude::*;

pub struct ExplosionEvent(pub Transform);

pub struct SpawnBulletEvent {
    pub transform: Transform,
    pub direction: Vec3,
    pub speed: f32,
}

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ExplosionEvent>()
            .add_event::<SpawnBulletEvent>();
    }
}
