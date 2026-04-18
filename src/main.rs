use crate::client::camera::Camera;
use crate::client::input::Input;
use crate::math::mat4::Mat4;
use crate::util::timer::{FrameRateLimit, Timer};
use log::{error, info};
use smallvec::smallvec;
use std::num::NonZero;
use std::sync::Arc;
use std::time::Instant;
use vulkano::buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents};
use vulkano::descriptor_set::allocator::{StandardDescriptorSetAllocator, StandardDescriptorSetAllocatorCreateInfo};
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
use vulkano::device::{Device, DeviceCreateInfo, DeviceExtensions, DeviceOwned, Queue, QueueCreateInfo, QueueFlags};
use vulkano::format::Format;
use vulkano::image::view::ImageView;
use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage};
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::pipeline::graphics::color_blend::{ColorBlendAttachmentState, ColorBlendState};
use vulkano::pipeline::graphics::depth_stencil::{DepthState, DepthStencilState};
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::multisample::MultisampleState;
use vulkano::pipeline::graphics::rasterization::RasterizationState;
use vulkano::pipeline::graphics::vertex_input::{Vertex, VertexDefinition};
use vulkano::pipeline::graphics::viewport::{Viewport, ViewportState};
use vulkano::pipeline::graphics::GraphicsPipelineCreateInfo;
use vulkano::pipeline::layout::PipelineDescriptorSetLayoutCreateInfo;
use vulkano::pipeline::{GraphicsPipeline, Pipeline, PipelineBindPoint, PipelineLayout, PipelineShaderStageCreateInfo};
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass};
use vulkano::shader::{EntryPoint, ShaderModule};
use vulkano::swapchain::{PresentMode, Surface, Swapchain, SwapchainAcquireFuture, SwapchainCreateInfo, SwapchainPresentInfo};
use vulkano::sync::GpuFuture;
use vulkano::{swapchain, sync, Validated, VulkanError, VulkanLibrary};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

mod util;
mod math;
mod client;

#[derive(BufferContents, Vertex)]
#[repr(C)]
struct MyVertex {
    #[format(R32G32B32_SFLOAT)]
    position: [f32; 3],
    #[format(R32G32B32_SFLOAT)]
    color: [f32; 3],
}

enum Game {
    Uninit,
    Init(GameData),
}

struct GameData {
    graphics: GraphicsEngine,
    input: Input,
    camera: Camera,
    timer: Timer,
}

struct GraphicsEngine {
    window: Arc<Window>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    memory_allocator: Arc<StandardMemoryAllocator>,
    command_buffer_allocator: StandardCommandBufferAllocator,
    pipeline: Arc<GraphicsPipeline>,
    render_pass: Arc<RenderPass>,
    vertex_buffer: Subbuffer<[MyVertex]>,
    vs: Arc<ShaderModule>,
    fs: Arc<ShaderModule>,
    swap_mechanism: SwapMechanism<vs::Data>,
    previous_frame_end: Option<Box<dyn GpuFuture>>,
    window_resized: bool,
    recreate_swapchain: bool,
    last_frame: Instant,
    frames: u32,
    time: f32,
}

struct SwapMechanism<T> {
    swapchain: Arc<Swapchain>,
    framebuffers: Vec<Arc<Framebuffer>>,
    uniform_buffer_sets: Vec<Arc<PersistentDescriptorSet>>,
    uniform_buffers: Vec<Subbuffer<T>>,
}

impl GraphicsEngine {
    fn update_fps(&mut self) {
        let now = Instant::now();
        let delta = now - self.last_frame;
        self.frames += 1;
        self.time += delta.as_secs_f32();
        if self.time >= 1.0 {
            info!("FPS: {} / {:.1}%", self.frames, 100.0 * 120.0 / self.frames as f32);
            self.frames = 0;
            self.time = 0.0;
        }
        self.last_frame = now;
    }
}

