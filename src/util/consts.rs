use bevy::prelude::*;

pub(crate) type FaceVectors = [Vec3; 6];

const MAX_X: f32 = 0.5;
const MIN_X: f32 = -0.5;
const MAX_Y: f32 = 0.5;
const MIN_Y: f32 = -0.5;
const MAX_Z: f32 = 0.5;
const MIN_Z: f32 = -0.5;

pub(crate) type FaceMesh = [([f32; 3], [f32; 3], [f32; 2]); 4];

// TODO: These can be constants
lazy_static! {
/*    // +Y
    pub(crate) static ref UP_NORMAL: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    pub(crate) static ref FACE_UP: FaceVectors = [
        Vec3::new(-0.5, 0.5, 0.5),
        Vec3::new(0.5, 0.5, 0.5),
        Vec3::new(-0.5, 0.5, -0.5),

        Vec3::new(0.5, 0.5, -0.5),
        Vec3::new(0.5, 0.5, 0.5),
        Vec3::new(-0.5, 0.5, -0.5)
    ];

    // -Y
    pub(crate) static ref DOWN_NORMAL: Vec3 = Vec3::new(0.0, -1.0, 0.0);
    pub(crate) static ref FACE_DOWN: FaceVectors = [
        Vec3::new(-0.5, -0.5, 0.5),
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new(0.5, -0.5, 0.5),

        Vec3::new(0.5, -0.5, -0.5),
        Vec3::new(0.5, -0.5, 0.5),
        Vec3::new(-0.5, -0.5, -0.5)
    ];

    // -Z
    pub(crate) static ref NORTH_NORMAL: Vec3 = Vec3::new(0.0, 0.0, -1.0);
    pub(crate) static ref FACE_NORTH: FaceVectors = [
        Vec3::new(0.5, 0.5, -0.5),
        Vec3::new(-0.5, 0.5, -0.5),
        Vec3::new(0.5, -0.5, -0.5),

        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new(-0.5, 0.5, -0.5),
        Vec3::new(0.5, -0.5, -0.5)
    ];

    // +X
    pub(crate) static ref EAST_NORMAL: Vec3 = Vec3::new(1.0, 0.0, 0.0);
    pub(crate) static ref FACE_EAST: FaceVectors = [
        Vec3::new(0.5, 0.5, 0.5),
        Vec3::new(0.5, -0.5, 0.5),
        Vec3::new(0.5, 0.5, -0.5),

        Vec3::new(0.5, -0.5, -0.5),
        Vec3::new(0.5, -0.5, 0.5),
        Vec3::new(0.5, 0.5, -0.5),
    ];

    // +Z
    pub(crate) static ref SOUTH_NORMAL: Vec3 = Vec3::new(0.0, 0.0, 1.0);
    pub(crate) static ref FACE_SOUTH: FaceVectors = [
        Vec3::new(-0.5, 0.5, 0.5),
        Vec3::new(0.5, 0.5, 0.5),
        Vec3::new(-0.5, -0.5, 0.5),

        Vec3::new(0.5, -0.5, 0.5),
        Vec3::new(0.5, 0.5, 0.5),
        Vec3::new(-0.5, -0.5, 0.5),
    ];

    // -X
    pub(crate) static ref WEST_NORMAL: Vec3 = Vec3::new(-1.0, 0.0, 0.0);
    pub(crate) static ref FACE_WEST: FaceVectors = [
        Vec3::new(-0.5, 0.5, -0.5),
        Vec3::new(-0.5, 0.5, 0.5),
        Vec3::new(-0.5, -0.5, -0.5),

        Vec3::new(-0.5, -0.5, 0.5),
        Vec3::new(-0.5, -0.5, -0.5),
        Vec3::new(-0.5, 0.5, 0.5),
    ];*/
    // Top +Z
    pub(crate) static ref PZ_FACE: FaceMesh = [
        ([MIN_X, MIN_Y, MAX_Z], [0., 0., 1.0], [0., 0.]),
        ([MAX_X, MIN_Y, MAX_Z], [0., 0., 1.0], [1.0, 0.]),
        ([MAX_X, MAX_Y, MAX_Z], [0., 0., 1.0], [1.0, 1.0]),
        ([MIN_X, MAX_Y, MAX_Z], [0., 0., 1.0], [0., 1.0]),
    ];
    // Bottom -Z
    pub(crate) static ref NZ_FACE: FaceMesh = [
        ([MIN_X, MAX_Y, MIN_Z], [0., 0., -1.0], [1.0, 0.]),
        ([MAX_X, MAX_Y, MIN_Z], [0., 0., -1.0], [0., 0.]),
        ([MAX_X, MIN_Y, MIN_Z], [0., 0., -1.0], [0., 1.0]),
        ([MIN_X, MIN_Y, MIN_Z], [0., 0., -1.0], [1.0, 1.0]),
    ];
    // Right +X
    pub(crate) static ref PX_FACE: FaceMesh = [
        ([MAX_X, MIN_Y, MIN_Z], [1.0, 0., 0.], [0., 0.]),
        ([MAX_X, MAX_Y, MIN_Z], [1.0, 0., 0.], [1.0, 0.]),
        ([MAX_X, MAX_Y, MAX_Z], [1.0, 0., 0.], [1.0, 1.0]),
        ([MAX_X, MIN_Y, MAX_Z], [1.0, 0., 0.], [0., 1.0]),
    ];
    // Left -X
    pub(crate) static ref NX_FACE: FaceMesh = [
        ([MIN_X, MIN_Y, MAX_Z], [-1.0, 0., 0.], [1.0, 0.]),
        ([MIN_X, MAX_Y, MAX_Z], [-1.0, 0., 0.], [0., 0.]),
        ([MIN_X, MAX_Y, MIN_Z], [-1.0, 0., 0.], [0., 1.0]),
        ([MIN_X, MIN_Y, MIN_Z], [-1.0, 0., 0.], [1.0, 1.0]),
    ];
    // Front +Y
    pub(crate) static ref PY_FACE: FaceMesh = [
        ([MAX_X, MAX_Y, MIN_Z], [0., 1.0, 0.], [1.0, 0.]),
        ([MIN_X, MAX_Y, MIN_Z], [0., 1.0, 0.], [0., 0.]),
        ([MIN_X, MAX_Y, MAX_Z], [0., 1.0, 0.], [0., 1.0]),
        ([MAX_X, MAX_Y, MAX_Z], [0., 1.0, 0.], [1.0, 1.0]),
    ];
    // Back -Y
    pub(crate) static ref NY_FACE: FaceMesh = [
        ([MAX_X, MIN_Y, MAX_Z], [0., -1.0, 0.], [0., 0.]),
        ([MIN_X, MIN_Y, MAX_Z], [0., -1.0, 0.], [1.0, 0.]),
        ([MIN_X, MIN_Y, MIN_Z], [0., -1.0, 0.], [1.0, 1.0]),
        ([MAX_X, MIN_Y, MIN_Z], [0., -1.0, 0.], [0., 1.0]),
    ];
}
// Faces
// +Y
// pub(crate) const UP_NORMAL: Vec3 = Vec3::new(0.0, 1.0, 0.0);
// pub(crate) const FACE_UP: FaceVectors = [
//     Vec3::new(-0.5, 0.5, 0.5),
//     Vec3::new(0.5, 0.5, 0.5),
//     Vec3::new(-0.5, 0.5, -0.5),
//
//     Vec3::new(0.5, 0.5, -0.5),
//     Vec3::new(0.5, 0.5, 0.5),
//     Vec3::new(-0.5, 0.5, -0.5)
// ];

