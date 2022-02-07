use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

const MOUSE_SENSITIVITY: f32 = 0.05;

#[derive(Component)]
struct Player;

type Rotation = (f32, f32);

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(keyboard_controls)
        .add_system(mouse_controls)
        .run();
}

fn mouse_controls(mut rot: Local<Rotation>, mut events: EventReader<MouseMotion>, mut player: Query<&mut Transform, With<Player>>) {
    let mut pitch: f32 = 0.0;
    let mut yaw: f32 = 0.0;

    for mouse in events.iter() {
        pitch += mouse.delta.y * MOUSE_SENSITIVITY;
        yaw += -mouse.delta.x * MOUSE_SENSITIVITY;
    }

    let new_pitch = rot.0 + (pitch * PI/180.0);
    if !(new_pitch > PI/2.0 || new_pitch < -PI/2.0) {
        rot.0 = new_pitch;
    }
    rot.1 += yaw * PI/180.0;


    let mut trans = player.single_mut();
    trans.rotation = Quat::from_axis_angle(Vec3::Y, rot.1) * Quat::from_axis_angle(-Vec3::X, rot.0);
}

fn keyboard_controls(kb: Res<Input<KeyCode>>, mut player: Query<&mut Transform, With<Player>>) {
    let mut trans = player.single_mut();
    let local_fwd = trans.forward();
    let local_right = trans.right();

    if kb.pressed(KeyCode::Space) {
        trans.translation.y += 0.125;
    }
    if kb.pressed(KeyCode::LShift) {
        trans.translation.y -= 0.125;
    }
    if kb.pressed(KeyCode::W) {
        trans.translation += local_fwd * 0.125;
    }
    if kb.pressed(KeyCode::S) {
        trans.translation -= local_fwd * 0.125;
    }
    if kb.pressed(KeyCode::D) {
        trans.translation += local_right * 0.125;
    }
    if kb.pressed(KeyCode::A) {
        trans.translation -= local_right * 0.125;
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
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
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }).insert(Player);
}