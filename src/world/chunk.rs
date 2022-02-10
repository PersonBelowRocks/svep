#![allow(unused)]
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use crate::util::{Volume, VolumeIdx, FaceVectors, FaceMesh};
use super::voxel::Voxel;

pub(crate) const CHUNK_SIZE: usize = 32;
pub(crate) type ChunkPosition = IVec3;

pub(crate) struct Chunk {
    position: ChunkPosition,
    empty: bool,
    volume: Volume<Voxel, CHUNK_SIZE>,
}

pub(crate) struct ChunkMesh {
    position: ChunkPosition,
    vertices: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    indices: Vec<u32>
}

impl ChunkMesh {
    fn empty(position: ChunkPosition) -> Self {
        Self {
            position,
            vertices: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub(crate) fn position(&self) -> ChunkPosition {
        self.position
    }
}

#[allow(clippy::from_over_into)]
impl Into<Mesh> for ChunkMesh {
    fn into(self) -> Mesh {
        let mut out = Mesh::new(PrimitiveTopology::TriangleList);

        out.set_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices);
        out.set_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        out.set_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs);
        out.set_indices(Some(Indices::U32(self.indices)));

        out
    }
}

impl Chunk {
    pub(crate) fn random_chunk(position: ChunkPosition) -> Self {
        //let mut volume = Volume::filled(Voxel::inactive());
        // let mut empty = true;
        //
        // for idx in volume.iter_indices() {
        //     if rand::random::<f32>() > 0.075 {
        //         volume[idx].active = true;
        //         empty = false;
        //     }
        // }

        Self {
            position,
            empty: false,
            volume: Volume::filled(Voxel::active())
        }
    }

    pub(crate) fn new(position: ChunkPosition, data: Volume<Voxel, CHUNK_SIZE>) -> Self {
        let mut empty = true;
        if data.iter().any(|(_, v)| v.active) {
            empty = false;
        }

        Self {
            position,
            volume: data,
            empty,
        }
    }

    pub(crate) fn position(&self) -> ChunkPosition {
        self.position
    }

    pub(crate) fn create_mesh(&self) -> ChunkMesh {
        let mut mesh = ChunkMesh::empty(self.position());
        let mut current_index = 0u32;

        for (idx, voxel) in self.volume.iter() {
            if !voxel.active { continue; }
            let this_pos = volume_idx_to_vec(idx);

            for (direction, neighbor_idx) in NeighborIterator::from_idx(idx) {
                let neighbor_voxel = self.volume[neighbor_idx];
                if !neighbor_voxel.active {
                    let neighbor_pos = volume_idx_to_vec(neighbor_idx);

                    // todo: we're currently adding vertex positions in reverse and negating normals;
                    //  fix the underlying model instead of doing all this extra work
                    let mut buf: Vec<[f32; 3]> = Vec::new();

                    for vertex in direction.get_face_mesh() {
                        // Vertex position, added in reverse cause model is weird
                        buf.push((Vec3::from(vertex.0) + this_pos).into());

                        // Vertex normal, negated for same reason as above
                        mesh.normals.push((-Vec3::from(vertex.1)).into());

                        // Vertex uv coords, probably also broken in some way
                        mesh.uvs.push(vertex.2);

                    }

                    buf.reverse();
                    mesh.vertices.append(&mut buf);

                    // We could just add the vertices with duplicates and interpret the entire buffer as a TriangleList
                    // which would be simpler but take up more memory, so we do this instead.
                    for index_offset in [0, 1, 2, 2, 3, 0u32] {
                        mesh.indices.push(index_offset + current_index);
                    }
                    // Each voxel face requires 4 vertices (2 triangles)
                    current_index += 4;
                }
            }
        }

        mesh
    }
}

impl From<Chunk> for Volume<Voxel, CHUNK_SIZE> {
    fn from(chunk: Chunk) -> Self {
        chunk.volume
    }
}

#[derive(Copy, Clone)]
enum Direction {
    UP,
    DOWN,
    NORTH,
    EAST,
    SOUTH,
    WEST
}

impl Direction {
    fn get_face_mesh(&self) -> FaceMesh {
        use crate::util::{
            PY_FACE,
            NY_FACE,
            NZ_FACE,
            PX_FACE,
            PZ_FACE,
            NX_FACE
        };

        match self {
            Self::UP => *PY_FACE,
            Self::DOWN => *NY_FACE,
            Self::NORTH => *NZ_FACE,
            Self::EAST => *PX_FACE,
            Self::SOUTH => *PZ_FACE,
            Self::WEST => *NX_FACE
        }
    }
}

struct NeighborIterator {
    neighbor_indices: [Option<(Direction, VolumeIdx)>; 6],
    current_idx: usize
}

impl NeighborIterator {
    fn from_idx(idx: VolumeIdx) -> Self {
        let mut idxs = [None; 6];

        if idx.0 + 1 < CHUNK_SIZE { idxs[0] = Some((Direction::EAST, (idx.0 + 1, idx.1, idx.2))) }
        if idx.0.checked_sub(1).is_some() { idxs[1] = Some((Direction::WEST, (idx.0 - 1, idx.1, idx.2))) }

        if idx.1 + 1 < CHUNK_SIZE { idxs[2] = Some((Direction::UP, (idx.0, idx.1 + 1, idx.2))) }
        if idx.1.checked_sub(1).is_some() { idxs[3] = Some((Direction::DOWN, (idx.0, idx.1 - 1, idx.2))) }

        if idx.2 + 1 < CHUNK_SIZE { idxs[4] = Some((Direction::SOUTH, (idx.0, idx.1, idx.2 + 1))) }
        if idx.2.checked_sub(1).is_some() { idxs[5] = Some((Direction::NORTH, (idx.0, idx.1, idx.2 - 1))) }

        Self {
            neighbor_indices: idxs,
            current_idx: 0
        }
    }
}

impl Iterator for NeighborIterator {
    type Item = (Direction, VolumeIdx);
    fn next(&mut self) -> Option<Self::Item> {
        while self.current_idx < 6 {
            let last_idx = self.current_idx;
            self.current_idx += 1;
            if let Some(idx) = self.neighbor_indices[last_idx] {
                return Some(idx);
            }
        }

        None
    }
}

fn volume_idx_to_vec(idx: VolumeIdx) -> Vec3 {
    Vec3::new(idx.0 as f32, idx.1 as f32, idx.2 as f32)
}
