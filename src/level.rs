use bevy::{gltf::GltfExtras, prelude::*};

use bevy_asset_loader::prelude::*;

use crate::{
    events::{SpawnPlayerEvent, TeardownLevelEvent},
    skybox::{set_skybox_texture, Cubemap, SkyboxPlugin, SkyboxState},
};

pub struct LevelPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum LevelState {
    Loading,
    Loaded,
}

#[derive(AssetCollection, Resource)]
struct LevelAssets {
    #[asset(path = "models/LV_test/lv_test.gltf#Scene0")]
    lv_test: Handle<Scene>,
    #[asset(path = "models/LV1/lv_1.gltf#Scene0")]
    lv_1: Handle<Scene>,
}

#[derive(Component)]
struct Level;

#[derive(Component)]
struct PlayerPathRaw {
    parent_transform: Transform,
}

#[derive(Component)]
pub struct PlayerPath {
    pub points: Vec<PathTransformDescriptor>,
    pub path_length: f32,
}

impl PlayerPath {
    pub fn lerp_next_transform(&self, distance_along_path: f32) -> Transform {
        // iterate through the points, get the previous and next point
        // lerp between them based on the distance along the path
        for i in 0..self.points.len() {
            let prev = self.points[i];
            let next = self.points[(i + 1) % self.points.len()];
            if distance_along_path >= prev.distance_along_path
                && (distance_along_path <= next.distance_along_path
                    || next.distance_along_path == 0.0)
            {
                let t = (distance_along_path - prev.distance_along_path)
                    / (next.distance_along_path - prev.distance_along_path);
                let transform = Transform::from_translation(
                    prev.transform
                        .translation
                        .lerp(next.transform.translation, t),
                )
                .with_rotation(prev.transform.rotation.lerp(next.transform.rotation, t));

                // println!("lerp: begin{:?}, end{:?}, val{:?}, t{}", prev.transform.rotation.to_euler(EulerRot::XYZ), next.transform.rotation.to_euler(EulerRot::XYZ), transform.rotation.to_euler(EulerRot::XYZ), t);

                return transform;
            }
        }

        return Transform::IDENTITY;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PathTransformDescriptor {
    pub transform: Transform,
    pub distance_along_path: f32,
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(LevelState::Loading)
            .add_loading_state(
                LoadingState::new(LevelState::Loading)
                    .continue_to_state(LevelState::Loaded)
                    .with_collection::<LevelAssets>(),
            )
            .add_system_set(SystemSet::on_enter(LevelState::Loaded).with_system(setup))
            .add_system_set(
                SystemSet::on_update(LevelState::Loaded).with_system(change_level_input),
            )
            .add_system(setup_level_data)
            .add_system(get_path_data)
            .add_plugin(SkyboxPlugin)
            .add_system_set(SystemSet::on_enter(SkyboxState::Loaded).with_system(load_skybox))
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 1.0,
            });
    }
}

fn setup(
    commands: Commands,
    level_assets: Res<LevelAssets>,
    ev_spawn: EventWriter<SpawnPlayerEvent>,
) {
    load_level(commands, level_assets, 0, ev_spawn); // load the first level
}

fn setup_level_data(
    mut commands: Commands,
    q: Query<(Entity, &Transform, &GltfExtras, Option<&Children>)>,
) {
    for (entity, transform, extra, children) in q.iter() {
        println!("{:?}", extra);
        if extra.value.contains("PATH") {
            if extra.value.contains("PATH::PLAYER") {
                println!("Player Path found");
                println!("Transform: {:?}", transform);
                if let Some(children) = children {
                    for child in children.iter() {
                        println!("Child: {:?}", child);
                        commands.entity(*child).insert(PlayerPathRaw {
                            parent_transform: transform.clone(),
                        }); // the nurbs path must be a child or else it will not work
                    }
                }
            }
        }
        commands.entity(entity).remove::<GltfExtras>();
    }
}

