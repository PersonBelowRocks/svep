#[derive(Default, Copy, Clone)]
pub(crate) struct Vertex {
    position: (f32, f32, f32),
}

impl Vertex {
    pub(crate) const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: (x, y, z)
        }
    }
}

vulkano::impl_vertex!(Vertex, position);

#[derive(Default, Copy, Clone)]
pub(crate) struct Normal {
    normal: (f32, f32, f32),
}

impl Normal {
    pub(crate) const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            normal: (x, y, z)
        }
    }
}

vulkano::impl_vertex!(Normal, normal);

#[derive(Default, Copy, Clone)]
pub(crate) struct TexCoord {
    texcoord: (f32, f32)
}

impl TexCoord {
    pub(crate) const fn new(x: f32, y: f32) -> Self {
        Self {
            texcoord: (x, y)
        }
    }
}

#[allow(unused)]
pub(crate) const VERTICES: [Vertex; 8] = [
    // Vertex::new(-0.5, -0.5, -0.5),
    // Vertex::new(0.5, -0.5, -0.5),
    // Vertex::new(0.5, 0.5, -0.5),
    // Vertex::new(-0.5, 0.5, -0.5),
    // Vertex::new(-0.5, -0.5, 0.5),
    // Vertex::new(0.5, -0.5, 0.5),
    // Vertex::new(0.5, 0.5, 0.5),
    // Vertex::new(-0.5, 0.5, 0.5),
    Vertex::new(-0.5, -0.5, 0.5),
    Vertex::new(0.5, -0.5, 0.5),
    Vertex::new(-0.5, 0.5, 0.5),
    Vertex::new(0.5, 0.5, 0.5),

    Vertex::new(-0.5, 0.5, -0.5),
    Vertex::new(0.5, 0.5, -0.5),
    Vertex::new(-0.5, -0.5, -0.5),
    Vertex::new(0.5, -0.5, -0.5),
];

#[allow(unused)]
pub(crate) const NORMALS: [Normal; 12] = [
    // Normal::new(0.0, 0.0, 1.0),
    // Normal::new(0.0, 0.0, 1.0),
    //
    // Normal::new(1.0, 0.0, 0.0),
    // Normal::new(1.0, 0.0, 0.0),
    //
    // Normal::new(0.0, 0.0, -1.0),
    // Normal::new(0.0, 0.0, -1.0),
    //
    // Normal::new(-1.0, 0.0, 0.0),
    // Normal::new(-1.0, 0.0, 0.0),
    //
    // Normal::new(0.0, 1.0, 0.0),
    // Normal::new(0.0, 1.0, 0.0),
    //
    // Normal::new(0.0, -1.0, 0.0),
    // Normal::new(0.0, -1.0, 0.0),

    //

    // Normal::new(1.0, 0.0, 0.0),
    // Normal::new(1.0, 0.0, 0.0),
    // Normal::new(0.0, 1.0, 0.0),
    // Normal::new(0.0, 1.0, 0.0),
    // Normal::new(0.0, 0.0, 1.0),
    // Normal::new(0.0, 0.0, 1.0),
    // Normal::new(-1.0, 0.0, 0.0),
    // Normal::new(-1.0, 0.0, 0.0),
    // Normal::new(0.0, -1.0, 0.0),
    // Normal::new(0.0, -1.0, 0.0),
    // Normal::new(0.0, 0.0, -1.0),
    // Normal::new(0.0, 0.0, -1.0),

    Normal::new(0.0, 0.0, 1.0),
    Normal::new(0.0, 0.0, 1.0),
    Normal::new(0.0, 1.0, 0.0),
    Normal::new(0.0, 1.0, 0.0),
    Normal::new(0.0, 0.0, -1.0),
    Normal::new(0.0, 0.0, -1.0),
    Normal::new(0.0, -1.0, 0.0),
    Normal::new(0.0, -1.0, 0.0),
    Normal::new(1.0, 0.0, 0.0),
    Normal::new(1.0, 0.0, 0.0),
    Normal::new(-1.0, 0.0, 0.0),
    Normal::new(-1.0, 0.0, 0.0),
];

#[allow(unused)]
pub(crate) const TEX_COORDS: [TexCoord; 4] = [
    TexCoord::new(0.0, 0.0),
    TexCoord::new(1.0, 0.0),
    TexCoord::new(1.0, 1.0),
    TexCoord::new(0.0, 1.0),
];

#[allow(unused)]
pub(crate) const INDICES: [u16; 36] = [
    // 0, 1, 3, 2, 1, 3,
    // 1, 5, 2, 2, 5, 6,
    // 5, 4, 6, 6, 4, 7,
    // 4, 0, 7, 7, 0, 3,
    // 3, 2, 7, 7, 2, 6,
    // 4, 5, 0, 0, 5, 1
    0,
    1,
    2,
    2,
    1,
    3,

    2,
    3,
    4,
    4,
    3,
    5,

    4,
    5,
    6,
    6,
    5,
    7,

    6,
    7,
    0,
    0,
    7,
    1,

    1,
    7,
    3,
    3,
    7,
    5,

    6,
    0,
    4,
    4,
    0,
    2,
];

#[allow(unused)]
pub(crate) const TEX_INDICES: [u16; 6] = [
    0, 1, 3, 3, 1, 2
];
