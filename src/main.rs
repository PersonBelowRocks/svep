mod teapot;
mod cube;
mod vkwrapper;

use std::sync::Arc;
use std::time::{Duration, Instant};
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer, CpuBufferPool, TypedBufferAccess};
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType, QueueFamily};
use vulkano::device::{Device, DeviceExtensions, Features, Queue};
use vulkano::image::{AttachmentImage, ImageUsage, SwapchainImage};
use vulkano::instance::{Instance, InstanceExtensions};
use vulkano::swapchain::{AcquireError, Swapchain, SwapchainCreationError};
use vulkano::sync::{FlushError, GpuFuture};
use vulkano::{swapchain, Version};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use vulkano_win::VkSurfaceBuild;
use vulkano::format::Format;
use vulkano::image::view::ImageView;
use vulkano::pipeline::graphics::depth_stencil::DepthStencilState;
use vulkano::pipeline::graphics::input_assembly::{InputAssemblyState, PrimitiveTopology};
use vulkano::pipeline::graphics::vertex_input::BuffersDefinition;
use vulkano::pipeline::graphics::viewport::{Viewport, ViewportState};
use vulkano::pipeline::{GraphicsPipeline, Pipeline, PipelineBindPoint};
use vulkano::render_pass::{Framebuffer, RenderPass, Subpass};
use vulkano::shader::ShaderModule;
use vulkano::image::traits::ImageAccess;
use vulkano::sync;
use winit::event::{Event, WindowEvent};
use nalgebra_glm::{Mat4, Mat3, Vec3, Vec4};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, SubpassContents};
use vulkano::descriptor_set::PersistentDescriptorSet;
use vulkano::descriptor_set::DescriptorSetsCollection;
use vulkano::descriptor_set::persistent::PersistentDescriptorSetBuilder;
use vulkano::pipeline::graphics::input_assembly::PrimitiveTopology::TriangleList;
use vulkano::pipeline::graphics::rasterization::{CullMode, PolygonMode, RasterizationState};

// mod chunks;
// mod loader;
// mod render;
// mod teapot;
// mod util;

// Scan codes
const W: u32 = 0x11;
const A: u32 = 0x1e;
const S: u32 = 0x1f;
const D: u32 = 0x20;
const SPACE: u32 = 0x39;
const LSHIFT: u32 = 0x2a;

// #[derive(Copy, Clone)]
// struct Vertex {
//     position: [f32; 2],
// }
//
// implement_vertex!(Vertex, position);

