#[macro_use]
extern crate lazy_static;

mod systems;
mod components;

mod world;
mod util;

use bevy::prelude::*;
use crate::world::manager::ChunkManager;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(systems::keyboard_controls)
        .add_system(systems::mouse_controls)
        .add_system(systems::skylight)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let mut cm = ChunkManager::default();

    for x in -10..10i32 {
        for z in -10..10i32 {
            for y in -3..3i32 {
                commands.spawn_bundle(PbrBundle {
                    mesh: meshes.add(cm.generate_new(IVec3::new(x, y, z)).create_mesh().into()),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgb(0.6, 0.6, 0.6),
                        metallic: 0.0,
                        perceptual_roughness: 0.6,
                        reflectance: 0.001,
                        .. Default::default()
                    }),
                    transform: Transform::from_translation(Vec3::new(x as f32 * 32.0, y as f32 * 32.0, z as f32 * 32.0)),
                    ..Default::default()
                });
            }

        }
    }

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