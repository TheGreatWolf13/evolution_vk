mod swapchain;
mod pipeline;

use crate::client::engine::pipeline::{Pipeline, PipelineConsumer};
use crate::client::engine::swapchain::SwapChain;
use crate::client::input::InputHandler;
use crate::client::mesh::Mesh;
use crate::client::vertex::{VertexFormat, VertexPosCol, VertexPosTex};
use log::{error, info};
use std::sync::Arc;
use std::time::Instant;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferToImageInfo, PrimaryCommandBufferAbstract, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents};
use vulkano::descriptor_set::allocator::{StandardDescriptorSetAllocator, StandardDescriptorSetAllocatorCreateInfo};
use vulkano::descriptor_set::WriteDescriptorSet;
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
use vulkano::device::{Device, DeviceCreateInfo, DeviceExtensions, Features, Queue, QueueCreateInfo, QueueFlags};
use vulkano::format::Format;
use vulkano::image::sampler::{Filter, Sampler, SamplerAddressMode, SamplerCreateInfo};
use vulkano::image::view::ImageView;
use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage};
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::render_pass::RenderPass;
use vulkano::swapchain::Surface;
use vulkano::sync::GpuFuture;
use vulkano::{sync, Validated, VulkanError, VulkanLibrary};
use winit::dpi::{LogicalPosition, Position};
use winit::event_loop::ActiveEventLoop;
use winit::window::{CursorGrabMode, Window, WindowAttributes};

pub struct GraphicsEngine {
    window: Arc<Window>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    memory_allocator: Arc<StandardMemoryAllocator>,
    cb_allocator: StandardCommandBufferAllocator,
    render_pass: Arc<RenderPass>,
    tex_pipeline: Pipeline<VertexPosTex>,
    col_pipeline: Pipeline<VertexPosCol>,
    swapchain: SwapChain,
    viewport: Viewport,
    previous_frame_end: Option<Box<dyn GpuFuture>>,
    last_frame: Instant,
    frames: u32,
    time: f32,
    window_resized: bool,
    window_focused: bool,
    mouse_grabbed: bool,
    wireframe: bool,
}

