#[macro_use]
extern crate lazy_static;

mod systems;
mod components;

mod world;
mod util;

use bevy::prelude::*;
use crate::shape::Cube;
use crate::util::PlayerMoveEvent;
use crate::world::chunk::ChunkPosition;
use crate::world::manager::ChunkManager;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_event::<PlayerMoveEvent>()
        .add_startup_system(setup)
        .add_system(systems::keyboard_controls)
        .add_system(systems::mouse_controls)
        .add_system(systems::skylight)
        .add_system(systems::log_player_chunk_boundary_crossing)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let mut cm = ChunkManager::default();
    let mut positions: Vec<ChunkPosition> = Vec::new();

    for x in -10..=10i32 {
        for z in -10..=10i32 {
            for y in -10..=10i32 {
                positions.push(ChunkPosition::new(x, y, z));
            }
        }
    }

    let chunk_meshes = cm.gen_mesh_multi(positions);

    for mesh in chunk_meshes {
        let pos = mesh.position();
        let trans = Vec3::new(pos.x as f32 * 32.0, pos.y as f32 * 32.0, pos.z as f32 * 32.0);

        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(mesh.into()),
            material: materials.add(StandardMaterial {
                // base_color: Color::rgb(0.6, 0.6, 0.6),
                metallic: 0.0,
                perceptual_roughness: 0.6,
                reflectance: 0.001,
                .. Default::default()
            }),
            transform: Transform::from_translation(trans),
            ..Default::default()
        });
    }

    use crate::shape::Cube as ABC;

    // light
    let size = 100.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 80_000.0,
            shadows_enabled: true,
            shadow_projection: OrthographicProjection {
                left: -size,
                right: size,
                bottom: -size,
                top: size,
                near: -size,
                far: size,
                ..Default::default()
            },
            ..Default::default()
        },
        transform: Transform::from_rotation(Quat::from_axis_angle(Vec3::X, -2.0)),
        ..Default::default()
    });

    // camera + player
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0., 1.0, 0.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }).insert(components::Player::default());
}