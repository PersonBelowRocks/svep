#![allow(unused)]
use crate::util::Volume;
use crate::render::vertex::Vertex;
use super::voxel::Voxel;
use cgmath::Vector3;

pub(crate) const CHUNK_SIZE: usize = 32;
pub(crate) type ChunkPosition = (u32, u32, u32);

pub(crate) struct Chunk {
    position: ChunkPosition,
    empty: bool,
    volume: Volume<Voxel, CHUNK_SIZE>,
}

pub(crate) struct ChunkMesh {
    vertices: Vec<Vertex>,
}

enum FaceGeometry {
    UP = [
        Vector3::new(-0.5, 0.5, 0.5),
        Vector3::new(0.5, 0.5, 0.5),
        Vector3::new(-0.5, 0.5, -0.5),

    ], // +Y
    DOWN = todo!(), // -Y
    NORTH = todo!(), // -Z
    EAST = todo!(), // +X
    SOUTH = todo!(), // +Z
    WEST = todo!(), // -X
}

impl Chunk {
    pub(crate) fn random_chunk(position: ChunkPosition) -> Self {
        let mut volume = Volume::filled(Voxel::inactive());
        let mut empty = true;

        for idx in volume.iter_indices() {
            if rand::random::<f32>() > 0.33 {
                volume[idx].active = true;
                empty = false;
            }
        }

        Self {
            position,
            empty,
            volume
        }
    }

    pub(crate) fn create_mesh(&self) -> ChunkMesh {
        let mut vertices: Vec<Vertex> = Vec::new();

        for (idx, voxel) in self.volume.iter() {
            if !voxel.active { continue; }


        }
    }
}

