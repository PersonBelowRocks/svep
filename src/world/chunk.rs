#![allow(unused)]
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use crate::util::{Volume, VolumeIdx, FaceVectors, FaceMesh};
use super::voxel::Voxel;

pub(crate) const CHUNK_SIZE: usize = 32;
pub(crate) type ChunkPosition = (u32, u32, u32);

pub(crate) struct Chunk {
    position: ChunkPosition,
    empty: bool,
    volume: Volume<Voxel, CHUNK_SIZE>,
}

pub(crate) struct ChunkMesh {
    vertices: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    indices: Vec<u32>
}

impl ChunkMesh {
    fn empty() -> Self {
        Self {
            vertices: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            indices: Vec::new(),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Mesh> for ChunkMesh {
    fn into(self) -> Mesh {
        let mut out = Mesh::new(PrimitiveTopology::TriangleList);

        /*let vertices = self
            .vertices
            .iter()
            .map(|a| a.0.to_array())
            .collect::<Vec<_>>();

        println!("vertices going to GPU mem: {:?}", &vertices);
        out.set_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

        out.set_attribute(Mesh::ATTRIBUTE_NORMAL,
                          self
                              .vertices
                              .iter()
                              .map(|a| a.1.to_array())
                              .collect::<Vec<_>>());

        out.set_attribute(Mesh::ATTRIBUTE_COLOR,
                          (0..self.vertices.len()).map(|_| [0.5, 0.5, 0.82, 1.0])
                              .collect::<Vec<_>>());

        out.set_indices(Some(Indices::U32(
            (0u32..(self.vertices.len() as u32)).collect::<Vec<u32>>()
        )));*/

        //println!("vertices: {:?}\n", &self.vertices);
        //println!("normals: {:?}\n", &self.normals);
        //println!("indices: {:?}\n", &self.indices);

        out.set_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices);
        out.set_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        out.set_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs);
        out.set_indices(Some(Indices::U32(self.indices)));

        out
    }
}

impl Chunk {
    pub(crate) fn example_chunk() -> Self {
        let mut volume = Volume::filled(Voxel::inactive());

        volume[(5, 5, 5)].active = true;
        volume[(5, 4, 5)].active = true;
        volume[(5, 6, 5)].active = true;

        Self {
            position: (1, 0, 0),
            empty: false,
            volume
        }
    }

    pub(crate) fn random_chunk(position: ChunkPosition) -> Self {
        let mut volume = Volume::filled(Voxel::inactive());
        let mut empty = true;

        for idx in volume.iter_indices() {
            if rand::random::<f32>() > 0.075 {
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
        let mut mesh = ChunkMesh::empty();
        let mut current_index = 0u32;

        for (idx, voxel) in self.volume.iter() {
            if !voxel.active { continue; }
            let this_pos = volume_idx_to_vec(idx);

            for (direction, neighbor_idx) in NeighborIterator::from_idx(idx) {
                let neighbor_voxel = self.volume[neighbor_idx];
                if !neighbor_voxel.active {
                    let neighbor_pos = volume_idx_to_vec(neighbor_idx);

                    // TODO: mesh is "backwards", so triangles facing you are culled instead of the
                    //  reverse. lighting is also messed up. investigate and fix!
                    for vertex in direction.get_face_mesh() {
                        // Vertex position
                        mesh.vertices.push( (Vec3::from(vertex.0) + this_pos).into() );
                        // mesh.vertices.push(vertex.0);
                        // Vertex normal
                        mesh.normals.push(vertex.1);
                        // Vertex uv coords
                        mesh.uvs.push(vertex.2);

                    }
                    for index_offset in [0, 1, 2, 2, 3, 0u32] {
                        mesh.indices.push(index_offset + current_index);
                    }
                    current_index += 4;
                }
            }
        }
        //println!("{:?}", mesh.vertices);
        //println!("vertex count: {}", mesh.vertices.len());

        mesh
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

/*    fn get_face_normal(&self) -> Vec3 {
        use crate::util::{
            UP_NORMAL,
            DOWN_NORMAL,
            NORTH_NORMAL,
            EAST_NORMAL,
            SOUTH_NORMAL,
            WEST_NORMAL
        };

        match self {
            Self::UP => *UP_NORMAL,
            Self::DOWN => *DOWN_NORMAL,
            Self::NORTH => *NORTH_NORMAL,
            Self::EAST => *EAST_NORMAL,
            Self::SOUTH => *SOUTH_NORMAL,
            Self::WEST => *WEST_NORMAL
        }
    }*/
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