// Temporary main function for testing
fn main() {
    let required_extensions = vulkano_win::required_extensions();
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::none()
    };

    let instance = Instance::new(None, Version::V1_2, &required_extensions, None)
        .expect("failed to create instance");

    println!("Vulkan instance: {:?}", instance);

    println!("Automatically selecting GPU...");
    let all_phys_devices = PhysicalDevice::enumerate(&instance);

    let physical_device: PhysicalDevice = all_phys_devices
        .max_by_key(|device| match device.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 5,
            PhysicalDeviceType::IntegratedGpu => 4,
            PhysicalDeviceType::VirtualGpu => 3,
            PhysicalDeviceType::Cpu => 2,
            PhysicalDeviceType::Other => 1,
        })
        .expect("could not find a usable device");

    println!(
        "Selected device with name {} and type {:?}",
        &physical_device.properties().device_name,
        &physical_device.properties().device_type
    );

    let queue_family: QueueFamily =
        physical_device.queue_families().next().unwrap();

    let (device, mut queues) = Device::new(physical_device,
                                           physical_device.supported_features(),
                             &physical_device.required_extensions()
                                               .union(&device_extensions),
                             [(queue_family, 0.5)])
        .expect("failed to create device instance");

    println!("Created device instance '{:?}'", device);

    let queues = queues.collect::<Vec<_>>();

    println!("Found following queues:");
    for q in queues.iter() {
        println!("  - {:?}", q);
    }
    println!("\n");

    let queue = queues.into_iter().next().expect("no queues");

    println!("Selected first queue: {:?}", queue);

    // Vulkan setup is complete
    // We're now moving onto winit/window setup
    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new().with_title("Hello Vulkan!")
        // This was really confusing to set up, for our version of vulkan-winit we need winit v0.25.
        // The trait declaring this method is not implemented for other versions of winit.
        .build_vk_surface(&event_loop, instance.clone())
        .unwrap();

    let mut dimensions: [u32; 2] = surface.window().inner_size().into();
    println!("Window dimensions: {}, {}", dimensions[0], dimensions[1]);

    // Build swapchain and swapchain images
    // todo: do more research on what exactly is going on here, and what a swapchain is / how it works
    let (mut swapchain, images) = {
        let caps = surface.capabilities(physical_device).unwrap();
        let format = caps.supported_formats[0].0;
        let composite_alpha = caps.supported_composite_alpha.iter().next().unwrap();
        let dimensions: [u32; 2] = surface.window().inner_size().into();

        Swapchain::start(device.clone(), surface.clone())
            .num_images(caps.min_image_count)
            .format(format)
            .dimensions(dimensions)
            .usage(ImageUsage::color_attachment())
            .sharing_mode(&queue)
            .composite_alpha(composite_alpha)
            .build()
            .unwrap()
    };

    // Load voxel cube model
    // let (vertices, normals, indices) = from_wavefront("resources/models/cube.obj");
    // let vertices = cube::VERTICES;
    // let normals = cube::NORMALS;
    // let indices = cube::INDICES;

    let (vertices, indices) = from_wavefront("resources/models/cube.obj");

    let vertex_buffer = CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage::vertex_buffer(),
        false,
        vertices.iter().cloned()
    ).unwrap();

    // let normal_buffer = CpuAccessibleBuffer::from_iter(
    //     device.clone(),
    //     BufferUsage::vertex_buffer(),
    //     false,
    //     normals.iter().cloned()
    // ).unwrap();

    let index_buffer = CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage::index_buffer(),
        false,
        indices.iter().cloned()
    ).unwrap();

    let uniform_buffer = CpuBufferPool::<vertex_shader::ty::Data>::new(
        device.clone(),
        BufferUsage::all()
    );

    let vshader = vertex_shader::load(device.clone()).unwrap();
    let fshader = fragment_shader::load(device.clone()).unwrap();

    println!("Vertex shader: {:?}", vshader);
    println!("Fragment shader: {:?}", fshader);

    // todo: research render passes
    let render_pass = vulkano::single_pass_renderpass!(device.clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: swapchain.format(),
                samples: 1,
            },
            depth: {
                load: Clear,
                store: DontCare,
                format: Format::D16_UNORM,
                samples: 1,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {depth}
        }
    ).unwrap();

    let (mut pipeline, mut framebuffers, mut dimensions) =
        window_size_dependent_setup(device.clone(), &vshader, &fshader, &images, render_pass.clone());
    let mut recreate_swapchain = false;

    let mut previous_frame_end = Some(sync::now(device.clone()).boxed());
    let rotation_start = Instant::now();

    // Begin event loop
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent { event: WindowEvent::Resized(_), .. } => {
                recreate_swapchain = true;
            }
            // todo: change this to MainEventsCleared and see what happens
            Event::RedrawEventsCleared => {
                previous_frame_end.as_mut().unwrap().cleanup_finished();

                if recreate_swapchain {
                    dimensions = surface.window().inner_size().into();
                    let (new_swapchain, new_images) =
                        match swapchain.recreate().dimensions(dimensions).build() {
                            Ok(r) => r,
                            Err(SwapchainCreationError::UnsupportedDimensions) => return,
                            Err(e) => panic!("failed to recreate swapchain: {:?}", e),
                        };

                    swapchain = new_swapchain;
                    let (new_pipeline, new_framebuffers, dimensions) = window_size_dependent_setup(
                        device.clone(),
                        &vshader,
                        &fshader,
                        &new_images,
                        render_pass.clone()
                    );

                    pipeline = new_pipeline;
                    framebuffers = new_framebuffers;
                    recreate_swapchain = false;
                }

                let uniform_buffer_subbuffer = {
                    let elapsed = rotation_start.elapsed();

                    let rotation = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1_000_000_000.0;

                    let rotation_1 = nalgebra_glm::rotation(rotation as f32, &Vec3::new(0.0, 1.0, 0.0));

                    let rotation_2 = nalgebra_glm::rotation((rotation as f32).cos()/2.0, &Vec3::new(0.0, 0.0, 1.0));

                    let aspect_ratio = dimensions[0] as f32 / dimensions[1] as f32;

                    let proj = Mat4::new_perspective(
                        aspect_ratio,
                        std::f32::consts::FRAC_PI_2,
                        0.01,
                        100.0
                    );

                    let view = nalgebra_glm::Mat4::look_at_rh(
                        &Vec3::new(0.3, 0.3, 1.0).into(),
                        &Vec3::new(0.0, 0.0, 0.0).into(),
                        &Vec3::new(0.0, -1.0, 0.0)
                    );

                    let scale = Mat4::new_scaling(0.8);

                    let rotation = rotation_1 * rotation_2;
                    let uniform_data = vertex_shader::ty::Data {
                        world: Mat4::from(rotation).into(),
                        view: (view * scale).into(),
                        proj: proj.into(),
                    };

                    uniform_buffer.next(uniform_data).unwrap()
                };

                let layout = pipeline.layout().descriptor_set_layouts().get(0).unwrap();
                let mut set_builder = PersistentDescriptorSet::start(
                    layout.clone(),
                );

                set_builder.add_buffer(uniform_buffer_subbuffer.clone()).unwrap();
                let set = set_builder.build().unwrap();

                let (image_num, suboptimal, acquire_future) =
                    match swapchain::acquire_next_image(swapchain.clone(), None) {
                        Ok(r) => r,
                        Err(AcquireError::OutOfDate) => {
                            recreate_swapchain = true;
                            return;
                        },
                        Err(e) => panic!("Failed to acquire next image: {:?}", e),
                    };

                if suboptimal {
                    recreate_swapchain = true;
                }

                let mut builder = AutoCommandBufferBuilder::primary(
                    device.clone(),
                    queue.family(),
                    CommandBufferUsage::OneTimeSubmit,
                ).unwrap();

                builder.begin_render_pass(
                    framebuffers[image_num].clone(),
                    SubpassContents::Inline,
                    vec![[0.0, 0.0, 1.0, 1.0].into(), 1.0.into()],
                )
                    .unwrap()
                    .bind_pipeline_graphics(pipeline.clone())
                    .bind_descriptor_sets(
                        PipelineBindPoint::Graphics,
                        pipeline.layout().clone(),
                        0,
                        vec![set]
                    )
                    .bind_vertex_buffers(0, vertex_buffer.clone()) //normal_buffer.clone()))
                    //.bind_index_buffer(index_buffer.clone())
                    //.draw_indexed(index_buffer.len() as u32, 1, 0, 0, 0)
                    .draw(vertex_buffer.len() as u32, 1, 0, 0)
                    .unwrap()
                    .end_render_pass()
                    .unwrap();

                let command_buffer = builder.build().unwrap();

                let future = previous_frame_end
                    .take()
                    .unwrap()
                    .join(acquire_future)
                    .then_execute(queue.clone(), command_buffer)
                    .unwrap()
                    .then_swapchain_present(queue.clone(), swapchain.clone(), image_num)
                    .then_signal_fence_and_flush();

                match future {
                    Ok(future) => {
                        previous_frame_end = Some(future.boxed());
                    },
                    Err(FlushError::OutOfDate) => {
                        recreate_swapchain = true;
                        previous_frame_end = Some(sync::now(device.clone()).boxed());
                    },
                    Err(e) => {
                        println!("Failed to flush future: {:?}", e);
                        previous_frame_end = Some(sync::now(device.clone()).boxed());
                    }
                }
            },
            _ => (),
        }
    })
}

