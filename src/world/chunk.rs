#![allow(unused)]
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use crate::util::{Volume, VolumeIdx, FaceVectors, FaceMesh};
use super::voxel::Voxel;
use std::sync::Arc;
use bevy::utils::HashMap;
use crate::world::manager::neighbor_vecs;

pub(crate) const CHUNK_SIZE: usize = 32;
pub(crate) const CHUNK_SIZE_I32: i32 = CHUNK_SIZE as i32;
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
        let vertex_count = self.vertices.len();

        out.set_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices);
        out.set_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        out.set_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs);
        // out.set_attribute(Mesh::ATTRIBUTE_COLOR, (0..vertex_count).map(|_| [0.07, 0.82, 0.33, 1.0f32]).collect::<Vec<_>>());
        out.set_indices(Some(Indices::U32(self.indices)));

        out
    }
}

impl std::ops::Index<IVec3> for Chunk {
    type Output = Voxel;

    #[inline(always)]
    fn index(&self, index: IVec3) -> &Self::Output {
        &self.volume[(index.x as usize, index.y as usize, index.z as usize)]
    }
}

impl Chunk {
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

    pub(crate) fn create_mesh(&self, neighbor_chunks: [Option<Arc<Chunk>>; 6]) -> ChunkMesh {
        let chunk_bounds = (0..CHUNK_SIZE_I32);
        let mut mesh = ChunkMesh::empty(self.position());
        let mut neighbors = HashMap::from_iter(
            neighbor_chunks
                .into_iter()
                .filter(|c| c.is_some())
                .map(|c| (c.as_ref().unwrap().position(), c.unwrap()))
        );

        // We calculate offsets from this value that eventually go in the index buffer.
        let mut current_index = 0u32;
        for (idx, voxel) in self.volume.iter() {
            if !voxel.active { continue; }

            let voxel_pos = IVec3::new(idx.0 as i32, idx.1 as i32, idx.2 as i32);

            for nb_vox_pos in neighbor_vecs(voxel_pos) {
                let mut should_draw = false;

                if !chunk_bounds.contains(&nb_vox_pos.x)
                    || !chunk_bounds.contains(&nb_vox_pos.y)
                    || !chunk_bounds.contains(&nb_vox_pos.z) {

                    // Voxel is in another chunk
                    let other_chunk_pos = IVec3::new(
                        (nb_vox_pos.x as f64 / CHUNK_SIZE_I32 as f64).floor() as i32 + self.position().x,
                        (nb_vox_pos.y as f64 / CHUNK_SIZE_I32 as f64).floor() as i32 + self.position().y,
                        (nb_vox_pos.z as f64 / CHUNK_SIZE_I32 as f64).floor() as i32 + self.position().z,
                    );

                    // If this chunk position isn't in the hashmap it means that the chunk is not
                    // generated. This is different from an empty chunk. Read comment below.
                    if !(neighbors.contains_key(&other_chunk_pos)) {
                        // It may be preferred to have this set to false so you don't render sides of
                        // chunks that are facing non-generated terrain. But for testing it's useful
                        // to fix the amount of terrain generated and view it while being positioned
                        // in non-generated areas, so we'll leave this as true for now.
                        should_draw = true;
                    } else {

                        // That chunk exists so we need to check what block is there.
                        let nb_vox_pos_in_other_chunk = IVec3::new(
                            nb_vox_pos.x.rem_euclid(CHUNK_SIZE_I32),
                            nb_vox_pos.y.rem_euclid(CHUNK_SIZE_I32),
                            nb_vox_pos.z.rem_euclid(CHUNK_SIZE_I32),
                        );

                        should_draw = !(neighbors.get(&other_chunk_pos).unwrap()[nb_vox_pos_in_other_chunk].active);
                    }

                } else {
                    // Voxel is in the same chunk
                    should_draw = !(self[nb_vox_pos].active);
                }

                if should_draw {
                    let voxel_pos_f32 = Vec3::new(
                        voxel_pos.x as f32,
                        voxel_pos.y as f32,
                        voxel_pos.z as f32
                    );

                    // Determine what face mesh to use for this.
                    let direction = Direction::from_relative(voxel_pos, nb_vox_pos);
                    let face_mesh = direction.get_face_mesh();

                    let mut buf: Vec<[f32; 3]> = Vec::with_capacity(4);
                    for (vertex, normal, uv) in face_mesh {
                        //buf.push((Vec3::from(vertex) + voxel_pos_f32).into());
                        mesh.vertices.push((Vec3::from(vertex) + voxel_pos_f32).into());

                        mesh.normals.push((Vec3::from(normal)).into());

                        mesh.uvs.push(uv);
                    }

                    //buf.reverse();
                    //mesh.vertices.append(&mut buf);

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

    pub(crate) fn create_mesh_old(&self, neighbor_chunks: [Option<Arc<Chunk>>; 6]) -> ChunkMesh {
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

    /// Determines which face of [from] points towards [to]. The two vectors have to be adjacent
    /// to each other or else the function will panic.
    fn from_relative(from: IVec3, to: IVec3) -> Self {
        let direction = to - from;

        match direction.to_array() {
            [0, 1, 0] => Self::UP,
            [0, -1, 0] => Self::DOWN,
            [0, 0, -1] => Self::NORTH,
            [1, 0, 0] => Self::EAST,
            [0, 0, 1] => Self::SOUTH,
            [-1, 0, 0] => Self::WEST,
            _ => panic!("Vectors were not adjacent")
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
