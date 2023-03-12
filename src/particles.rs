use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use crate::events::ExplosionEvent;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HanabiPlugin)
            .add_startup_system(setup_fireworks)
            .add_system(handle_explosion_events);
    }
}

#[derive(Component)]
pub struct Firework;

fn setup_fireworks(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let mut color_gradient1 = Gradient::new();
    color_gradient1.add_key(0.0, Vec4::new(4.0, 4.0, 4.0, 1.0));
    color_gradient1.add_key(0.1, Vec4::new(4.0, 4.0, 0.0, 1.0));
    color_gradient1.add_key(0.9, Vec4::new(4.0, 0.0, 0.0, 1.0));
    color_gradient1.add_key(1.0, Vec4::new(4.0, 0.0, 0.0, 0.0));

    let mut size_gradient1 = Gradient::new();
    size_gradient1.add_key(0.0, Vec2::splat(0.1));
    size_gradient1.add_key(0.3, Vec2::splat(0.1));
    size_gradient1.add_key(1.0, Vec2::splat(0.0));

    let firework_fx = effects.add(
        EffectAsset {
            name: "firework".to_string(),
            capacity: 32768,
            spawner: Spawner::once(2500.0.into(), false),
            ..Default::default()
        }
        .init(InitPositionSphereModifier {
            center: Vec3::ZERO,
            radius: 2.,
            dimension: ShapeDimension::Volume,
        })
        .init(InitVelocitySphereModifier {
            center: Vec3::ZERO,
            // Give a bit of variation by randomizing the initial speed
            speed: Value::Uniform((65., 75.)),
        })
        .init(InitLifetimeModifier {
            // Give a bit of variation by randomizing the lifetime per particle
            lifetime: Value::Uniform((0.8, 1.2)),
        })
        .init(InitAgeModifier {
            // Give a bit of variation by randomizing the age per particle. This will control the
            // starting color and starting size of particles.
            age: Value::Uniform((0.0, 0.2)),
        })
        .update(LinearDragModifier { drag: 5. })
        .update(AccelModifier::constant(Vec3::new(0., -8., 0.)))
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient1,
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient1,
        }),
    );

    commands.spawn((ParticleEffectBundle::new(firework_fx), Firework));
}

fn handle_explosion_events(
    mut query_effect: Query<(&mut ParticleEffect, &mut Transform), With<Firework>>,
    mut ev_explosion: EventReader<ExplosionEvent>,
) {
    for (mut firework_fx, mut firework_transform) in query_effect.iter_mut() {
        for ev in ev_explosion.iter() {
            firework_transform.translation = ev.0.translation;
            firework_fx.maybe_spawner().unwrap().reset();
        }
    }
}