#[derive(Default, Copy, Clone, Debug)]
pub(crate) struct ModelVertex {
    position: (f32, f32, f32),
}

vulkano::impl_vertex!(ModelVertex, position);

#[derive(Default, Copy, Clone, Debug)]
pub(crate) struct ModelVertex2 {
    position: (f32, f32, f32),
    normal: (f32, f32, f32)
}

vulkano::impl_vertex!(ModelVertex2, position, normal);

#[derive(Default, Copy, Clone, Debug)]
pub(crate) struct ModelNormal {
    normal: (f32, f32, f32)
}

vulkano::impl_vertex!(ModelNormal, normal);

mod vertex_shader {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "resources/shaders/vulkan-vs-1.glsl"
    }
}

mod fragment_shader {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "resources/shaders/vulkan-fs-1.glsl"
    }
}

pub(crate) fn from_wavefront(path: &str) -> (Vec<ModelVertex2>, /* Vec<ModelNormal>, */ Vec<u16>) {
    let raw_bytes = std::fs::read(path).expect("could not read file");
    let mut data = std::io::BufReader::new(&raw_bytes[..]);
    let data = obj::ObjData::load_buf(&mut data).unwrap();

    let mut vertex_data: Vec<ModelVertex2> = Vec::new();
    // let mut normal_data: Vec<ModelNormal> = Vec::new();
    let mut index_data: Vec<u16> = Vec::new();

    for object in data.objects.iter() {
        for polygon in object.groups.iter().flat_map(|g| g.polys.iter()) {
            match polygon {
                obj::SimplePolygon(indices) => {
                    for v in indices.iter() {
                        let position = data.position[v.0];
                        let normal = v.2
                            .map(|index| data.normal[index])
                            .unwrap_or([0.0, 0.0, 0.0]);

                        let vertex = ModelVertex2 {
                            position: (position[0], position[1], position[2]),
                            normal: (normal[0], normal[1], normal[2])
                        };

                        println!("VERTEX: {:?}", &vertex);

                        vertex_data.push(vertex);

                        // normal_data.push(ModelNormal {
                        //     normal: (normal[0], normal[1], normal[2]),
                        // });

                        index_data.push(v.0 as u16);
                    }
                }
            }
        }
    }
    println!("{:?}", index_data);
    (
        vertex_data,
        // normal_data,
        index_data
    )
}

