#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub texture: [f32; 2],
}

implement_vertex!(Vertex, position, normal, texture);

use nalgebra_glm::{Vec3, Mat4};
pub fn view_matrix(pos: &Vec3, direction: &Vec3, up: &Vec3) -> Mat4 {

    let f = direction.normalize();
    let s_norm = up.cross(&f).normalize();
    let u = f.cross(&s_norm);

    let p = Vec3::from([
        -pos.dot(&s_norm),
        -pos.dot(&u),
        -pos.dot(&f),
    ]);

    let mut candidate = Mat4::from([
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [     p[0], p[1], p[2], 1.0],
    ]);

    for v in candidate.iter_mut() {
        if v.is_nan() {
            *v = 0.0;
        }
    }

    candidate

    // todo!()

    // Mat4::from([
    //     [1.0, 0.0, 0.0, 0.0],
    //     [0.0, 1.0, 0.0, 0.0],
    //     [0.0, 0.0, 1.0, 0.0],
    //     [0.0, 0.0, 0.0, 1.0],
    // ])
}