impl GraphicsEngine {
    pub fn new(event_loop: &ActiveEventLoop) -> Self {
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
        let (physical_device, queue_family_index) = Self::select_physical_device(&instance, &surface, &device_extensions);
        let (device, mut queues) = Device::new(
            physical_device.clone(),
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                enabled_extensions: device_extensions,
                enabled_features: Features {
                    fill_mode_non_solid: true,
                    ..Features::empty()
                },
                ..Default::default()
            },
        ).expect("failed to create device");
        let window_size = window.inner_size();
        let queue = queues.next().unwrap();
        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        let image_format = physical_device.surface_formats(&surface, Default::default()).unwrap()[0].0;
        let render_pass = Self::get_render_pass(device.clone(), image_format);
        let swapchain = SwapChain::new(window_size, image_format, physical_device.clone(), device.clone(), surface.clone(), render_pass.clone(), memory_allocator.clone());
        let cb_allocator = StandardCommandBufferAllocator::new(device.clone(), Default::default());
        let mut uploader = AutoCommandBufferBuilder::primary(
            &cb_allocator,
            queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        ).unwrap();
        let ds_allocator = Arc::new(StandardDescriptorSetAllocator::new(device.clone(), StandardDescriptorSetAllocatorCreateInfo::default()));
        let tex_pipeline = {
            let texture = {
                let image = image::open("res/assets/textures/cobblestone.png").unwrap().to_rgba8();
                let extent = [image.width(), image.height(), 1];
                let upload_buffer = Buffer::from_iter(
                    memory_allocator.clone(),
                    BufferCreateInfo {
                        usage: BufferUsage::TRANSFER_SRC,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                        ..Default::default()
                    },
                    image.into_iter().cloned(),
                ).unwrap();
                let image = Image::new(
                    memory_allocator.clone(),
                    ImageCreateInfo {
                        image_type: ImageType::Dim2d,
                        format: Format::R8G8B8A8_SRGB,
                        extent,
                        usage: ImageUsage::TRANSFER_DST | ImageUsage::SAMPLED,
                        ..Default::default()
                    },
                    AllocationCreateInfo::default(),
                ).unwrap();
                uploader.copy_buffer_to_image(CopyBufferToImageInfo::buffer_image(upload_buffer, image.clone())).unwrap();
                ImageView::new_default(image).unwrap()
            };
            let sampler = Sampler::new(
                device.clone(),
                SamplerCreateInfo {
                    mag_filter: Filter::Nearest,
                    min_filter: Filter::Linear,
                    address_mode: [SamplerAddressMode::ClampToEdge; 3],
                    ..Default::default()
                },
            ).unwrap();
            Pipeline::new(memory_allocator.clone(), &ds_allocator, render_pass.clone(), &swapchain, |buffer| {
                [
                    WriteDescriptorSet::buffer(0, buffer.clone()),
                    WriteDescriptorSet::sampler(1, sampler.clone()),
                    WriteDescriptorSet::image_view(2, texture.clone()),
                ]
            })
        };
        let col_pipeline = Pipeline::new(memory_allocator.clone(), &ds_allocator, render_pass.clone(), &swapchain, |buffer| {
            [WriteDescriptorSet::buffer(0, buffer.clone())]
        });
        let viewport = Viewport {
            offset: [0.0, window_size.height as f32],
            extent: [window_size.width as f32, -(window_size.height as f32)],
            depth_range: 0.0..=1.0,
        };
        Self {
            window,
            device: device.clone(),
            queue: queue.clone(),
            memory_allocator,
            cb_allocator,
            swapchain,
            tex_pipeline,
            col_pipeline,
            render_pass,
            viewport,
            previous_frame_end: Some(uploader.build().unwrap().execute(queue).unwrap().boxed()),
            window_resized: false,
            last_frame: Instant::now(),
            frames: 0,
            time: 0.0,
            mouse_grabbed: false,
            window_focused: true,
            wireframe: false,
        }
    }

    pub fn get_allocator(&self) -> &Arc<StandardMemoryAllocator> {
        &self.memory_allocator
    }

    fn select_physical_device(instance: &Arc<Instance>, surface: &Arc<Surface>, device_extensions: &DeviceExtensions) -> (Arc<PhysicalDevice>, u32) {
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

    fn get_render_pass(device: Arc<Device>, format: Format) -> Arc<RenderPass> {
        vulkano::single_pass_renderpass!(
            device,
            attachments: {
                color: {
                    format: format,
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

    pub fn set_window_should_resize(&mut self, resize: bool) {
        self.window_resized = resize;
    }

    pub fn set_window_focused(&mut self, focused: bool) {
        self.window_focused = focused;
    }

    pub fn get_window(&self) -> &Arc<Window> {
        &self.window
    }

    pub fn is_mouse_grabbed(&self) -> bool {
        self.mouse_grabbed
    }

    pub fn is_window_focused(&self) -> bool {
        self.window_focused
    }

    pub fn update_fps(&mut self) {
        let now = Instant::now();
        let delta = now - self.last_frame;
        self.frames += 1;
        self.time += delta.as_secs_f32();
        if self.time >= 1.0 {
            let (phys, virt) = memory_stats::memory_stats().map_or_else(|| (0, 0), |stats| (stats.physical_mem, stats.virtual_mem));
            info!("FPS: {} / {:.1}% / Phys: {:.1}MB / Virt: {:.1}MB", self.frames, 100.0 * 120.0 / self.frames as f32, phys as f32 / 1024.0 / 1024.0, virt as f32 / 1024.0 / 1024.0);
            self.frames = 0;
            self.time = 0.0;
        }
        self.last_frame = now;
    }

    pub fn grab_mouse(&mut self, grab: bool) {
        let size = self.window.inner_size();
        if grab {
            self.window.set_cursor_position(Position::Logical(LogicalPosition::new(size.width as f64 / 2.0, size.height as f64 / 2.0))).unwrap();
            self.window.set_cursor_grab(CursorGrabMode::Locked).unwrap();
            self.window.set_cursor_visible(false);
            self.mouse_grabbed = true;
        } //
        else {
            self.window.set_cursor_grab(CursorGrabMode::None).unwrap();
            self.window.set_cursor_position(Position::Logical(LogicalPosition::new(size.width as f64 / 2.0, size.height as f64 / 2.0))).unwrap();
            self.window.set_cursor_visible(true);
            self.mouse_grabbed = false;
        }
    }

    pub fn resize_or_update_swapchain(&mut self) {
        if self.window_resized || self.swapchain.needs_recreate() {
            let new_dimensions = self.window.inner_size();
            if new_dimensions.width == 0 || new_dimensions.height == 0 {
                return;
            }
            self.viewport.offset = [0.0, new_dimensions.height as f32];
            self.viewport.extent = [new_dimensions.width as f32, -(new_dimensions.height as f32)];
            self.swapchain.recreate(new_dimensions, self.render_pass.clone(), self.memory_allocator.clone());
            self.window_resized = false;
        }
    }

    pub fn swap_buffers(&mut self, tex: (<VertexPosTex as VertexFormat>::Uniform, &Vec<Mesh<VertexPosTex>>), col: (<VertexPosCol as VertexFormat>::Uniform, &Vec<Mesh<VertexPosCol>>)) {
        self.swapchain.swap_buffers(|swapchain, acquire_future, framebuffer, present_info| {
            let mut builder = AutoCommandBufferBuilder::primary(&self.cb_allocator, self.queue.queue_family_index(), CommandBufferUsage::OneTimeSubmit).unwrap();
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
                .set_viewport(0, [self.viewport.clone()].into_iter().collect())
                .unwrap()
                .render(&self.tex_pipeline, swapchain, tex.1)
                .unwrap()
                .render(&self.col_pipeline, swapchain, col.1)
                .unwrap()
                .end_render_pass(Default::default())
                .unwrap();
            let command_buffer = builder.build().unwrap();
            acquire_future.wait(None).unwrap();
            self.previous_frame_end.as_mut().unwrap().cleanup_finished();
            self.tex_pipeline.write_uniform(tex.0, swapchain);
            self.col_pipeline.write_uniform(col.0, swapchain);
            let future = self.previous_frame_end
                             .take()
                             .unwrap()
                             .join(acquire_future)
                             .then_execute(self.queue.clone(), command_buffer)
                             .unwrap()
                             .then_swapchain_present(self.queue.clone(), present_info)
                             .then_signal_fence_and_flush();
            match future.map_err(Validated::unwrap) {
                Ok(future) => {
                    self.previous_frame_end = Some(future.boxed());
                }
                Err(VulkanError::OutOfDate) => {
                    swapchain.set_needs_recreate();
                    self.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
                }
                Err(e) => {
                    error!("failed to flush future: {e}");
                    self.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
                }
            }
        });
    }
}

impl InputHandler for GraphicsEngine {
    fn toggle_grab_mouse(&mut self) {
        self.grab_mouse(!self.mouse_grabbed);
    }

    fn toggle_wireframe(&mut self) {
        self.wireframe = !self.wireframe;
        self.col_pipeline.set_wireframe(self.wireframe);
        self.tex_pipeline.set_wireframe(self.wireframe);
    }
}