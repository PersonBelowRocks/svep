use bevy::prelude::*;

use noise::{NoiseFn, Perlin, Worley, Fbm, SuperSimplex};
use crate::util::Volume;

use crate::world::chunk::{Chunk, ChunkPosition, CHUNK_SIZE};
use crate::world::voxel::Voxel;

const PERLIN_THRESHOLD: f64 = 0.33;
const CHUNK_SIZE_F64: f64 = CHUNK_SIZE as f64;

pub(crate) struct ChunkManager {
    chunks: Vec<Chunk>,
    visible_chunks: Vec<ChunkPosition>,
    noisegen: Worley
}

impl Default for ChunkManager {
    fn default() -> Self {
        Self {
            chunks: Vec::new(),
            visible_chunks: Vec::new(),
            noisegen: Worley::new(),
        }
    }
}

impl ChunkManager {
    pub(crate) fn generate_new(&mut self, pos: ChunkPosition) -> Chunk {
        let mut vol: Volume<_, 32> = Volume::filled(Voxel::inactive());

        for idx in vol.iter_indices() {
            let x = (idx.0 as f64 / CHUNK_SIZE_F64) + (pos.x as f64);
            let y = (idx.1 as f64 / CHUNK_SIZE_F64) + (pos.y as f64);
            let z = (idx.2 as f64 / CHUNK_SIZE_F64) + (pos.z as f64);

            let noise = self.noisegen.get([x/3.0, y/3.0, z/3.0]);
            if noise > PERLIN_THRESHOLD {
                vol[idx].active = true;
            }
        }

        // self.chunks.push(Chunk::new(pos, vol));
        Chunk::new(pos, vol)
    }
}