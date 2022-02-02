use std::fmt;
use std::fmt::Formatter;
use std::sync::Arc;
use std::time::Instant;
use nalgebra_glm::{Mat4, Vec3};
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer, CpuBufferPool, TypedBufferAccess};
use vulkano::descriptor_set::PersistentDescriptorSet;
use vulkano::device::{Device, Queue};
use vulkano::pipeline::{GraphicsPipeline, Pipeline, PipelineBindPoint};
use vulkano::render_pass::{Framebuffer, RenderPass, Subpass};
use vulkano::swapchain::{AcquireError, Surface, Swapchain, SwapchainCreationError};
use vulkano::{swapchain, sync, Version};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, SubpassContents};
use vulkano::format::Format;
use vulkano::device::DeviceExtensions;
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType, QueueFamily};
use vulkano::image::{AttachmentImage, ImageAccess, ImageUsage, SwapchainImage};
use vulkano::image::view::ImageView;
use vulkano::instance::{Instance, InstanceExtensions};
use vulkano::pipeline::graphics::depth_stencil::DepthStencilState;
use vulkano::pipeline::graphics::input_assembly::{InputAssemblyState, PrimitiveTopology};
use vulkano::pipeline::graphics::rasterization::{CullMode, RasterizationState};
use vulkano::pipeline::graphics::vertex_input::BuffersDefinition;
use vulkano::pipeline::graphics::viewport::{Viewport, ViewportState};
use vulkano::shader::ShaderModule;
use vulkano::sync::{FlushError, GpuFuture};
use vulkano_win::VkSurfaceBuild;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

fn get_good_gpu<'a, I: Iterator<Item=PhysicalDevice<'a>>>(devices: I) -> Option<PhysicalDevice<'a>> {
    devices.max_by_key(|device| match device.properties().device_type {
        PhysicalDeviceType::DiscreteGpu => 5,
        PhysicalDeviceType::IntegratedGpu => 4,
        PhysicalDeviceType::VirtualGpu => 3,
        PhysicalDeviceType::Cpu => 2,
        PhysicalDeviceType::Other => 1,
    })
}

struct VulkanParams {
    instance_extensions: InstanceExtensions,
    device_extensions: DeviceExtensions,
    api_version: Version,
}

impl Default for VulkanParams {
    fn default() -> Self {
        Self {
            instance_extensions: vulkano_win::required_extensions(),
            device_extensions: DeviceExtensions {
                khr_swapchain: true,
                ..DeviceExtensions::none()
            },
            api_version: Version::V1_2,
        }
    }
}

pub(crate) struct Vulkan {
    event_loop: EventLoop<()>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain<Window>>,
    pipeline: Arc<GraphicsPipeline>,
    framebuffers: Vec<Arc<Framebuffer>>,
    render_pass: Arc<RenderPass>,
    surface: Arc<Surface<Window>>,
    vertex_buffer: Arc<CpuAccessibleBuffer<[ModelVertex]>>,
    index_buffer: Arc<CpuAccessibleBuffer<[u16]>>,
    uniform_buffer: CpuBufferPool<vertex_shader::ty::Data>,
    vshader: Arc<ShaderModule>,
    fshader: Arc<ShaderModule>
}

impl Vulkan {
    /// Create a wrapper instance.
    pub(crate) fn setup() -> Self {
        let default_params = VulkanParams::default();
        let event_loop = EventLoop::new();

        let instance = Instance::new(
            None,
            default_params.api_version,
            &default_params.instance_extensions,
            None
        ).unwrap();

        let physical_device =
            get_good_gpu(PhysicalDevice::enumerate(&instance))
                .expect("couldn't find a usable device");

        let queue_family: QueueFamily =
            physical_device.queue_families().next().unwrap();

        let (device, mut queues) = Device::new(physical_device,
                                               physical_device.supported_features(),
                                               &physical_device.required_extensions()
                                                   .union(&default_params.device_extensions),
                                               [(queue_family, 0.5)])
            .expect("failed to create device instance");

        let queue = queues.next().expect("no queues");

        let surface = WindowBuilder::new().with_title("vkwrapper")
            .build_vk_surface(&event_loop, instance.clone())
            .unwrap();

        let dims: [u32; 2] = surface.window().inner_size().into();

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

        let (vertices, indices) =
            from_wavefront("resources/models/cube.obj");

        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::vertex_buffer(),
            false,
            vertices.iter().cloned()
        ).unwrap();