impl<T> SwapMechanism<T> {
    fn swap_buffers<F: FnMut()>(&mut self, mut if_suboptimal: F, mut after_swap: impl FnMut(SwapchainAcquireFuture, Arc<Framebuffer>, &Subbuffer<T>, Arc<PersistentDescriptorSet>, SwapchainPresentInfo, F)) {
        let (image_index, suboptimal, acquire_future) = match swapchain::acquire_next_image(self.swapchain.clone(), None).map_err(Validated::unwrap) {
            Ok(r) => r,
            Err(VulkanError::OutOfDate) => {
                if_suboptimal();
                return;
            }
            Err(e) => panic!("failed to acquire next image: {e}"),
        };
        if suboptimal {
            if_suboptimal();
        }
        after_swap(
            acquire_future,
            self.framebuffers[image_index as usize].clone(),
            &self.uniform_buffers[image_index as usize],
            self.uniform_buffer_sets[image_index as usize].clone(),
            SwapchainPresentInfo::swapchain_image_index(self.swapchain.clone(), image_index),
            if_suboptimal,
        );
    }

    fn recreate(&mut self, new_dimensions: PhysicalSize<u32>) -> Vec<Arc<Image>> {
        let (swapchain, images) = self.swapchain.recreate(SwapchainCreateInfo {
            image_extent: new_dimensions.into(),
            ..self.swapchain.create_info()
        }).unwrap();
        self.swapchain = swapchain;
        images
    }
}