fn get_path_data(
    mut commands: Commands,
    q: Query<(Entity, &Handle<Mesh>, &PlayerPathRaw)>,
    meshes: Res<Assets<Mesh>>,
    mut ev_spawn: EventWriter<SpawnPlayerEvent>,
) {
    for (entity, mesh, raw_path) in q.iter() {
        let mut path_node_transforms: Vec<PathTransformDescriptor> = [].to_vec();
        let mut total_distance = 0.0;
        if let Some(mesh) = meshes.get(mesh) {
            println!("Mesh Raw Data: {:?}", mesh);
            // get vertex positions
            let raw_vertex_positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
            // convert to Vec3
            raw_vertex_positions
                .get_bytes()
                .chunks_exact(12)
                .for_each(|chunk| {
                    let x = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                    let y = f32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]);
                    let z = f32::from_le_bytes([chunk[8], chunk[9], chunk[10], chunk[11]]);
                    // apply transform to positions
                    let path_node_transform =
                        raw_path.parent_transform * Transform::from_xyz(x, y, z);
                    path_node_transforms.push(PathTransformDescriptor {
                        transform: path_node_transform,
                        distance_along_path: 0.0,
                    });
                });
            // rotate each path_node_transform so that it is facing the next path_node_transform,
            // the last path_node_transform will be facing the first path_node_transform
            // also compute the total distance of the path
            for i in 0..path_node_transforms.len() {
                let mut next_index = i + 1;
                if next_index >= path_node_transforms.len() {
                    next_index = 0;
                }
                let current_transform = path_node_transforms[i].clone();
                let next_transform = path_node_transforms[next_index].clone();
                let distance =
                    next_transform.transform.translation - current_transform.transform.translation;
                total_distance += distance.length();
                if next_index != 0 {
                    path_node_transforms[next_index].distance_along_path = total_distance.clone();
                }

                path_node_transforms[i]
                    .transform
                    .look_at(next_transform.transform.translation, Vec3::Y);
            }
            println!("Node Transforms: {:?}", path_node_transforms);
            println!("Path Length: {}", total_distance);
        }
        commands.entity(entity).insert(PlayerPath {
            points: path_node_transforms.clone(),
            path_length: total_distance,
        });
        commands.entity(entity).remove::<PlayerPathRaw>();

        let spawn: Transform = path_node_transforms
            .first()
            .ok_or(PathTransformDescriptor {
                transform: Transform::IDENTITY,
                distance_along_path: 0.0,
            })
            .unwrap()
            .clone()
            .transform;

        ev_spawn.send(SpawnPlayerEvent { transform: spawn });

        // remove the mesh component so that it doesn't render
        commands.entity(entity).remove::<Handle<Mesh>>();
    }
}

// change the skybox image
fn load_skybox(cubemap: ResMut<Cubemap>, asset_server: Res<AssetServer>) {
    set_skybox_texture(cubemap, asset_server.load("textures/sky.png"));
}

// change level based on index
// if index is 0, load lv_test
// if index is 1, load lv_1
fn load_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    index: usize,
    mut ev_spawn: EventWriter<SpawnPlayerEvent>,
) {
    let level: Handle<Scene>;
    match index {
        0 => {
            level = level_assets.lv_test.clone();
        }
        1 => {
            level = level_assets.lv_1.clone();
        }
        _ => {
            level = level_assets.lv_test.clone();
        }
    }

    commands.spawn((
        SceneBundle {
            scene: level.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Level,
    ));

    if level.clone().id() != level_assets.lv_test.id() {
        ev_spawn.send(SpawnPlayerEvent {
            transform: Transform::IDENTITY,
        });
    }
}

// call change level with keyboard input
// @TODO: Use an input map
fn change_level_input(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    keyboard_input: Res<Input<KeyCode>>,
    ev_spawn: EventWriter<SpawnPlayerEvent>,
    mut ev_teardown: EventWriter<TeardownLevelEvent>,
    q: Query<Entity, With<Level>>,
) {
    let selection: Option<usize>;
    if keyboard_input.just_pressed(KeyCode::Key0) {
        selection = Some(0);
    } else if keyboard_input.just_pressed(KeyCode::Key1) {
        selection = Some(1);
    } else {
        selection = None;
    }
    match selection {
        Some(scene_index) => {
            for entity in q.iter() {
                commands.entity(entity).despawn_recursive();
            }
            ev_teardown.send(TeardownLevelEvent {});
            load_level(commands, level_assets, scene_index, ev_spawn)
        }
        None => {}
    }
}
