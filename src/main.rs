use crate::math::angle::AngleDeg;
use crate::math::mat4::Mat4;
use crate::math::vec3::Vec3;
use log::{error, info};
use smallvec::smallvec;
use std::sync::Arc;
use std::time::Instant;
use vulkano::buffer::allocator::{SubbufferAllocator, SubbufferAllocatorCreateInfo};
use vulkano::buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage};
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents};
use vulkano::descriptor_set::allocator::{StandardDescriptorSetAllocator, StandardDescriptorSetAllocatorCreateInfo};
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
use vulkano::device::{Device, DeviceCreateInfo, DeviceExtensions, DeviceOwned, QueueCreateInfo, QueueFlags};
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
use vulkano::shader::EntryPoint;
use vulkano::swapchain::{Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo};
use vulkano::sync::GpuFuture;
use vulkano::{swapchain, sync, Validated, VulkanError, VulkanLibrary};
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Fullscreen, Window, WindowBuilder};

mod util;
mod math;

#[derive(BufferContents, Vertex)]
#[repr(C)]
struct MyVertex {
    #[format(R32G32B32_SFLOAT)]
    position: [f32; 3],
}

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: r"
            #version 460

            layout(location = 0) in vec3 position;

            layout(set = 0, binding = 0) uniform Data {
                mat4 world;
                mat4 view;
                mat4 proj;
            } uniforms;

            void main() {
                mat4 worldview = uniforms.view * uniforms.world;
                gl_Position = uniforms.proj * worldview * vec4(position, 1.0);
            }
        ",
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: r"
            #version 460

            layout(location = 0) out vec4 f_color;

            void main() {
                f_color = vec4(1.0, 0.0, 0.0, 1.0);
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
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let event_loop = EventLoop::new();
    let required_extensions = Surface::required_extensions(&event_loop);
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            enabled_extensions: required_extensions,
            ..Default::default()
        },
    ).expect("failed to create instance");
    let builder = WindowBuilder::new().with_title("Evolution VK");
    let window = Arc::new(builder.build(&event_loop).unwrap());
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
            enabled_extensions: device_extensions, // new
            ..Default::default()
        },
    ).expect("failed to create device");
    let queue = queues.next().unwrap();
    let (mut swapchain, images) = {
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
                ..Default::default()
            },
        ).unwrap()
    };
    let render_pass = get_render_pass(device.clone(), swapchain.clone());
    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
    let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(device.clone(), StandardDescriptorSetAllocatorCreateInfo::default()));
    let mut previous_frame_end = Some(sync::now(device.clone()).boxed());
    let vertex1 = MyVertex {
        position: [-0.5, -0.5, 0.0],
    };
    let vertex2 = MyVertex {
        position: [0.0, 0.5, 0.0],
    };
    let vertex3 = MyVertex {
        position: [0.5, -0.25, 0.0],
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
    let (mut framebuffers, mut pipeline) = get_framebuffers_and_pipeline(window.inner_size(), &images, render_pass.clone(), memory_allocator.clone(), vs.entry_point("main").unwrap(), fs.entry_point("main").unwrap());
    let command_buffer_allocator = StandardCommandBufferAllocator::new(device.clone(), Default::default());
    let uniform_buffer = SubbufferAllocator::new(
        memory_allocator.clone(),
        SubbufferAllocatorCreateInfo {
            buffer_usage: BufferUsage::UNIFORM_BUFFER,
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
    );
    let mut window_resized = false;
    let mut recreate_swapchain = false;
    let mut last_frame = Instant::now();
    let mut frames = 0;
    let mut time = 0.0;
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        }
        Event::WindowEvent {
            event: WindowEvent::Resized(_),
            ..
        } => {
            window_resized = true;
        }
        Event::MainEventsCleared => {
            let now = Instant::now();
            let delta = now - last_frame;
            frames += 1;
            time += delta.as_secs_f32();
            if time >= 1.0 {
                info!("FPS: {}", frames);
                frames = 0;
                time = 0.0;
            }
            last_frame = now;
            if window_resized || recreate_swapchain {
                window_resized = false;
                recreate_swapchain = false;
                let new_dimensions = window.inner_size();
                let (new_swapchain, new_images) = swapchain.recreate(SwapchainCreateInfo {
                    image_extent: new_dimensions.into(),
                    ..swapchain.create_info()
                }).expect("failed to recreate swapchain");
                swapchain = new_swapchain;
                let (new_framebuffers, new_pipeline) = get_framebuffers_and_pipeline(new_dimensions, &new_images, render_pass.clone(), memory_allocator.clone(), vs.entry_point("main").unwrap(), fs.entry_point("main").unwrap());
                pipeline = new_pipeline;
                framebuffers = new_framebuffers;
            }
            let uniform_buffer_subbuffer = {
                let uniform_data = vs::Data {
                    world: Mat4::IDENTITY.into(),
                    view: Mat4::look_to_rh((0.0, 0.0, 0.0), (0.0, 0.0, -1.0), Vec3::Y).into(),
                    proj: Mat4::perspective(AngleDeg::new(50.0), window.inner_size().width as f32 / window.inner_size().height as f32, 0.01, 100.0).into(),
                };
                let subbuffer = uniform_buffer.allocate_sized().unwrap();
                *subbuffer.write().unwrap() = uniform_data;
                subbuffer
            };
            let layout = &pipeline.layout().set_layouts()[0];
            let set = PersistentDescriptorSet::new(
                &descriptor_set_allocator,
                layout.clone(),
                [WriteDescriptorSet::buffer(0, uniform_buffer_subbuffer)],
                [],
            ).unwrap();
            let (image_index, suboptimal, acquire_future) = match swapchain::acquire_next_image(swapchain.clone(), None).map_err(Validated::unwrap) {
                Ok(r) => r,
                Err(VulkanError::OutOfDate) => {
                    recreate_swapchain = true;
                    return;
                }
                Err(e) => panic!("failed to acquire next image: {e}"),
            };
            if suboptimal {
                recreate_swapchain = true;
            }
            let mut builder = AutoCommandBufferBuilder::primary(&command_buffer_allocator, queue.queue_family_index(), CommandBufferUsage::OneTimeSubmit).unwrap();
            builder
                .begin_render_pass(
                    RenderPassBeginInfo {
                        clear_values: vec![
                            Some([0.0, 0.0, 1.0, 1.0].into()),
                            Some(1.0.into()),
                        ],
                        ..RenderPassBeginInfo::framebuffer(framebuffers[image_index as usize].clone())
                    },
                    SubpassBeginInfo {
                        contents: SubpassContents::Inline,
                        ..Default::default()
                    },
                ).unwrap()
                .bind_pipeline_graphics(pipeline.clone())
                .unwrap()
                .bind_descriptor_sets(PipelineBindPoint::Graphics, pipeline.layout().clone(), 0, set)
                .unwrap()
                .bind_vertex_buffers(0, vertex_buffer.clone())
                .unwrap()
                .draw(vertex_buffer.len() as u32, 1, 0, 0)
                .unwrap()
                .end_render_pass(Default::default())
                .unwrap();
            let command_buffer = builder.build().unwrap();
            let future = previous_frame_end
                .take()
                .unwrap()
                .join(acquire_future)
                .then_execute(queue.clone(), command_buffer)
                .unwrap()
                .then_swapchain_present(
                    queue.clone(),
                    SwapchainPresentInfo::swapchain_image_index(swapchain.clone(), image_index),
                )
                .then_signal_fence_and_flush();
            match future.map_err(Validated::unwrap) {
                Ok(future) => {
                    previous_frame_end = Some(future.boxed());
                }
                Err(VulkanError::OutOfDate) => {
                    recreate_swapchain = true;
                    previous_frame_end = Some(sync::now(device.clone()).boxed());
                }
                Err(e) => {
                    error!("failed to flush future: {e}");
                    previous_frame_end = Some(sync::now(device.clone()).boxed());
                }
            }
        }
        _ => (),
    });
}