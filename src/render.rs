use crate::chunks::*;
use std::sync::mpsc::{Receiver, Sender, SyncSender, TrySendError};
use glium::{Display, Program, glutin::event_loop::EventLoop, DrawParameters, Surface};
use glium::vertex::VertexBufferAny;
use glium::glutin;
use hashbrown::HashMap;
use nalgebra_glm::{
    Mat4,
    Vec3
};

pub enum KeyEvent {
    KeyDown(u32),
    KeyUp(u32)
}

pub struct ChunkRenderer {
    loaded_chunks: HashMap<ChunkId, Chunk>,
    event_loop: EventLoop<()>,
    display: Display,
    shaders: Program,
    draw_parameters: DrawParameters<'static>,
    model_buffer: VertexBufferAny,
    chunk_rx: Receiver<ChunkRenderOp>,
    view_rx: Receiver<Mat4>,
    mouse_tx: SyncSender<(f64, f64)>,
    keyboard_tx: SyncSender<KeyEvent>
}

impl ChunkRenderer {
    pub fn make(vertex_source: &str, fragment_source: &str) -> (Self, RenderController) {
        use std::sync::mpsc;

        use glium::glutin;

        let (chunk_tx, chunk_rx) = mpsc::channel::<ChunkRenderOp>();
        let (view_tx, view_rx) = mpsc::channel::<Mat4>();

        // These channels will send input from mouse and keyboard to the controller thread
        let (mouse_tx, mouse_rx) = mpsc::sync_channel::<(f64, f64)>(32);
        let (keyboard_tx, keyboard_rx) = mpsc::sync_channel::<KeyEvent>(32);

        // Set up all our stuff here.
        let mut event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new().with_title("wowza!");
        let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        // Compile shaders.
        let shaders = glium::Program::from_source(
            &display,
            vertex_source,
            fragment_source,
            None,
        ).unwrap();

        // Draw parameters
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
            ..Default::default()
        };

        let (model, _) = crate::loader::load_wavefront_file(&display, "resources/models/cube.obj");

        let renderer = Self {
            loaded_chunks: HashMap::new(),
            event_loop,
            display,
            shaders,
            draw_parameters: params,
            model_buffer: model,
            chunk_rx,
            view_rx,
            mouse_tx,
            keyboard_tx
        };

