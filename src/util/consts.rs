use bevy::prelude::*;

pub(crate) type FaceVectors = [Vec3; 6];

lazy_static! {
    // +Y
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