        let index_buffer = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::index_buffer(),
            false,
            indices.iter().cloned()
        ).unwrap();

        let uniform_buffer = CpuBufferPool::<vertex_shader::ty::Data>::new(
            device.clone(),
            BufferUsage::uniform_buffer()
        );

        let vshader = vertex_shader::load(device.clone()).unwrap();
        let fshader = fragment_shader::load(device.clone()).unwrap();

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

        Self {
            event_loop,
            device,
            queue,
            swapchain,
            pipeline,
            framebuffers,
            render_pass,
            surface,
            vertex_buffer,
            index_buffer,
            uniform_buffer,
            vshader,
            fshader
        }
    }


    pub(crate) fn run(mut self) {

        let mut recreate_swapchain = false;
        let mut previous_frame_end = Some(sync::now(self.device.clone()).boxed());
        let mut dimensions = self.swapchain.surface().window().inner_size().into();
        let rotation_start = Instant::now();

        self.event_loop.run(move |event, _, control_flow| {
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
                        dimensions = self.surface.window().inner_size().into();
                        let (new_swapchain, new_images) =
                            match self.swapchain.recreate().dimensions(dimensions).build() {
                                Ok(r) => r,
                                Err(SwapchainCreationError::UnsupportedDimensions) => return,
                                Err(e) => panic!("failed to recreate swapchain: {:?}", e),
                            };

                        self.swapchain = new_swapchain;
                        let (new_pipeline, new_framebuffers, dimensions) = window_size_dependent_setup(
                            self.device.clone(),
                            &self.vshader,
                            &self.fshader,
                            &new_images,
                            self.render_pass.clone()
                        );

                        self.pipeline = new_pipeline;
                        self.framebuffers = new_framebuffers;
                        recreate_swapchain = false;
                    }

                    let uniform_buffer_subbuffer = {
                        let elapsed = rotation_start.elapsed();

                        let rotation = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1_000_000_000.0;

                        let rotation_1 = nalgebra_glm::rotation(rotation as f32, &Vec3::new(0.0, 1.0, 0.0));

                        let rotation_2 = nalgebra_glm::rotation((rotation as f32).cos() / 2.0, &Vec3::new(0.0, 0.0, 1.0));

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

                        self.uniform_buffer.next(uniform_data).unwrap()
                    };

                    let layout = self.pipeline.layout().descriptor_set_layouts().get(0).unwrap();
                    let mut set_builder = PersistentDescriptorSet::start(
                        layout.clone(),
                    );

                    set_builder.add_buffer(uniform_buffer_subbuffer.clone()).unwrap();
                    let set = set_builder.build().unwrap();

                    let (image_num, suboptimal, acquire_future) =
                        match swapchain::acquire_next_image(self.swapchain.clone(), None) {
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
                        self.device.clone(),
                        self.queue.family(),
                        CommandBufferUsage::OneTimeSubmit,
                    ).unwrap();

                    builder.begin_render_pass(
                        self.framebuffers[image_num].clone(),
                        SubpassContents::Inline,
                        vec![[0.0, 0.0, 1.0, 1.0].into(), 1.0.into()],
                    )
                        .unwrap()
                        .bind_pipeline_graphics(self.pipeline.clone())
                        .bind_descriptor_sets(
                            PipelineBindPoint::Graphics,
                            self.pipeline.layout().clone(),
                            0,
                            vec![set]
                        )
                        .bind_vertex_buffers(0, self.vertex_buffer.clone()) //normal_buffer.clone()))
                        //.bind_index_buffer(index_buffer.clone())
                        //.draw_indexed(index_buffer.len() as u32, 1, 0, 0, 0)
                        .draw(self.vertex_buffer.len() as u32, 1, 0, 0)
                        .unwrap()
                        .end_render_pass()
                        .unwrap();

                    let command_buffer = builder.build().unwrap();

                    let future = previous_frame_end
                        .take()
                        .unwrap()
                        .join(acquire_future)
                        .then_execute(self.queue.clone(), command_buffer)
                        .unwrap()
                        .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), image_num)
                        .then_signal_fence_and_flush();

                    match future {
                        Ok(future) => {
                            previous_frame_end = Some(future.boxed());
                        },
                        Err(FlushError::OutOfDate) => {
                            recreate_swapchain = true;
                            previous_frame_end = Some(sync::now(self.device.clone()).boxed());
                        },
                        Err(e) => {
                            println!("Failed to flush future: {:?}", e);
                            previous_frame_end = Some(sync::now(self.device.clone()).boxed());
                        }
                    }
                },
                _ => (),
            }
        })
    }

    /// Wrapper debug information.
    fn debugstr(&self) -> String {
        format!(
            "--- Vulkan Wrapper ---
            \nDevice: {:?}
            \nQueue: {:?}
            \nSwapchain: {:?}
            \nPipeline: {:?}
            \nFramebuffers: {}",
        self.device, self.queue, self.swapchain, self.pipeline, self.framebuffers.len()
        )
    }
}

impl fmt::Display for Vulkan {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.debugstr())
    }
}

impl fmt::Debug for Vulkan {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.debugstr())
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub(crate) struct ModelVertex {
    position: (f32, f32, f32),
    normal: (f32, f32, f32)
}

vulkano::impl_vertex!(ModelVertex, position, normal);

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
                .vertex::<ModelVertex>()
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

pub(crate) fn from_wavefront(path: &str) -> (Vec<ModelVertex>, Vec<u16>) {
    let raw_bytes = std::fs::read(path).expect("could not read file");
    let mut data = std::io::BufReader::new(&raw_bytes[..]);
    let data = obj::ObjData::load_buf(&mut data).unwrap();

    let mut vertex_data: Vec<ModelVertex> = Vec::new();
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

                        let vertex = ModelVertex {
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