        (renderer, RenderController { chunk_tx, view_tx, mouse_rx, keyboard_rx })
    }

    pub fn run(mut self, fps: f64) {

        let mut voxelbuf_cache: HashMap<ChunkId, glium::VertexBuffer<Voxel>> = HashMap::new();

        let mut camera = Mat4::zeros();
        let model = Mat4::from([
            [0.8, 0.0, 0.0, 0.0],
            [0.0, 0.8, 0.0, 0.0],
            [0.0, 0.0, 0.8, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ]);


        use std::time::{Instant, Duration};

        // Hijack whatever thread this was called from (currently cannot be called in any other thread
        // than main) and turn that into the render thread. A controller thread should be started
        // prior to calling this function in order to actually be able to send data to be rendered.
        let starting_time = Instant::now();
        self.event_loop.run(move |ev, _, control_flow| {
            let next_frame_time =
                std::time::Instant::now() + std::time::Duration::from_secs_f64(1.0 / fps);

            let t = Instant::now().duration_since(starting_time).as_secs_f32();

            if let Ok(cam_matrix) = self.view_rx.try_recv() {
                // println!("cam set {}", &cam_matrix);
                camera = cam_matrix;
            }
            if let Ok(op) = self.chunk_rx.try_recv() {
                println!("got chunk op");
                match op {
                    ChunkRenderOp::Load(chunk) => {
                        self.loaded_chunks.insert(chunk.id(), chunk);
                    },
                    ChunkRenderOp::Unload(id) => {
                        self.loaded_chunks.remove(&id);
                    }
                    ChunkRenderOp::Mutate(_) => unimplemented!()
                }
            }

            *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

            #[allow(clippy::single_match)]
            match ev {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    },
                    _ => return,
                },
                glutin::event::Event::DeviceEvent { event, .. } => match event {
                    glutin::event::DeviceEvent::MouseMotion { delta } => {
                        let result = self.mouse_tx.try_send(delta);
                        match result {
                            Err(error) => {
                                match error {
                                    TrySendError::Full(_) => (),
                                    _ => panic!()
                                }
                            }
                            _ => ()
                        }
                    },
                    glutin::event::DeviceEvent::Key(kb) => {
                        let key = match kb.state {
                            glutin::event::ElementState::Pressed => KeyEvent::KeyDown(kb.scancode),
                            glutin::event::ElementState::Released => KeyEvent::KeyUp(kb.scancode)
                        };
                        let result = self.keyboard_tx.try_send(key);
                        match result {
                            Err(error) => {
                                match error {
                                    TrySendError::Full(_) => (),
                                    _ => panic!()
                                }
                            }
                            _ => ()
                        }
                    }
                    _ => (),
                },
                glutin::event::Event::MainEventsCleared => {
                    let mut target = self.display.draw();

                    // todo: use nalgebra_glm's perspective matrix instead.
                    let perspective = {
                        let (w, h) = target.get_dimensions();

                        let fov: f32 = nalgebra_glm::pi::<f32>() / 3.0;
                        let f = 1.0 / (fov / 2.0).tan();

                        let aspect_ratio = h as f32 / w as f32;

                        let zfar = 1024.0;
                        let znear = 0.1;

                        Mat4::from([
                            [f * aspect_ratio, 0.0, 0.0, 0.0],
                            [0.0, f, 0.0, 0.0],
                            [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
                            [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
                        ])
                    };

                    let camera_r: [[f32; 4]; 4] = camera.into();
                    let perspective_r: [[f32; 4]; 4] = perspective.into();
                    let model_r: [[f32; 4]; 4] = model.into();

                    let uniforms = uniform! {
                        t: t,
                        camera: camera_r,
                        perspective: perspective_r,
                        model: model_r,
                    };

                    target.clear_color_and_depth((0.45, 0.5, 1.0, 1.0), 1.0);

                    for (id, chunk) in self.loaded_chunks.iter() {

                        // Cache just follows whatever loaded_chunks says. If the chunk is not loaded remove
                        // it from the cache as well.
                        if voxelbuf_cache.contains_key(id) && !self.loaded_chunks.contains_key(id) {
                            voxelbuf_cache.remove(id);
                            continue;
                        }

                        // Construct new buffer and cache it or get cached buffer.
                        let mut voxel_buffer = if !voxelbuf_cache.contains_key(id) {
                            let flattened_voxels = chunk.voxels
                                .iter()
                                .flatten()
                                .flatten().copied()
                                .flatten()
                                .collect::<Vec<Voxel>>();


                            let _buf = glium::VertexBuffer::dynamic(
                                &self.display,
                                &flattened_voxels
                            ).unwrap();

                            // _buf[0] = Voxel::new((0.0, 0.0, 0.0), chunk.pos);
                            // let lol = _buf.read();

                            voxelbuf_cache.insert(*id, _buf);
                            voxelbuf_cache.get(id).unwrap()

                        } else {
                            voxelbuf_cache.get(id).unwrap()
                        };

                        // 1 chunk = 1 draw call
                        // todo: investigate if further instancing of voxels is 1. practical and 2. grants
                        //  any significant performance improvement.
                        target.draw(
                            (
                                &self.model_buffer,
                                voxel_buffer.per_instance().unwrap(),
                            ),
                            &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                            &self.shaders,
                            &uniforms,
                            &self.draw_parameters
                        ).unwrap();
                    }

                    target.finish().unwrap();
                },
                _ => ()
            }
        })
    }
}

pub struct RenderController {
    chunk_tx: Sender<ChunkRenderOp>,
    view_tx: Sender<Mat4>,
    mouse_rx: Receiver<(f64, f64)>,
    keyboard_rx: Receiver<KeyEvent>
}

impl RenderController {
    pub fn load_chunk(&self, chunk: Chunk) {
        self.chunk_tx.send(ChunkRenderOp::Load(chunk)).unwrap();
    }

    pub fn unload_chunk(&self, chunk_id: ChunkId) {
        self.chunk_tx.send(ChunkRenderOp::Unload(chunk_id)).unwrap();
    }

    pub fn set_camera(&self, pos: &Vec3, dir: &Vec3) {
        let cam_matrix = crate::util::view_matrix(
            pos, dir, &Vec3::new(0.0, 1.0, 0.0)
        );
        self.view_tx.send(cam_matrix).unwrap();
    }

    pub fn mouse_delta(&self) -> Option<(f64, f64)> {
        if let Ok(input) = self.mouse_rx.try_recv() {
            return Some(input)
        }
        None
    }

    pub fn key_event(&self) -> Option<KeyEvent> {
        if let Ok(event) = self.keyboard_rx.try_recv() {
            return Some(event)
        }
        None
    }
}