// -Y
// pub(crate) const DOWN_NORMAL: Vec3 = Vec3::new(0.0, -1.0, 0.0);
// pub(crate) const FACE_DOWN: FaceVectors = [
//     Vec3::new(-0.5, -0.5, 0.5),
//     Vec3::new(-0.5, -0.5, -0.5),
//     Vec3::new(0.5, -0.5, 0.5),
//
//     Vec3::new(0.5, -0.5, -0.5),
//     Vec3::new(0.5, -0.5, 0.5),
//     Vec3::new(-0.5, -0.5, -0.5)
// ];

// -Z
// pub(crate) const NORTH_NORMAL: Vec3 = Vec3::new(0.0, 0.0, -1.0);
// pub(crate) const FACE_NORTH: FaceVectors = [
//     Vec3::new(0.5, 0.5, -0.5),
//     Vec3::new(-0.5, 0.5, -0.5),
//     Vec3::new(0.5, -0.5, -0.5),
//
//     Vec3::new(-0.5, -0.5, -0.5),
//     Vec3::new(-0.5, 0.5, -0.5),
//     Vec3::new(0.5, -0.5, -0.5)
// ];

// +X
// pub(crate) const EAST_NORMAL: Vec3 = Vec3::new(1.0, 0.0, 0.0);
// pub(crate) const FACE_EAST: FaceVectors = [
//     Vec3::new(0.5, 0.5, 0.5),
//     Vec3::new(0.5, -0.5, 0.5),
//     Vec3::new(0.5, 0.5, -0.5),
//
//     Vec3::new(0.5, -0.5, -0.5),
//     Vec3::new(0.5, -0.5, 0.5),
//     Vec3::new(0.5, 0.5, -0.5),
// ];

// +Z
// pub(crate) const SOUTH_NORMAL: Vec3 = Vec3::new(0.0, 0.0, 1.0);
// pub(crate) const FACE_SOUTH: FaceVectors = [
//     Vec3::new(-0.5, 0.5, 0.5),
//     Vec3::new(0.5, 0.5, 0.5),
//     Vec3::new(-0.5, -0.5, 0.5),
//
//     Vec3::new(0.5, -0.5, 0.5),
//     Vec3::new(0.5, 0.5, 0.5),
//     Vec3::new(-0.5, -0.5, 0.5),
// ];

// -X
// pub(crate) const WEST_NORMAL: Vec3 = Vec3::new(-1.0, 0.0, 0.0);
// pub(crate) const FACE_WEST: FaceVectors = [
//     Vec3::new(-0.5, 0.5, -0.5),
//     Vec3::new(-0.5, 0.5, 0.5),
//     Vec3::new(-0.5, -0.5, -0.5),
//
//     Vec3::new(-0.5, -0.5, 0.5),
//     Vec3::new(-0.5, -0.5, -0.5),
//     Vec3::new(-0.5, 0.5, 0.5),
// ];