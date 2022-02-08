# svep
WASD + mouse to move around. Current example is just a bunch of randomly generated chunks.
Expect the game to freeze when starting, currently generating chunks blocks everything else from happening too, so just wait until everything's generated.

TO DO:
 - [ ] Chunk management and generation system (+ fix issue with voxels on a chunk border not considering their outwards face)
 - [ ] Multithreaded chunk generation and mesh building.
 - [ ] Colors and different voxel themes/types.
 - [ ] Light sources.
 - [ ] Physics and normal ground-based controls (jumping, etc.).
 - [ ] Modding / plugin API (thread safe and with ECS patterns)