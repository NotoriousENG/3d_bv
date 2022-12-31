use bevy::prelude::*;
use bevy_hanabi::prelude::*;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HanabiPlugin)
            .add_startup_system(setup_fireworks)
            .add_startup_system(setup_shooting_stars);
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
            spawner: Spawner::once(500.0.into(), false),
            ..Default::default()
        }
        .init(PositionSphereModifier {
            dimension: ShapeDimension::Volume,
            radius: 2.,
            speed: 70_f32.into(),
            center: Vec3::ZERO,
        })
        .init(ParticleLifetimeModifier { lifetime: 1. })
        .update(LinearDragModifier { drag: 5. })
        .update(AccelModifier {
            accel: Vec3::new(0., -8., 0.),
        })
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient1,
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient1,
        }),
    );

    commands.spawn((ParticleEffectBundle::new(firework_fx), Firework));
}

#[derive(Component)]
pub struct ShootingStars;

fn setup_shooting_stars(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let mut color_gradient1 = Gradient::new();
    color_gradient1.add_key(0.0, Vec4::new(0.0, 0.0, 0.0, 1.0));
    color_gradient1.add_key(1.0, Vec4::new(4.0, 4.0, 4.0, 1.0));

    let mut size_gradient1 = Gradient::new();
    size_gradient1.add_key(0.0, Vec2::splat(0.0));
    size_gradient1.add_key(1.0, Vec2::splat(1.0));

    let shooting_stars_fx = effects.add(
        EffectAsset {
            name: "shooting_stars".to_string(),
            capacity: 32768,
            spawner: Spawner::rate(500.0.into()),
            ..Default::default()
        }
        .init(PositionSphereModifier {
            dimension: ShapeDimension::Surface,
            speed: 70_f32.into(),
            radius: 15.,
            center: Vec3::new(0., 0., -300.)
        })
        .init(ParticleLifetimeModifier { lifetime: 10. })
        .render(BillboardModifier {})
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient1,
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient1,
        }),
    );

    commands.spawn((ParticleEffectBundle::new(shooting_stars_fx), ShootingStars));
}