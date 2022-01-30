#[macro_use]
extern crate glium;

use std::f32::consts::FRAC_PI_2;
use std::time::Duration;
use glium::{vertex, Surface};
use crate::chunks::Voxel;

mod loader;
mod teapot;
mod util;
mod chunks;
mod render;

// Scan codes
const W: u32 = 0x11;
const A: u32 = 0x1e;
const S: u32 = 0x1f;
const D: u32 = 0x20;
const SPACE: u32 = 0x39;
const LSHIFT: u32 = 0x2a;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn filled_chunk(chunk_pos: (f32, f32, f32)) -> chunks::Chunk {
    let mut voxels: Vec<Vec<Vec<Option<chunks::Voxel>>>> = vec![vec![vec![None; 128]; 128]; 128];

    for i in 0..128 {
        for ii in 0..128 {
            for iii in 0..128 {
                voxels[i][ii][iii] = Some(Voxel::new_anon((i as f32, ii as f32, iii as f32)))
            }
        }
    }

    chunks::Chunk::new(chunk_pos, voxels)
}

fn main() {
    /*
    //
    //
    // // Panics if can't find source code
    // let shader = loader::load_shaders(&display, "resources/shaders").unwrap();
    //
    // // Load voxel cube
    // let (model, _) = loader::load_wavefront_file(&display, "resources/models/cube.obj");
    // let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    //
    // use std::sync::mpsc;
    //
    // let (tx, rx) = mpsc::channel::<chunks::ChunkRenderOp>();
    // // This thread will handle miscellaneous I/O stuff once we get to that.
    // std::thread::spawn(move || {
    //
    //     let mut voxels = Vec::new();
    //
    //     for _ in 0..10_000 {
    //         voxels.push(
    //             chunks::Voxel::new_anon((
    //                 rand::random::<f32>() * 128.0,
    //                 rand::random::<f32>() * 128.0,
    //                 rand::random::<f32>() * 128.0,
    //             )));
    //     }
    //
    //
    //     // std::thread::sleep(std::time::Duration::from_secs(1));
    //     let mut chunk = chunks::Chunk::new(
    //         (0.0, 0.0, 0.0),
    //         voxels.clone()
    //     );
    //
    //     tx.send(chunks::ChunkRenderOp::Load(chunk)).unwrap();
    //
    //     std::thread::sleep(std::time::Duration::from_secs(5));
    //     let mut chunk = chunks::Chunk::new(
    //         (1.0, 0.0, 0.0),
    //         voxels.clone()
    //     );
    //
    //     tx.send(chunks::ChunkRenderOp::Load(chunk)).unwrap();
    //
    //
    //
    //
    //     // tx.send(chunks::ChunkRenderOp::Unload((0, 0, 0))).unwrap();
    //
    // });
    //
    // use std::sync::Arc;
    // use hashbrown::HashMap;
    // let mut loaded_chunks: HashMap<(i32, i32, i32), Arc<chunks::Chunk>> = HashMap::new();
    //
    // // When we started
    // use std::time;
    // let then = time::Instant::now();
    //
    // // This hijacks the main thread and starts drawing frames.
    // event_loop.run(move |ev, _, control_flow| {
    //     let next_frame_time =
    //         std::time::Instant::now() + std::time::Duration::from_secs_f64(1.0 / 144.0);
    //
    //     // This is the time uniform we feed to our shaders.
    //     let t = time::Instant::now().duration_since(then).as_secs_f32();
    //
    //
    //
    //     use nalgebra_glm::{Mat4, Vec3};
    //
    //     // Draw a frame.
    //     let mut target = display.draw();
    //
    //     let rot = Vec3::new(0.0, t, 0.0);
    //
    //     let scaling = Mat4::from([
    //         [0.2, 0.0, 0.0, 0.0],
    //         [0.0, 0.2, 0.0, 0.0],
    //         [0.0, 0.0, 0.2, 0.0],
    //         [0.0, 0.0, 100.0, 1.0f32],
    //     ]);
    //
    //     let transform = scaling * Mat4::new_rotation(rot);
    //
    //     let perspective = {
    //         let (w, h) = target.get_dimensions();
    //
    //         let fov: f32 = nalgebra_glm::pi::<f32>() / 3.0;
    //         let f = 1.0 / (fov / 2.0).tan();
    //
    //         let aspect_ratio = h as f32 / w as f32;
    //
    //         let zfar = 1024.0;
    //         let znear = 0.1;
    //
    //         Mat4::from([
    //             [f * aspect_ratio, 0.0, 0.0, 0.0],
    //             [0.0, f, 0.0, 0.0],
    //             [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
    //             [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
    //         ])
    //     };
    //
    //     let transform_r: [[f32; 4]; 4] = transform.into();
    //     let perspective_r: [[f32; 4]; 4] = perspective.into();
    //
    //     // Build our shader's uniforms
    //     let uniforms = uniform! {
    //         t: t,
    //         transform: transform_r,
    //         perspective: perspective_r
    //     };
    //
    //     // #[derive(Copy, Clone)]
    //     // struct Attr {
    //     //     world_position: (f32, f32, f32),
    //     // }
    //     //
    //     // implement_vertex!(Attr, world_position);
    //     //
    //     // let y_offset = -2.0 + (t * 30.0).cos() * 2.5;
    //     /*
    //     let mut per_instance = vertex::VertexBuffer::dynamic(
    //         &display,
    //         // &[
    //         //     Attr { world_position: (1.0, 0.0 + y_offset, 0.0) },
    //         //     Attr { world_position: (-1.0, 0.0 + y_offset, 0.0) },
    //         //     Attr { world_position: (1.0, 1.0 + y_offset, 0.0) },
    //         //     Attr { world_position: (-1.0, 1.0 + y_offset, 0.0) },
    //         //     Attr { world_position: (0.0, 1.0 + y_offset, 0.0) },
    //         //
    //         //     Attr { world_position: (1.0, 2.0 + y_offset, 0.0) },
    //         //     Attr { world_position: (0.0, 2.0 + y_offset, 0.0) },
    //         //     Attr { world_position: (-1.0, 2.0 + y_offset, 0.0) },
    //         //
    //         //     Attr { world_position: (-1.0, 3.0 + y_offset, 0.0) },
    //         //     Attr { world_position: (0.0, 4.0 + y_offset, 0.0) },
    //         //     Attr { world_position: (1.0, 4.0 + y_offset, 0.0) },
    //         // ]
    //         // {
    //         //     let vecs = [
    //         //         Vec3::new(1.0, 1.0, 0.0),
    //         //         Vec3::new(1.0, 2.0, 0.0),
    //         //         Vec3::new(0.0, 3.0, 0.0),
    //         //         Vec3::new(-1.0, 3.0, 0.0),
    //         //         Vec3::new(-2.0, 2.0, 0.0),
    //         //         Vec3::new(0.0, 0.0, 0.0),
    //         //         Vec3::new(1.0, -1.0, 0.0),
    //         //         Vec3::new(1.0, -2.0, 0.0),
    //         //         Vec3::new(0.0, -3.0, 0.0),
    //         //         Vec3::new(-1.0, -3.0, 0.0),
    //         //         Vec3::new(-2.0, -2.0, 0.0),
    //         //
    //         //         // eyes
    //         //         Vec3::new(-4.0, 2.0, 0.0),
    //         //         Vec3::new(-4.0, -2.0, 0.0),
    //         //     ];
    //         //
    //         //     let offset = Vec3::new(rand::random::<f32>() + 2.0, rand::random::<f32>(), rand::random::<f32>());
    //         //     &vecs.map(|v| {
    //         //             let spazzed = (v + offset);
    //         //             Attr {world_position: (spazzed.x, spazzed.y, spazzed.z)}
    //         //         }
    //         //     )
    //         // }
    //         &[Attr {
    //             world_position: (0.0, 0.0, 0.0),
    //         }],
    //     )
    //     .unwrap(); */
    //
    //     if let Ok(chunk_op) = rx.try_recv() {
    //
    //         println!("recieved chunk op!");
    //
    //         use chunks::ChunkRenderOp;
    //         match chunk_op {
    //             ChunkRenderOp::Load(chunk) => {
    //                 println!("chunk pos x {}", &chunk.pos.0);
    //                 loaded_chunks.insert(chunk.pos(), chunk);
    //             }
    //             ChunkRenderOp::Unload(coords) => {
    //                 loaded_chunks.remove(&coords);
    //             }
    //         }
    //     }
    //
    //     let mut per_instance = {
    //
    //         let mut voxels = Vec::new();
    //         for chunk in loaded_chunks.values() {
    //             // println!("rendering chunk pos x {}", &chunk.pos.0);
    //             let mut has_said = false;
    //             for &voxel in chunk.voxels() {
    //                 if voxel.chunk.0 != 0.0 && !has_said {
    //                     has_said = true;
    //                     println!("voxel at nonzero chunk: {:?}", &voxel);
    //                 }
    //                 voxels.push(voxel);
    //             }
    //         }
    //
    //         glium::VertexBuffer::dynamic(&display, &voxels).unwrap()
    //
    //     };
    //
    //     use glium::Surface;
    //     target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
    //
    //     target
    //         .draw(
    //             (&model, per_instance.per_instance().unwrap()),
    //             &indices,
    //             &shader,
    //             &uniforms,
    //             &params,
    //         )
    //         .unwrap();
    //
    //     target.finish().unwrap();
    //
    //     *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    //
    //     #[allow(clippy::single_match)]
    //     match ev {
    //         glutin::event::Event::WindowEvent { event, .. } => match event {
    //             glutin::event::WindowEvent::CloseRequested => {
    //                 *control_flow = glutin::event_loop::ControlFlow::Exit;
    //                 return;
    //             }
    //             _ => return,
    //         },
    //         _ => (),
    //     }
    // });
    */

    let (vert_source, frag_source) = loader::get_shader_src("resources/shaders").unwrap();

    let (mut renderer, mut controller) = render::ChunkRenderer::make(&vert_source, &frag_source);

    // Controller thread
    std::thread::spawn(move || {
        use nalgebra_glm::{
            Vec3,
            Mat4
        };

        let mut voxels: Vec<Vec<Vec<Option<Voxel>>>> =
            vec![vec![vec![None; 128]; 128]; 128];

        for _ in 0..10_000 {
            let pos = (
                    (rand::random::<f32>() * 128.0) as usize,
                    (rand::random::<f32>() * 128.0) as usize,
                    (rand::random::<f32>() * 128.0) as usize,
            );

            voxels[pos.0][pos.1][pos.2] = Some(chunks::Voxel::new_anon(
                (
                        pos.0 as f32,
                        pos.1 as f32,
                        pos.2 as f32
                    )
            ));
        }


        // std::thread::sleep(std::time::Duration::from_secs(1));
        let mut chunk = chunks::Chunk::new(
            (0.0, 0.0, 0.0),
            voxels.clone() // .clone()
        );

        controller.load_chunk(chunk);

        let mut chunk = chunks::Chunk::new(
            (1.0, 0.0, 0.0),
            voxels.clone() // .clone()
        );

        controller.load_chunk(chunk);

        let mut chunk = chunks::Chunk::new(
            (1.0, 1.0, 0.0),
            voxels.clone() // .clone()
        );

        controller.load_chunk(chunk);

        use std::time::Instant;
        let then = Instant::now();

        let mut pos = Vec3::new(0.0, 0.0, 0.0);

        let mut right: f32 = 0.0;
        let mut fwd: f32 = 0.0;
        let mut up: f32 = 0.0;

        let mut pitch: f32 = 0.0;
        let mut yaw: f32 = 0.0;

        loop {
            let now = Instant::now().duration_since(then).as_secs_f32();

            let x = -yaw.sin() * pitch.cos();
            let y = pitch.sin();
            let z = -yaw.cos() * pitch.cos();

            let pointing = Vec3::new(x, y, z).normalize();

            let pointing_right = Vec3::new(
                -yaw.cos(),
                0.0,
                yaw.sin()
            ).normalize();

            if let Some(mouse_delta) = controller.mouse_delta() {

                // todo: Use f32.clamp() instead of this nonsense
                let pitch_d = pitch + (-(mouse_delta.1)/360.0) as f32;
                if pitch_d > FRAC_PI_2 {
                    pitch = FRAC_PI_2 - 0.0001;
                } else if pitch_d < -FRAC_PI_2 {
                    pitch = -FRAC_PI_2 + 0.0001;
                } else {
                    pitch = pitch_d;
                }

                yaw = yaw + (mouse_delta.0/360.0) as f32;
            }

            if let Some(event) = controller.key_event() {
                match event {
                    render::KeyEvent::KeyDown(code) => {
                        match code {
                            W => {
                                fwd += 1.0;
                            },
                            A => {
                                right += -1.0;
                            },
                            S => {
                                fwd += -1.0;
                            },
                            D => {
                                right += 1.0;
                            },
                            SPACE => {
                                up += 1.0;
                            },
                            LSHIFT => {
                                up += -1.0;
                            }
                            _ => (),
                        }
                    },
                    render::KeyEvent::KeyUp(code) => {
                        match code {
                            W => {
                                fwd -= 1.0;
                            },
                            A => {
                                right -= -1.0;
                            },
                            S => {
                                fwd -= -1.0;
                            },
                            D => {
                                right -= 1.0;
                            },
                            SPACE => {
                                up -= 1.0;
                            },
                            LSHIFT => {
                                up -= -1.0;
                            }
                            _ => ()
                        }
                    }
                }
                fwd = fwd.clamp(-1.0, 1.0);
                right = right.clamp(-1.0, 1.0);
                up = up.clamp(-1.0, 1.0);
            }

            pos += ((pointing * fwd) + (pointing_right * right) + (Vec3::new(0.0, 1.0, 0.0) * up)) * 0.1;

            controller.set_camera(&pos, &pointing);
            std::thread::sleep(Duration::from_nanos(100));
        }

    });

    renderer.run(144.0);
}
