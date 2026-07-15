pub mod buffer;
mod future;
mod pipeline;
mod queue;
mod swapchain;
mod window;

use crate::client::camera::Camera;
use crate::client::engine::buffer::{BufferConsumer, SectionBuffers};
use crate::client::engine::future::ExecutionFuture;
use crate::client::engine::pipeline::{Pipeline, PipelineConsumer};
use crate::client::engine::queue::Queue;
use crate::client::engine::swapchain::SwapChain;
use crate::client::engine::window::WindowParams;
use crate::client::input::InputHandler;
use crate::client::mesh::SectionMesh;
use crate::client::texture::TextureManager;
use crate::client::vertex::VertexPosTex;
use crate::if_else;
use crate::level::Level;
use crate::math::mat4::Mat4F32;
use crate::math::vec2u::Vec2U32;
use crate::math::vec3f::Vec3F32;
use crate::math::Vector3;
use log::{error, info};
use std::sync::Arc;
use std::time::Instant;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::command_buffer::{AutoCommandBufferBuilder, BlitImageInfo, CommandBufferUsage, CopyBufferToImageInfo, DrawIndexedIndirectCommand, PrimaryAutoCommandBuffer, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents};
use vulkano::descriptor_set::allocator::{StandardDescriptorSetAllocator, StandardDescriptorSetAllocatorCreateInfo};
use vulkano::descriptor_set::WriteDescriptorSet;
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
use vulkano::device::{Device, DeviceCreateInfo, DeviceExtensions, Features, QueueCreateInfo, QueueFlags};
use vulkano::format::Format;
use vulkano::image::sampler::{Filter, Sampler, SamplerAddressMode, SamplerCreateInfo, SamplerMipmapMode, LOD_CLAMP_NONE};
use vulkano::image::view::ImageView;
use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage};
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::render_pass::RenderPass;
use vulkano::swapchain::Surface;
use vulkano::{Validated, VulkanError, VulkanLibrary};
use winit::dpi::{LogicalPosition, Position};
use winit::event_loop::ActiveEventLoop;
use winit::window::{CursorGrabMode, Window, WindowAttributes};

pub struct GraphicsEngine {
    window: Arc<Window>,
    device: Arc<Device>,
    graphics_queue: Queue,
    transfer_queue: Queue,
    memory_allocator: Arc<StandardMemoryAllocator>,
    cb_allocator: StandardCommandBufferAllocator,
    ds_allocator: Arc<StandardDescriptorSetAllocator>,
    render_pass: Arc<RenderPass>,
    terrain_pipeline: Pipeline<VertexPosTex>,
    swapchain: SwapChain,
    viewport: Viewport,
    section_buffers: SectionBuffers<VertexPosTex>,
    exec_future: ExecutionFuture,
    last_frame: Instant,
    window_params: WindowParams,
    frames: u32,
    time: f32,
    mouse_grabbed: bool,
    wireframe: bool,
}

impl GraphicsEngine {
    pub const FRAMES_IN_FLIGHT: u32 = 2;

