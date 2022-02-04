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

/*
pub const _VERTICES: [Vertex; 36] = [
    // Face 1
    // 0, 7, 5
    Vertex { position: (0.5, 0.5, 0.5) },
    Vertex { position: (0.5, -0.5, 0.5) },
    Vertex { position: (0.5, 0.5, -0.5) },
    // 4, 7, 5
    Vertex { position: (0.5, -0.5, -0.5) },
    Vertex { position: (0.5, -0.5, 0.5) },
    Vertex { position: (0.5, 0.5, -0.5) },

    // Face 2
    // 5, 6, 4
    Vertex { position: (0.5, 0.5, -0.5) },
    Vertex { position: (-0.5, 0.5, -0.5) },
    Vertex { position: (0.5, -0.5, -0.5) },
    // 3, 6, 4
    Vertex { position: (-0.5, -0.5, -0.5) },
    Vertex { position: (-0.5, 0.5, -0.5) },
    Vertex { position: (0.5, -0.5, -0.5) },

    // Face 3
    // 6, 1, 3
    Vertex { position: (-0.5, 0.5, -0.5) },
    Vertex { position: (-0.5, 0.5, 0.5) },
    Vertex { position: (-0.5, -0.5, -0.5) },
    // 2, 3, 1
    Vertex { position: (-0.5, -0.5, 0.5) },
    Vertex { position: (-0.5, -0.5, -0.5) },
    Vertex { position: (-0.5, 0.5, 0.5) },

    // Face 4
    // 1, 0, 2
    Vertex { position: (-0.5, 0.5, 0.5) },
    Vertex { position: (0.5, 0.5, 0.5) },
    Vertex { position: (-0.5, -0.5, 0.5) },
    // 7, 0, 2
    Vertex { position: (0.5, -0.5, 0.5) },
    Vertex { position: (0.5, 0.5, 0.5) },
    Vertex { position: (-0.5, -0.5, 0.5) },

    // Face 5
    // 1, 0, 6
    Vertex { position: (-0.5, 0.5, 0.5) },
    Vertex { position: (0.5, 0.5, 0.5) },
    Vertex { position: (-0.5, 0.5, -0.5) },
    // 5, 0, 6
    Vertex { position: (0.5, 0.5, -0.5) },
    Vertex { position: (0.5, 0.5, 0.5) },
    Vertex { position: (-0.5, 0.5, -0.5) },

    // Face 6
    // 2, 3, 7
    Vertex { position: (-0.5, -0.5, 0.5) },
    Vertex { position: (-0.5, -0.5, -0.5) },
    Vertex { position: (0.5, -0.5, 0.5) },
    // 4, 7, 3
    Vertex { position: (0.5, -0.5, -0.5) },
    Vertex { position: (0.5, -0.5, 0.5) },
    Vertex { position: (-0.5, -0.5, -0.5) },
];
 */
