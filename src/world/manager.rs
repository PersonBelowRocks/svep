use std::collections::HashMap;
use std::sync::{Arc, mpsc, RwLock};
use std::time::Instant;
use bevy::prelude::*;

use noise::{NoiseFn, Perlin, Worley, Fbm, SuperSimplex};
use threadpool::ThreadPool;
use crate::util::{CPU_COUNT, Volume};

use crate::world::chunk::{Chunk, ChunkPosition, CHUNK_SIZE, ChunkMesh};
use crate::world::voxel::Voxel;

const PERLIN_THRESHOLD: f64 = 0.33;
const CHUNK_SIZE_F64: f64 = CHUNK_SIZE as f64;

// The chunk manager is responsible for storing state associated with chunks and chunk generation.
// When a player moves and loads / generates new chunks, the chunk manager dispatches worker
// threads that do the generation and mesh building of these chunks.
// This happens in a sort of pipeline:
//     Chunk position -> {thread pool} generation passes -> SYNC -> {thread pool} mesh building ->
//     mesh loading
//
// First, the positions of all chunks that are supposed to be generated is obtained. The positions
// are distributed among worker threads that each build a chunk of terrain from the chunk position,
// chunk size, and noise algorithm(s)/generation passes. A generation pass is one "iteration" through
// the chunk's terrain, for example one pass can be used to generate the base surface terrain (large
// "background" geometry like mountains, valleys, plains, etc) and then another pass generates canyons,
// rivers, caves, etc. This can almost be though of as a shader pass; some input variables result in
// some output geometry, and communication across these passes/workers is not allowed.
//
// After the terrain generation, all workers sync up and add the generated chunks to the manager.
// Then all workers will start up again for the second stage (a sort of fragment shader if you will)
// where the chunk geometry is read and analyzed to produce meshes for each chunk. Here the workers
// are allowed to communicate with the chunk manager, this allows them to check chunks adjacent to
// the one they're meshing to see if a face should be drawn there or not.
// This stage is also interesting because the number of chunks generated is not the same as the number
// of chunks that are meshed. This is because if a chunk is generated with no geometry it's marked as
// "empty" and can be disregarded when rendering because there's nothing to actually render.
//
// After meshes are generated they are loaded into the Assets<Mesh> resource and a handle for them is
// stored by the chunk manager.
//
// Note about generating geometry adjacent to existing, meshed, and rendered geometry:
// When a worker meshes a chunk it can only generate faces for that chunk, not any other chunks.
// This would create the following scenario (2D with 4x4 chunks, 0=active 1=inactive):
//     1)
//         1 1 1 1 -- We start with a completely solid rendered chunk.
//         1 1 1 1    The 3D equivalent appearance of this chunk would be a full chunk-sized cube.
//         1 1 1 1
//         1 1 1 1
//
//     2)
//         old          new
//         1 1 1 1 <--- 1 0 0 0 -- Here we're generating a new chunk adjacent to the old one.
//         1 1 1 1 <--- 1 0 0 0    This chunk's left side is completely full of voxels and will obscure
//         1 1 1 1 <--- 1 0 0 0    the old chunk's right side. That means the old chunks right side is
//         1 1 1 1 <--- 1 0 0 0    no longer visible and we shouldn't draw it. But because workers cannot
//                                 draw meshes for any other chunk than the one it's working on, we will
//                                 have to submit the old chunk for re-meshing along with the new chunk.
//
//     3) NOT CORRECT! Current implementation gives each worker a pointer to all adjacent chunks, no
//        asking is happening! This may however change in the future.
//
//   *---- mesh worker 1
//   |
//   |            mesh worker 2 ----*  Our chunks are currently being meshed, notice the voxel in
//   |     old         new          |  the [brackets], it's on the edge of a chunk and requires
//   |     1 1 1 1     1 0 0 0      |  information about the adjacent chunk to be meshed. Meshing does
//   *-->  1 1 1 1     1 0 0 0   <--*  not mutate the actual chunk so it's fine to ask about adjacent
//         1 1 1 1     1 0 0 0         chunks even if they're also currently being meshed (chunk data
//         1 1 1[1]   <1>0 0 0         does not suddenly change during the mesh stage, so all workers
//              /      ^               will see the same thing at all times, making things simple).
//             /       *----------------------*
// "what's to the right side of this voxel?"  |
//            |                "a solid voxel, don't draw a face there!"
//            V                               |
//          {CHUNK MANAGER} ------------------*

