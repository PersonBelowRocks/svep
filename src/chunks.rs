use std::sync;
use glium::VertexFormat;

pub type ChunkCoords = (f32, f32, f32);

// todo: Voxels should store which face of itself is covered up by another voxel. These faces can be
//  culled in the vertex shader to reduce fragment shader invocations.
#[derive(Copy, Clone, Debug)]
pub struct Voxel {
    pub pos: (f32, f32, f32),
    pub chunk: ChunkCoords,
}

implement_vertex!(Voxel, pos, chunk);

impl Voxel {
    pub fn new(pos: (f32, f32, f32), chunk: ChunkCoords) -> Self {
        Self {
            pos,
            chunk
        }
    }

    pub fn new_anon(pos: (f32, f32, f32)) -> Self {
        Self {
            pos,
            chunk: (0.0, 0.0, 0.0),
        }
    }
}

pub type ChunkId = (i32, i32, i32);

// Chunk is a 128x128x128 cube of voxels
// todo: chunks could be made a lot more efficient if voxels were stored in a 1D array / vertex buffer,
//  and a 3D array with indices into this buffer was used to mutate the chunk. This would avoid
//  having to reload the entire chunk into the renderer just to make 1 change visible.
//  chunks should provide a safe and ergonomic API for mutating in this way.
#[derive(Clone)]
pub struct Chunk {
    pub pos: ChunkCoords,
    pub voxels: Vec<Vec<Vec<Option<Voxel>>>>
}

impl Chunk {
    pub fn new(pos: ChunkCoords, mut voxels: Vec<Vec<Vec<Option<Voxel>>>>) -> Self {

        // Center each voxel onto this chunk
        for mut vv_v in voxels.iter_mut() {
            for mut v_v in vv_v.iter_mut() {
                for mut v in v_v.iter_mut() {
                    match v {
                        Some(vox) => vox.chunk = pos,
                        _ => continue
                    }
                }
            }
        }

        Self {
            pos,
            voxels
        }
    }

    #[inline(always)]
    pub fn id(&self) -> (i32, i32, i32) {
        (self.pos.0 as i32, self.pos.1 as i32, self.pos.2 as i32)
    }
}

pub enum MutationType {
    Remove((usize, usize, usize)),
    Set((usize, usize, usize))
}

pub type ChunkMutation = Vec<MutationType>;


pub enum ChunkRenderOp {
    Load(Chunk),
    Unload(ChunkId),
    Mutate(ChunkMutation)
}