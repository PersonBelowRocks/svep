use bevy::prelude::*;
use std::f32::consts::PI;
use bevy::input::mouse::MouseMotion;
use crate::components::Player;

const MOUSE_SENSITIVITY: f32 = 0.05;

type Rotation = (f32, f32);

pub(crate) fn mouse_controls(mut rot: Local<Rotation>, mut events: EventReader<MouseMotion>, mut player: Query<&mut Transform, With<Player>>) {
    let mut pitch: f32 = 0.0;
    let mut yaw: f32 = 0.0;

    for mouse in events.iter() {
        pitch += mouse.delta.y * MOUSE_SENSITIVITY;
        yaw += -mouse.delta.x * MOUSE_SENSITIVITY;
    }

    let new_pitch = rot.0 + (pitch * PI/180.0);
    if (-PI/2.0..=PI/2.0).contains(&new_pitch) {
        rot.0 = new_pitch;
    }
    rot.1 += yaw * PI/180.0;


    let mut trans = player.single_mut();
    trans.rotation = Quat::from_axis_angle(Vec3::Y, rot.1) * Quat::from_axis_angle(-Vec3::X, rot.0);
}

pub(crate) fn keyboard_controls(kb: Res<Input<KeyCode>>, mut player: Query<&mut Transform, With<Player>>) {
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