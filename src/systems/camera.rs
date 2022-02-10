use bevy::prelude::*;
use std::f32::consts::PI;
use bevy::input::mouse::MouseMotion;
use crate::components::Player;
use crate::util::PlayerMoveEvent;

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

const SPEED: f32 = 0.8;

pub(crate) fn keyboard_controls(mut writer: EventWriter<PlayerMoveEvent>, kb: Res<Input<KeyCode>>, mut query: Query<(Entity, &mut Transform, &mut Player)>) {
    let (entity, mut trans, mut player) = query.single_mut();
    let local_fwd = trans.forward();
    let local_right = trans.right();

    let mut movement = Vec3::new(0., 0., 0.);

    if kb.pressed(KeyCode::Space) {
        movement.y += SPEED;
    }
    if kb.pressed(KeyCode::LShift) {
        movement.y -= SPEED;
    }
    if kb.pressed(KeyCode::W) {
        movement += local_fwd * SPEED;
    }
    if kb.pressed(KeyCode::S) {
        movement -= local_fwd * SPEED;
    }
    if kb.pressed(KeyCode::D) {
        movement += local_right * SPEED;
    }
    if kb.pressed(KeyCode::A) {
        movement -= local_right * SPEED;
    }

    let old_position = player.position;

    trans.translation += movement;
    player.position += movement;

    writer.send(PlayerMoveEvent::new(entity, old_position, player.position));
}