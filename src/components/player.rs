use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct Player {
    position: Vec3,
}

impl Default for Player {
    fn default() -> Self {
        Self { position: Vec3::new(0.0, 0.0, 0.0) }
    }
}