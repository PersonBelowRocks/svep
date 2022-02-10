use bevy::prelude::*;
use crate::world::chunk::{CHUNK_SIZE, ChunkPosition};

const CHUNK_SIZE_F32: f32 = CHUNK_SIZE as f32;

pub(crate) struct PlayerMoveEvent {
    entity: Entity,
    from: Vec3,
    to: Vec3,
}

impl PlayerMoveEvent {
    pub(crate) const fn new(entity: Entity, from: Vec3, to: Vec3) -> Self {
        Self {
            entity,
            from,
            to
        }
    }

    #[inline]
    pub(crate) fn entity(&self) -> Entity {
        self.entity
    }

    #[inline]
    pub(crate) fn moved_from(&self) -> Vec3 {
        self.from
    }

    #[inline]
    pub(crate) fn moved_to(&self) -> Vec3 {
        self.to
    }

    /// Position of the chunk the player moved from.
    #[inline(always)]
    pub(crate) fn moved_from_chunk(&self) -> ChunkPosition {
        ChunkPosition::new(
            (self.from.x / CHUNK_SIZE_F32).floor() as i32,
            (self.from.y / CHUNK_SIZE_F32).floor() as i32,
            (self.from.z / CHUNK_SIZE_F32).floor() as i32,
        )
    }

    /// Position of the chunk the player moved to. If the player didn't cross a chunk boundary
    /// then this is the same as the chunk the player moved from.
    #[inline(always)]
    pub(crate) fn moved_to_chunk(&self) -> ChunkPosition {
        ChunkPosition::new(
            (self.to.x / CHUNK_SIZE_F32).floor() as i32,
            (self.to.y / CHUNK_SIZE_F32).floor() as i32,
            (self.to.z / CHUNK_SIZE_F32).floor() as i32,
        )
    }
}