// todo: research how this entire thing works, it is just copied from the vulkano examples
fn window_size_dependent_setup(
    device: Arc<Device>,
    vs: &ShaderModule,
    fs: &ShaderModule,
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<RenderPass>,
) -> (Arc<GraphicsPipeline>, Vec<Arc<Framebuffer>>, [u32; 2]) {
    let dimensions = images[0].dimensions().width_height();

    let depth_buffer = ImageView::new(
        AttachmentImage::transient(device.clone(), dimensions, Format::D16_UNORM).unwrap(),
    )
        .unwrap();

    let framebuffers = images
        .iter()
        .map(|image| {
            let view = ImageView::new(image.clone()).unwrap();
            Framebuffer::start(render_pass.clone())
                .add(view)
                .unwrap()
                .add(depth_buffer.clone())
                .unwrap()
                .build()
                .unwrap()
        })
        .collect::<Vec<_>>();

    // In the triangle example we use a dynamic viewport, as its a simple example.
    // However in the teapot example, we recreate the pipelines with a hardcoded viewport instead.
    // This allows the driver to optimize things, at the cost of slower window resizes.
    // https://computergraphics.stackexchange.com/questions/5742/vulkan-best-way-of-updating-pipeline-viewport
    let pipeline = GraphicsPipeline::start()
        .vertex_input_state(
            BuffersDefinition::new()
                .vertex::<ModelVertex2>()
                //.vertex::<cube::Normal>(),
        )
        .vertex_shader(vs.entry_point("main").unwrap(), ())
        .input_assembly_state(InputAssemblyState::new().topology(PrimitiveTopology::TriangleList))
        .rasterization_state(
            RasterizationState::new()//.polygon_mode(PolygonMode::Line)
                .cull_mode(CullMode::Front)
        )
        .viewport_state(ViewportState::viewport_fixed_scissor_irrelevant([
            Viewport {
                origin: [0.0, 0.0],
                dimensions: [dimensions[0] as f32, dimensions[1] as f32],
                depth_range: 0.0..1.0,
            },
        ]))
        .fragment_shader(fs.entry_point("main").unwrap(), ())
        .depth_stencil_state(DepthStencilState::simple_depth_test())
        .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
        .build(device.clone())
        .unwrap();

    (pipeline, framebuffers, dimensions)
}


