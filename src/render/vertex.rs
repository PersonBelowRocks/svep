#![allow(unused)]
use cgmath::Vector3;

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct Vertex {
    position: [f32; 3],
}

impl Vertex {
    pub(crate) const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { position: [x, y, z] }
    }

    pub(crate) fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ]
        }
    }
}

type FaceVectors = [Vector3<f32>; 6];

// Faces
const UP: FaceVectors = [
    Vector3::new(-0.5, 0.5, 0.5),
    Vector3::new(0.5, 0.5, 0.5),
    Vector3::new(-0.5, 0.5, -0.5),

    Vector3::new(0.5, 0.5, -0.5),
    Vector3::new(0.5, 0.5, 0.5),
    Vector3::new(-0.5, 0.5, -0.5)
];

const DOWN: FaceVectors = [
    Vector3::new(-0.5, -0.5, 0.5),
    Vector3::new(-0.5, -0.5, -0.5),
    Vector3::new(0.5, -0.5, 0.5),

    Vector3::new(0.5, -0.5, -0.5),
    Vector3::new(0.5, -0.5, 0.5),
    Vector3::new(-0.5, -0.5, -0.5)
];

const NORTH: FaceVectors = [
    Vector3::new(0.5, 0.5, -0.5),
    Vector3::new(-0.5, 0.5, -0.5),
    Vector3::new(0.5, -0.5, -0.5),

    Vector3::new(-0.5, -0.5, -0.5),
    Vector3::new(-0.5, 0.5, -0.5),
    Vector3::new(0.5, -0.5, -0.5)
];
