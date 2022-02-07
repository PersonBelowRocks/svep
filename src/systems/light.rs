use bevy::prelude::*;

pub(crate) fn skylight(mut query: Query<&mut Transform, With<DirectionalLight>>) {
    for mut light in query.iter_mut() {
        light.rotate(Quat::from_axis_angle(Vec3::Z, 0.0001));
    }
}