/* Old main
fn _main() {
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
        use nalgebra_glm::{Mat4, Vec3};

        let mut voxels: Vec<Vec<Vec<Option<Voxel>>>> = vec![vec![vec![None; 128]; 128]; 128];

        for _ in 0..10_000 {
            let pos = (
                (rand::random::<f32>() * 128.0) as usize,
                (rand::random::<f32>() * 128.0) as usize,
                (rand::random::<f32>() * 128.0) as usize,
            );

            voxels[pos.0][pos.1][pos.2] = Some(chunks::Voxel::new_anon((
                pos.0 as f32,
                pos.1 as f32,
                pos.2 as f32,
            )));
        }

        // std::thread::sleep(std::time::Duration::from_secs(1));
        let mut chunk = chunks::Chunk::new(
            (0.0, 0.0, 0.0),
            voxels.clone(), // .clone()
        );

        controller.load_chunk(chunk);

        let mut chunk = chunks::Chunk::new(
            (1.0, 0.0, 0.0),
            voxels.clone(), // .clone()
        );

        controller.load_chunk(chunk);

        let mut chunk = chunks::Chunk::new(
            (1.0, 1.0, 0.0),
            voxels.clone(), // .clone()
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

            let pointing_right = Vec3::new(-yaw.cos(), 0.0, yaw.sin()).normalize();

            if let Some(mouse_delta) = controller.mouse_delta() {
                // todo: Use f32.clamp() instead of this nonsense
                let pitch_d = pitch + (-(mouse_delta.1) / 360.0) as f32;
                if pitch_d > FRAC_PI_2 {
                    pitch = FRAC_PI_2 - 0.0001;
                } else if pitch_d < -FRAC_PI_2 {
                    pitch = -FRAC_PI_2 + 0.0001;
                } else {
                    pitch = pitch_d;
                }

                yaw = yaw + (mouse_delta.0 / 360.0) as f32;
            }

            if let Some(event) = controller.key_event() {
                match event {
                    render::KeyEvent::KeyDown(code) => match code {
                        W => {
                            fwd += 1.0;
                        }
                        A => {
                            right += -1.0;
                        }
                        S => {
                            fwd += -1.0;
                        }
                        D => {
                            right += 1.0;
                        }
                        SPACE => {
                            up += 1.0;
                        }
                        LSHIFT => {
                            up += -1.0;
                        }
                        _ => (),
                    },
                    render::KeyEvent::KeyUp(code) => match code {
                        W => {
                            fwd -= 1.0;
                        }
                        A => {
                            right -= -1.0;
                        }
                        S => {
                            fwd -= -1.0;
                        }
                        D => {
                            right -= 1.0;
                        }
                        SPACE => {
                            up -= 1.0;
                        }
                        LSHIFT => {
                            up -= -1.0;
                        }
                        _ => (),
                    },
                }
                fwd = fwd.clamp(-1.0, 1.0);
                right = right.clamp(-1.0, 1.0);
                up = up.clamp(-1.0, 1.0);
            }

            pos += ((pointing * fwd) + (pointing_right * right) + (Vec3::new(0.0, 1.0, 0.0) * up))
                * 0.1;

            controller.set_camera(&pos, &pointing);
            std::thread::sleep(Duration::from_nanos(100));
        }
    });

    renderer.run(144.0);
}
*/