    pub fn new(event_loop: &ActiveEventLoop, texture_manager: &TextureManager) -> Self {
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
            khr_shader_draw_parameters: true,
            ..DeviceExtensions::empty()
        };
        let (physical_device, graphics_family_index) = Self::select_physical_device(&instance, &surface, &device_extensions);
        let transfer_family_index = physical_device
            .queue_family_properties()
            .iter()
            .enumerate()
            .filter(|(_, q)| q.queue_flags.intersects(QueueFlags::TRANSFER))
            .min_by_key(|(_, q)| q.queue_flags.count())
            .map(|(i, _)| i as u32);
        let (device, queues) = {
            let mut queue_create_infos = vec![QueueCreateInfo {
                queue_family_index: graphics_family_index,
                ..Default::default()
            }];
            if let Some(transfer_family_index) = transfer_family_index && transfer_family_index != graphics_family_index {
                info!("found transfer");
                queue_create_infos.push(QueueCreateInfo {
                    queue_family_index: transfer_family_index,
                    ..Default::default()
                })
            } //
            else {
                let queue_family_properties = &physical_device.queue_family_properties()[graphics_family_index as usize];
                info!("two graphics");
                if queue_family_properties.queue_count > 1 {
                    queue_create_infos[0].queues = vec![0.5, 0.5];
                }
            }
            Device::new(
                physical_device.clone(),
                DeviceCreateInfo {
                    queue_create_infos,
                    enabled_extensions: device_extensions,
                    enabled_features: Features {
                        fill_mode_non_solid: true,
                        multi_draw_indirect: true,
                        ..Features::empty()
                    },
                    ..Default::default()
                },
            ).expect("failed to create device")
        };
        let window_size = window.inner_size();
        let (graphics_queue, transfer_queue) = Queue::new(queues);
        info!("Using queue family {} for graphics and queue family {} for transfers", graphics_queue.get_family_index(), transfer_queue.get_family_index());
        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        let image_format = physical_device.surface_formats(&surface, Default::default()).unwrap()[0].0;
        let render_pass = Self::get_render_pass(device.clone(), image_format);
        let swapchain = SwapChain::new(window_size, image_format, physical_device.clone(), device.clone(), surface.clone(), render_pass.clone(), memory_allocator.clone());
        let cb_allocator = StandardCommandBufferAllocator::new(device.clone(), Default::default());
        let mut uploader = AutoCommandBufferBuilder::primary(
            &cb_allocator,
            graphics_queue.get_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        ).unwrap();
        let ds_allocator = Arc::new(StandardDescriptorSetAllocator::new(device.clone(), StandardDescriptorSetAllocatorCreateInfo::default()));
        let terrain_pipeline = {
            let texture = {
                let image = texture_manager.get_atlas_image();
                let width = image.width();
                let height = image.height();
                let extent = [width, height, 1];
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
                const MIP_LEVELS: u32 = 4;
                let image = Image::new(
                    memory_allocator.clone(),
                    ImageCreateInfo {
                        image_type: ImageType::Dim2d,
                        format: Format::R8G8B8A8_SRGB,
                        extent,
                        usage: ImageUsage::TRANSFER_DST | ImageUsage::TRANSFER_SRC | ImageUsage::SAMPLED,
                        mip_levels: MIP_LEVELS + 1,
                        ..Default::default()
                    },
                    AllocationCreateInfo::default(),
                ).unwrap();
                uploader.copy_buffer_to_image(CopyBufferToImageInfo::buffer_image(upload_buffer, image.clone())).unwrap();
                let mut src_width = width;
                let mut src_height = height;
                for i in 0..MIP_LEVELS {
                    let dst_width = if_else!(src_width > 1 => src_width / 2 ; 1);
                    let dst_height = if_else!(src_height > 1 => src_height / 2 ; 1);
                    let mut info = BlitImageInfo::images(image.clone(), image.clone());
                    info.regions[0].src_subresource.mip_level = i;
                    info.regions[0].src_offsets = [
                        [0, 0, 0],
                        [src_width, src_height, 1],
                    ];
                    info.regions[0].dst_subresource.mip_level = i + 1;
                    info.regions[0].dst_offsets = [
                        [0, 0, 0],
                        [dst_width, dst_height, 1],
                    ];
                    info.filter = Filter::Linear;
                    uploader.blit_image(info).unwrap();
                    src_width = dst_width;
                    src_height = dst_height;
                }
                ImageView::new_default(image).unwrap()
            };
            let sampler = Sampler::new(
                device.clone(),
                SamplerCreateInfo {
                    mag_filter: Filter::Nearest,
                    min_filter: Filter::Nearest,
                    address_mode: [SamplerAddressMode::ClampToEdge; 3],
                    mipmap_mode: SamplerMipmapMode::Linear,
                    lod: 0.0..=LOD_CLAMP_NONE,
                    mip_lod_bias: -0.0,
                    ..Default::default()
                },
            ).unwrap();
            Pipeline::new(
                memory_allocator.clone(),
                &ds_allocator,
                render_pass.clone(),
                |buffer| {
                    [
                        WriteDescriptorSet::buffer(0, buffer.clone()),
                        WriteDescriptorSet::sampler(1, sampler.clone()),
                        WriteDescriptorSet::image_view(2, texture.clone()),
                    ]
                },
                |buffer| {
                    [
                        WriteDescriptorSet::buffer(0, buffer.clone()),
                    ]
                },
            )
        };
        let viewport = Viewport {
            offset: [0.0, window_size.height as f32],
            extent: [window_size.width as f32, -(window_size.height as f32)],
            depth_range: 0.0..=1.0,
        };
        let mut exec_future = ExecutionFuture::now(device.clone());
        exec_future.join(uploader.build().unwrap(), &graphics_queue).then_signal_fence_and_flush().unwrap();
        Self {
            window,
            device,
            graphics_queue,
            transfer_queue,
            memory_allocator: memory_allocator.clone(),
            cb_allocator,
            ds_allocator,
            swapchain,
            terrain_pipeline,
            render_pass,
            viewport,
            section_buffers: SectionBuffers::new(memory_allocator),
            exec_future,
            window_params: WindowParams::new(window_size),
            last_frame: Instant::now(),
            frames: 0,
            time: 0.0,
            mouse_grabbed: false,
            wireframe: false,
        }
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