impl ApplicationHandler for Game {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        match cause {
            StartCause::Init => {
                info!("Init");
                let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
                let window = Arc::new(event_loop.create_window(WindowAttributes::default().with_title("Evolution VK")).unwrap());
                let required_extensions = Surface::required_extensions(&window);
                let instance = Instance::new(
                    library,
                    InstanceCreateInfo {
                        flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                        enabled_extensions: required_extensions,
                        ..Default::default()
                    },
                ).expect("failed to create instance");
                let surface = Surface::from_window(instance.clone(), window.clone()).unwrap();
                let device_extensions = DeviceExtensions {
                    khr_swapchain: true,
                    ..DeviceExtensions::empty()
                };
                let (physical_device, queue_family_index) = select_physical_device(&instance, &surface, &device_extensions);
                let (device, mut queues) = Device::new(
                    physical_device.clone(),
                    DeviceCreateInfo {
                        queue_create_infos: vec![QueueCreateInfo {
                            queue_family_index,
                            ..Default::default()
                        }],
                        enabled_extensions: device_extensions,
                        ..Default::default()
                    },
                ).expect("failed to create device");
                let queue = queues.next().unwrap();
                let (swapchain, images) = {
                    let caps = physical_device.surface_capabilities(&surface, Default::default()).expect("failed to get surface capabilities");
                    let dimensions = window.inner_size();
                    let composite_alpha = caps.supported_composite_alpha.into_iter().next().unwrap();
                    let image_format = physical_device.surface_formats(&surface, Default::default()).unwrap()[0].0;
                    Swapchain::new(
                        device.clone(),
                        surface,
                        SwapchainCreateInfo {
                            min_image_count: caps.min_image_count,
                            image_format,
                            image_extent: dimensions.into(),
                            image_usage: ImageUsage::COLOR_ATTACHMENT,
                            composite_alpha,
                            present_mode: PresentMode::Immediate,
                            ..Default::default()
                        },
                    ).unwrap()
                };
                let render_pass = get_render_pass(device.clone(), swapchain.clone());
                let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
                let vertex1 = MyVertex {
                    position: [-0.5, -0.5, 1.0],
                    color: [1.0, 0.0, 0.0],
                };
                let vertex2 = MyVertex {
                    position: [0.0, 0.5, 1.0],
                    color: [0.0, 1.0, 0.0],
                };
                let vertex3 = MyVertex {
                    position: [0.5, -0.25, 1.0],
                    color: [0.0, 0.0, 1.0],
                };
                let vertex_buffer = Buffer::from_iter(
                    memory_allocator.clone(),
                    BufferCreateInfo {
                        usage: BufferUsage::VERTEX_BUFFER,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                        ..Default::default()
                    },
                    vec![vertex1, vertex2, vertex3],
                ).unwrap();
                let vs = vs::load(device.clone()).expect("failed to create shader module");
                let fs = fs::load(device.clone()).expect("failed to create shader module");
                let (framebuffers, pipeline) = get_framebuffers_and_pipeline(window.inner_size(), &images, render_pass.clone(), memory_allocator.clone(), vs.entry_point("main").unwrap(), fs.entry_point("main").unwrap());
                let uniform_buffers = (0..swapchain.image_count()).map(|_| {
                    Buffer::new_sized::<vs::Data>(memory_allocator.clone(), BufferCreateInfo {
                        usage: BufferUsage::UNIFORM_BUFFER,
                        ..Default::default()
                    }, AllocationCreateInfo {
                        memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                        ..Default::default()
                    }).unwrap()
                }).collect::<Vec<_>>();
                let command_buffer_allocator = StandardCommandBufferAllocator::new(device.clone(), Default::default());
                let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(device.clone(), StandardDescriptorSetAllocatorCreateInfo::default()));
                let uniform_buffer_sets = uniform_buffers.iter().map(|buffer| {
                    PersistentDescriptorSet::new(
                        &descriptor_set_allocator,
                        pipeline.layout().set_layouts()[0].clone(),
                        [WriteDescriptorSet::buffer(0, buffer.clone())],
                        [],
                    ).unwrap()
                }).collect::<Vec<_>>();
                *self = Game::Init(GameData {
                    graphics: GraphicsEngine {
                        device: device.clone(),
                        queue,
                        window,
                        pipeline,
                        render_pass,
                        memory_allocator,
                        vs,
                        fs,
                        vertex_buffer,
                        command_buffer_allocator,
                        swap_mechanism: SwapMechanism {
                            swapchain,
                            framebuffers,
                            uniform_buffers,
                            uniform_buffer_sets,
                        },
                        previous_frame_end: Some(Box::new(sync::now(device)) as Box<dyn GpuFuture>),
                        window_resized: false,
                        recreate_swapchain: false,
                        last_frame: Instant::now(),
                        frames: 0,
                        time: 0.0,
                    },
                    input: Input::new(),
                    camera: Camera::new(),
                    timer: Timer::new(NonZero::new(20).unwrap(), FrameRateLimit::Unlimited),
                });
                event_loop.set_control_flow(ControlFlow::Poll);
            }
            StartCause::Poll => {
                if let Game::Init(data) = self {
                    data.timer.wait(&data.graphics.window);
                }
            }
            _ => {}
        }
    }

    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        info!("Application resumed");
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::Resized(_) => {
                if let Game::Init(data) = self {
                    data.graphics.window_resized = true;
                }
            }
            WindowEvent::Moved(_) => {}
            WindowEvent::CloseRequested => {
                info!("Close requested");
                event_loop.exit();
            }
            WindowEvent::Destroyed => {}
            WindowEvent::DroppedFile(_) => {}
            WindowEvent::HoveredFile(_) => {}
            WindowEvent::HoveredFileCancelled => {}
            WindowEvent::Focused(_) => {}
            WindowEvent::KeyboardInput {
                event,
                ..
            } => {
                if let Game::Init(data) = self {
                    data.input.process_input(event);
                }
            }
            WindowEvent::ModifiersChanged(_) => {}
            WindowEvent::Ime(_) => {}
            WindowEvent::CursorMoved { .. } => {}
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::MouseWheel { .. } => {}
            WindowEvent::MouseInput { .. } => {}
            WindowEvent::RedrawRequested => {
                if let Game::Init(data) = self {
                    data.timer.try_tick(|| {
                        data.input.tick(&mut data.camera);
                    });
                    data.timer.try_frame(|partial_tick| {
                        let engine = &mut data.graphics;
                        data.camera.adjust(engine.window.inner_size(), partial_tick);
                        engine.update_fps();
                        if engine.window_resized || engine.recreate_swapchain {
                            engine.window_resized = false;
                            engine.recreate_swapchain = false;
                            let new_dimensions = engine.window.inner_size();
                            let new_images = engine.swap_mechanism.recreate(new_dimensions);
                            let (new_framebuffers, new_pipeline) = get_framebuffers_and_pipeline(new_dimensions, &new_images, engine.render_pass.clone(), engine.memory_allocator.clone(), engine.vs.entry_point("main").unwrap(), engine.fs.entry_point("main").unwrap());
                            engine.pipeline = new_pipeline;
                            engine.swap_mechanism.framebuffers = new_framebuffers;
                        }
                        engine.swap_mechanism.swap_buffers(|| engine.recreate_swapchain = true, |acquire_future, framebuffer, uniform_buffer, descriptor_set, present_info, mut if_suboptimal| {
                            let mut builder = AutoCommandBufferBuilder::primary(&engine.command_buffer_allocator, engine.queue.queue_family_index(), CommandBufferUsage::OneTimeSubmit).unwrap();
                            builder
                                .begin_render_pass(
                                    RenderPassBeginInfo {
                                        clear_values: vec![
                                            Some([0.0, 0.0, 0.0, 1.0].into()),
                                            Some(1.0.into()),
                                        ],
                                        ..RenderPassBeginInfo::framebuffer(framebuffer)
                                    },
                                    SubpassBeginInfo {
                                        contents: SubpassContents::Inline,
                                        ..Default::default()
                                    },
                                ).unwrap()
                                .bind_pipeline_graphics(engine.pipeline.clone())
                                .unwrap()
                                .bind_descriptor_sets(PipelineBindPoint::Graphics, engine.pipeline.layout().clone(), 0, descriptor_set)
                                .unwrap()
                                .bind_vertex_buffers(0, engine.vertex_buffer.clone())
                                .unwrap()
                                .draw(engine.vertex_buffer.len() as u32, 1, 0, 0)
                                .unwrap()
                                .end_render_pass(Default::default())
                                .unwrap();
                            let command_buffer = builder.build().unwrap();
                            acquire_future.wait(None).unwrap();
                            engine.previous_frame_end.as_mut().unwrap().cleanup_finished();
                            *uniform_buffer.write().unwrap() = vs::Data {
                                world: Mat4::IDENTITY.into(),
                                view: data.camera.get_view().into(),
                                proj: data.camera.get_proj().into(),
                            };
                            let future = engine.previous_frame_end
                                               .take()
                                               .unwrap()
                                               .join(acquire_future)
                                               .then_execute(engine.queue.clone(), command_buffer)
                                               .unwrap()
                                               .then_swapchain_present(engine.queue.clone(), present_info)
                                               .then_signal_fence_and_flush();
                            match future.map_err(Validated::unwrap) {
                                Ok(future) => {
                                    engine.previous_frame_end = Some(future.boxed());
                                }
                                Err(VulkanError::OutOfDate) => {
                                    if_suboptimal();
                                    engine.previous_frame_end = Some(sync::now(engine.device.clone()).boxed());
                                }
                                Err(e) => {
                                    error!("failed to flush future: {e}");
                                    engine.previous_frame_end = Some(sync::now(engine.device.clone()).boxed());
                                }
                            }
                        });
                    });
                    data.timer.wait(&data.graphics.window);
                }
            }
            _ => {}
        }
    }
}

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: r"
            #version 460

            layout(location = 0) in vec3 position;
            layout(location = 1) in vec3 color;

            layout(location = 0) out vec3 v_color;

            layout(set = 0, binding = 0) uniform Data {
                mat4 world;
                mat4 view;
                mat4 proj;
            } uniforms;

            void main() {
                mat4 worldview = uniforms.view * uniforms.world;
                gl_Position = uniforms.proj * worldview * vec4(position, 1.0);
                v_color = color;
            }
        ",
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: r"
            #version 460

            layout(location = 0) in vec3 color;

            layout(location = 0) out vec4 f_color;

            void main() {
                f_color = vec4(color, 1.0);
            }
        ",
    }
}

