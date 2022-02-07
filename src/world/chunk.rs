#![allow(unused)]
use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use crate::util::{Volume, VolumeIdx, FaceVectors};
use super::voxel::Voxel;

pub(crate) const CHUNK_SIZE: usize = 32;
pub(crate) type ChunkPosition = (u32, u32, u32);

pub(crate) struct Chunk {
    position: ChunkPosition,
    empty: bool,
    volume: Volume<Voxel, CHUNK_SIZE>,
}

pub(crate) struct ChunkMesh {
    vertices: Vec<(Vec3, Vec3, Vec2)>
}

impl ChunkMesh {
    fn empty() -> Self {
        Self { vertices: Vec::new() }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Mesh> for ChunkMesh {
    fn into(self) -> Mesh {
        let mut out = Mesh::new(PrimitiveTopology::TriangleList);

        out.set_attribute(Mesh::ATTRIBUTE_POSITION,
                          self
                              .vertices
                              .iter()
                              .map(|a| a.0.to_array())
                              .collect::<Vec<_>>());

        out.set_attribute(Mesh::ATTRIBUTE_NORMAL,
                          self
                              .vertices
                              .iter()
                              .map(|a| a.1.to_array())
                              .collect::<Vec<_>>());

        out.set_attribute(Mesh::ATTRIBUTE_COLOR,
                          (0..self.vertices.len()).map(|_| [0.5, 0.5, 0.82, 1.0])
                              .collect::<Vec<_>>());

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
        let mut mesh = ChunkMesh::empty();

        for (idx, voxel) in self.volume.iter() {
            if !voxel.active { continue; }

            for (direction, neighbor_idx) in NeighborIterator::from_idx(idx) {
                let neighbor_voxel = self.volume[neighbor_idx];
                if !neighbor_voxel.active {
                    let neighbor_pos = volume_idx_to_vec(neighbor_idx);

                    for vertex in direction.get_face_mesh() {
                        mesh.vertices.push((
                                vertex + neighbor_pos,
                                direction.get_face_normal(),
                                Vec2::new(1.0, 1.0)
                            ));
                    }
                }
            }
        }
        println!("{:?}", mesh.vertices);
        println!("vertex count: {}", mesh.vertices.len());

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
    fn get_face_mesh(&self) -> FaceVectors {
        use crate::util::{
            FACE_UP,
            FACE_DOWN,
            FACE_NORTH,
            FACE_EAST,
            FACE_SOUTH,
            FACE_WEST
        };

        match self {
            Self::UP => *FACE_UP,
            Self::DOWN => *FACE_DOWN,
            Self::NORTH => *FACE_NORTH,
            Self::EAST => *FACE_EAST,
            Self::SOUTH => *FACE_SOUTH,
            Self::WEST => *FACE_WEST
        }
    }

    fn get_face_normal(&self) -> Vec3 {
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