pub(crate) struct ChunkManager {
    chunks: Arc<RwLock<HashMap<ChunkPosition, Arc<Chunk>>>>,
    visible_chunks: Arc<HashMap<ChunkPosition, Handle<Mesh>>>,
    noisegen: Perlin,
    worker_pool: ThreadPool
}

impl Default for ChunkManager {
    fn default() -> Self {
        Self {
            chunks: Arc::new(RwLock::new(HashMap::new())),
            visible_chunks: Arc::new(HashMap::new()),
            noisegen: Perlin::new(),
            worker_pool: ThreadPool::new(*CPU_COUNT),
        }
    }
}

impl ChunkManager {
    pub(crate) fn gen_mesh_multi(&mut self, positions: Vec<ChunkPosition>) -> Vec<ChunkMesh> {
        let then = Instant::now();
        let chunks = self.generate_par(positions)
            .into_iter()
            .map(|chnk| Arc::new(chnk))
            .collect::<Vec<_>>();

        let elapsed = Instant::now().duration_since(then);
        println!("generation took {}ms", elapsed.as_millis());

        let then = Instant::now();
        for chunk in chunks.iter().cloned() {
            self.chunks.write().unwrap().insert(chunk.position(), chunk);
        }

        let elapsed = Instant::now().duration_since(then);
        println!("remembering took {}ms", elapsed.as_millis());

        let then = Instant::now();
        let meshes = self.mesh_par(chunks);

        let elapsed = Instant::now().duration_since(then);
        println!("meshing took {}ms", elapsed.as_millis());

        meshes
    }

    pub(crate) fn generate_new<G: NoiseFn<[f64; 3]>>(noisegen: G, pos: ChunkPosition) -> Chunk {
        let mut vol: Volume<_, 32> = Volume::filled(Voxel::active());

        for idx in vol.iter_indices() {
            let x = (idx.0 as f64 / CHUNK_SIZE_F64) + (pos.x as f64);
            let y = (idx.1 as f64 / CHUNK_SIZE_F64) + (pos.y as f64);
            let z = (idx.2 as f64 / CHUNK_SIZE_F64) + (pos.z as f64);

            let noise = noisegen.get([x/2.0, y/2.0, z/2.0]);
            if noise < PERLIN_THRESHOLD {
                vol[idx].active = false;
            }
        }

        // self.chunks.push(Chunk::new(pos, vol));
        Chunk::new(pos, vol)
    }

    fn generate_par(&self, chunks: Vec<ChunkPosition>) -> Vec<Chunk> {
        let expected_amount = chunks.len();

        let (tx, rx) = mpsc::channel::<Chunk>();

        for chunk_pos in chunks {
            let this_tx = tx.clone();
            let this_noisegen = self.noisegen.clone();

            self.worker_pool.execute(move || {
                this_tx.send(Self::generate_new(this_noisegen, chunk_pos)).unwrap();
            });
        }

        let mut collected = Vec::new();
        while collected.len() < expected_amount {
            collected.push(rx.recv().unwrap());
        }

        collected
    }

    fn mesh_par(&self, chunks: Vec<Arc<Chunk>>) -> Vec<ChunkMesh> {
        let expected_amount = chunks.len();

        let (tx, rx) = mpsc::channel::<ChunkMesh>();

        for chunk in chunks {
            let this_tx = tx.clone();
            let this_chunk_registry = self.chunks.clone();

            self.worker_pool.execute(move || {
                let chunk_registry = this_chunk_registry.read().unwrap();

                let mut neighbor_chunks = [None, None, None, None, None, None];
                for (idx, neighbor_chunk_pos) in neighbor_vecs(chunk.position()).into_iter().enumerate() {
                    neighbor_chunks[idx] = chunk_registry.get(&neighbor_chunk_pos).cloned();
                }

                this_tx.send(chunk.create_mesh(neighbor_chunks)).unwrap();
            });
        }

        let mut collected = Vec::new();

        while collected.len() < expected_amount {
            collected.push(rx.recv().unwrap());
        }

        collected
    }
}

#[inline(always)]
pub(crate) fn neighbor_vecs(vec: IVec3) -> [IVec3; 6] {
    [
        IVec3::new(0, 1, 0),
        IVec3::new(0, -1, 0),
        IVec3::new(1, 0, 0),
        IVec3::new(-1, 0, 0),
        IVec3::new(0, 0, 1),
        IVec3::new(0, 0, -1),
    ].map(|offset| offset + vec)
}