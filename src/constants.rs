use crate::math::deg_to_rad;
use bevy::prelude::Vec3;
use bevy::{core_pipeline::bloom::BloomSettings, prelude::*, render::camera::Projection};

pub const BOUNDS_POS: Vec3 = Vec3::new(15.0, 8.0, 300.0);

pub fn make_cam_entity(cam_transform: Transform) -> (bevy::prelude::Camera3dBundle, BloomSettings) {
    return (
        Camera3dBundle {
            camera: Camera {
                hdr: true, // disable to use rapier debug render pipeline
                ..default()
            },
            projection: Projection::Perspective(PerspectiveProjection {
                fov: deg_to_rad(70.0),
                near: 0.05,
                far: 300.0,
                ..default()
            }),
            transform: cam_transform,
            ..default()
        },
        BloomSettings {
            intensity: 0.05,
            ..default()
        },
    );
}