    pub fn get_window(&self) -> &Arc<Window> {
        &self.window
    }

    pub fn is_mouse_grabbed(&self) -> bool {
        self.mouse_grabbed
    }

    pub fn is_window_focused(&self) -> bool {
        self.window_params.is_window_focused()
    }

    pub fn set_window_focused(&mut self, focused: bool) {
        self.window_params.set_focused(focused);
    }

    pub fn changed_size(&mut self, size: impl Into<Vec2U32>) {
        self.window_params.changed_size(size);
    }

    pub fn update_fps(&mut self, player_pos: Vec3F32) {
        let now = Instant::now();
        let delta = now - self.last_frame;
        self.frames += 1;
        self.time += delta.as_secs_f32();
        if self.time >= 1.0 {
            let (phys, virt) = memory_stats::memory_stats().map_or_else(|| (0, 0), |stats| (stats.physical_mem, stats.virtual_mem));
            info!("FPS: {} / {:.1}% / Phys: {:.1}MB / Virt: {:.1}MB / Pos: {:.3}, {:.3}, {:.3}", self.frames, 100.0 * 120.0 / self.frames as f32, phys as f32 / 1024.0 / 1024.0, virt as f32 / 1024.0 / 1024.0, player_pos.x(), player_pos.y(), player_pos.z());
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

    pub fn update_section_meshes(&mut self, mut meshes: Vec<SectionMesh<VertexPosTex>>) {
        if meshes.is_empty() {
            return;
        }
        for mesh in &mut meshes {
            self.section_buffers.enforce_section_allocation(mesh, &self.memory_allocator);
        }
        self.section_buffers.reallocate_if_needed(&self.memory_allocator, &self.cb_allocator, &self.transfer_queue, &mut self.exec_future);
        let mut cb = AutoCommandBufferBuilder::primary(
            &self.cb_allocator,
            self.transfer_queue.get_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        ).unwrap();
        for mesh in meshes {
            self.section_buffers.submit(mesh, &self.memory_allocator, &mut cb);
        }
        self.exec_future.then_execute(cb.build().unwrap(), &self.transfer_queue);
    }

    pub fn resize_or_update_swapchain(&mut self) -> bool {
        if self.window_params.should_resize() || self.swapchain.needs_recreate() {
            let new_dimensions = self.window.inner_size();
            self.viewport.offset = [0.0, new_dimensions.height as f32];
            self.viewport.extent = [new_dimensions.width as f32, -(new_dimensions.height as f32)];
            self.swapchain.recreate(new_dimensions, self.render_pass.clone(), self.memory_allocator.clone());
            self.window_params.set_resized();
            true
        } //
        else {
            false
        }
    }

    pub fn render_game(&mut self, level: &Level, camera: &Camera) {
        if self.window_params.is_window_minimized() {
            return;
        }
        if let Some((acquire_future, framebuffer, present_info)) = self.swapchain.swap_buffers() {
            let mut cb = AutoCommandBufferBuilder::primary(&self.cb_allocator, self.graphics_queue.get_family_index(), CommandBufferUsage::OneTimeSubmit).unwrap();
            cb
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
                .unwrap();
            let transforms = self.render_level(&mut cb, level);
            cb.end_render_pass(Default::default()).unwrap();
            let cb = cb.build().unwrap();
            acquire_future.wait(None).unwrap();
            self.exec_future.cleanup_finished();
            self.terrain_pipeline.write_uniform(camera.get_uniform(), &self.swapchain);
            self.terrain_pipeline.write_storate(transforms, &self.swapchain);
            let future = self.exec_future
                             .join_future(acquire_future)
                             .then_execute(cb, &self.graphics_queue)
                             .then_swapchain_present(present_info, &self.graphics_queue)
                             .then_signal_fence_and_flush();
            match future.map_err(Validated::unwrap) {
                Ok(()) => (),
                Err(VulkanError::OutOfDate) => {
                    self.swapchain.set_needs_recreate();
                    self.exec_future = ExecutionFuture::now(self.device.clone());
                }
                Err(e) => {
                    error!("failed to flush future: {e}");
                    self.exec_future = ExecutionFuture::now(self.device.clone());
                }
            }
        }
    }

    fn render_level(&mut self, cb: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>, level: &Level) -> Vec<Mat4F32> {
        let min_y_section = level.get_min_y_section();
        let mut transforms = vec![];
        let mut draw_commands = vec![];
        let mut i = 0;
        level.get_chunks().for_each(|(pos, chunk)| {
            chunk.get_sections().iter().for_each(|section| {
                if let Some(region) = section.get_mesh_region() {
                    transforms.push(section.get_transform(section.get_pos(*pos, min_y_section)));
                    draw_commands.push(DrawIndexedIndirectCommand {
                        first_index: region.get_index_start(),
                        index_count: region.get_index_count(),
                        instance_count: 1,
                        first_instance: i,
                        vertex_offset: region.get_vertex_start(),
                    });
                    i += 1;
                }
            })
        });
        if !draw_commands.is_empty() {
            let indirect_buffer = Buffer::from_iter(
                self.memory_allocator.clone(),
                BufferCreateInfo {
                    usage: BufferUsage::INDIRECT_BUFFER,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default()
                },
                draw_commands,
            ).unwrap();
            self.terrain_pipeline.realloc_storage_if_needed(transforms.len(), &self.swapchain, &self.memory_allocator, &self.ds_allocator, |buffer| [WriteDescriptorSet::buffer(0, buffer.clone())]);
            cb
                .bind_pipeline(&mut self.terrain_pipeline, &self.swapchain).unwrap()
                .bind_buffers(&self.section_buffers).unwrap()
                .draw_indexed_indirect(indirect_buffer).unwrap();
        }
        transforms
    }
}

impl InputHandler for GraphicsEngine {
    fn toggle_grab_mouse(&mut self) {
        self.grab_mouse(!self.mouse_grabbed);
    }

    fn toggle_wireframe(&mut self) {
        self.wireframe = !self.wireframe;
        self.terrain_pipeline.set_wireframe(self.wireframe);
    }
}