#[macro_use]
extern crate lazy_static;

mod systems;
mod components;

mod world;
mod util;

use bevy::prelude::*;

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

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(world::chunk::Chunk::random_chunk((1, 0, 0)).create_mesh().into()),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.6, 0.6, 0.6),
            metallic: 0.0,
            perceptual_roughness: 0.6,
            reflectance: 0.001,
            .. Default::default()
        }),
        transform: Transform::from_translation(Vec3::new(1.0*32.0, 0.0, 0.0)),
        ..Default::default()
    });

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 500.0 })),
        //material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(15.0/255.0, 115.0/255.0, 46.0/255.0),
            metallic: 0.0,
            perceptual_roughness: 0.6,
            reflectance: 0.001,
            .. Default::default()
        }),
        ..Default::default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });

    // light
    let size = 7.5;
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
    // camera

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }).insert(components::Player);
}