pub fn select_physical_device(instance: &Arc<Instance>, surface: &Arc<Surface>, device_extensions: &DeviceExtensions) -> (Arc<PhysicalDevice>, u32) {
    instance
        .enumerate_physical_devices()
        .expect("failed to enumerate physical devices")
        .filter(|p| p.supported_extensions().contains(device_extensions))
        .filter_map(|p| {
            p.queue_family_properties()
             .iter()
             .enumerate()
             .position(|(i, q)| {
                 q.queue_flags.contains(QueueFlags::GRAPHICS) && p.surface_support(i as u32, surface).unwrap_or(false)
             })
             .map(|q| (p, q as u32))
        })
        .min_by_key(|(p, _)| {
            match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                _ => 4,
            }
        })
        .expect("no device available")
}

fn get_render_pass(device: Arc<Device>, swapchain: Arc<Swapchain>) -> Arc<RenderPass> {
    vulkano::single_pass_renderpass!(
        device,
        attachments: {
            color: {
                format: swapchain.image_format(),
                samples: 1,
                load_op: Clear,
                store_op: Store,
            },
            depth_stencil: {
                format: Format::D16_UNORM,
                samples: 1,
                load_op: Clear,
                store_op: DontCare,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {
                depth_stencil
            },
        },
    ).unwrap()
}

fn get_framebuffers_and_pipeline(window_size: PhysicalSize<u32>, images: &Vec<Arc<Image>>, render_pass: Arc<RenderPass>, allocator: Arc<StandardMemoryAllocator>, vs: EntryPoint, fs: EntryPoint) -> (Vec<Arc<Framebuffer>>, Arc<GraphicsPipeline>) {
    let device = allocator.device().clone();
    let depth_buffer = ImageView::new_default(
        Image::new(
            allocator,
            ImageCreateInfo {
                image_type: ImageType::Dim2d,
                format: Format::D16_UNORM,
                extent: images[0].extent(),
                usage: ImageUsage::DEPTH_STENCIL_ATTACHMENT | ImageUsage::TRANSIENT_ATTACHMENT,
                ..Default::default()
            },
            AllocationCreateInfo::default(),
        ).unwrap()
    ).unwrap();
    let framebuffers = images.iter().map(|image| {
        let view = ImageView::new_default(image.clone()).unwrap();
        Framebuffer::new(render_pass.clone(), FramebufferCreateInfo {
            attachments: vec![view, depth_buffer.clone()],
            ..Default::default()
        }).unwrap()
    }).collect();
    let pipeline = {
        let vertex_input_state = [MyVertex::per_vertex()].definition(&vs.info().input_interface).unwrap();
        let stages = [
            PipelineShaderStageCreateInfo::new(vs),
            PipelineShaderStageCreateInfo::new(fs),
        ];
        let layout = PipelineLayout::new(device.clone(), PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages).into_pipeline_layout_create_info(device.clone()).unwrap()).unwrap();
        let subpass = Subpass::from(render_pass.clone(), 0).unwrap();
        GraphicsPipeline::new(
            device,
            None,
            GraphicsPipelineCreateInfo {
                stages: stages.into_iter().collect(),
                vertex_input_state: Some(vertex_input_state),
                input_assembly_state: Some(InputAssemblyState::default()),
                viewport_state: Some(ViewportState {
                    viewports: smallvec![Viewport {
                        offset: [0.0, 0.0],
                        extent: window_size.into(),
                        depth_range: 0.0..=1.0,
                    }],
                    ..Default::default()
                }),
                rasterization_state: Some(RasterizationState::default()),
                depth_stencil_state: Some(DepthStencilState {
                    depth: Some(DepthState::simple()),
                    ..Default::default()
                }),
                multisample_state: Some(MultisampleState::default()),
                color_blend_state: Some(ColorBlendState::with_attachment_states(
                    subpass.num_color_attachments(),
                    ColorBlendAttachmentState::default(),
                )),
                subpass: Some(subpass.into()),
                ..GraphicsPipelineCreateInfo::layout(layout)
            },
        ).unwrap()
    };
    (framebuffers, pipeline)
}

fn main() {
    unsafe {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    info!("Initializing Evolution VK");
    let event_loop = EventLoop::new().unwrap();
    let mut game = Game::Uninit;
    event_loop.run_app(&mut game).unwrap();
    info!("Back to main");
}