use crate::client::engine::swapchain::{FrameVec, SwapChain};
use crate::client::vertex::VertexFormat;
use std::sync::Arc;
use tuple_map::TupleMap2;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::device::DeviceOwned;
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::pipeline::graphics::color_blend::{AttachmentBlend, ColorBlendAttachmentState, ColorBlendState};
use vulkano::pipeline::graphics::depth_stencil::{DepthState, DepthStencilState};
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::multisample::MultisampleState;
use vulkano::pipeline::graphics::rasterization::{CullMode, PolygonMode, RasterizationState};
use vulkano::pipeline::graphics::vertex_input::VertexDefinition;
use vulkano::pipeline::graphics::viewport::ViewportState;
use vulkano::pipeline::graphics::GraphicsPipelineCreateInfo;
use vulkano::pipeline::layout::PipelineDescriptorSetLayoutCreateInfo;
use vulkano::pipeline::{DynamicState, GraphicsPipeline, Pipeline as P, PipelineBindPoint, PipelineLayout, PipelineShaderStageCreateInfo};
use vulkano::render_pass::{RenderPass, Subpass};
use vulkano::ValidationError;

pub(super) struct Pipeline<V: VertexFormat> {
    pipeline: Arc<GraphicsPipeline>,
    uniform_buffers: FrameVec<Subbuffer<V::Uniform>>,
    descriptor_sets: FrameVec<Arc<PersistentDescriptorSet>>,
    vertex_buffer: Subbuffer<[V]>, //Depends on the object
}

pub trait PipelineConsumer {
    fn render<V: VertexFormat>(&mut self, pipeline: &Pipeline<V>, swapchain: &SwapChain) -> Result<&mut Self, Box<ValidationError>>;
}

impl PipelineConsumer for AutoCommandBufferBuilder<PrimaryAutoCommandBuffer> {
    fn render<V: VertexFormat>(&mut self, pipeline: &Pipeline<V>, swapchain: &SwapChain) -> Result<&mut Self, Box<ValidationError>> {
        self
            .bind_pipeline_graphics(pipeline.pipeline.clone())?
            .bind_descriptor_sets(PipelineBindPoint::Graphics, pipeline.pipeline.layout().clone(), 0, pipeline.descriptor_sets.get(&swapchain).clone())?
            .bind_vertex_buffers(0, pipeline.vertex_buffer.clone())?
            .draw(pipeline.vertex_buffer.len() as u32, 1, 0, 0)?;
        Ok(self)
    }
}

impl<V: VertexFormat> Pipeline<V> {
    pub fn new<R: IntoIterator<Item = WriteDescriptorSet>>(vertices: Vec<V>, allocator: Arc<StandardMemoryAllocator>, ds_allocator: &Arc<StandardDescriptorSetAllocator>, render_pass: Arc<RenderPass>, swapchain: &SwapChain, mut binding_maker: impl FnMut(&Subbuffer<V::Uniform>) -> R) -> Pipeline<V> {
        let device = allocator.device().clone();
        let (vs, fs) = V::load_shaders(device.clone());
        let (vs_entry, fs_entry) = (vs, fs).map(|s| s.entry_point("main").unwrap());
        let pipeline = {
            let vertex_input_state = [V::per_vertex()].definition(&vs_entry.info().input_interface).unwrap();
            let stages = [
                PipelineShaderStageCreateInfo::new(vs_entry),
                PipelineShaderStageCreateInfo::new(fs_entry),
            ];
            let layout = PipelineLayout::new(device.clone(), PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages).into_pipeline_layout_create_info(device.clone()).unwrap()).unwrap();
            let subpass = Subpass::from(render_pass.clone(), 0).unwrap();
            GraphicsPipeline::new(
                device.clone(),
                None,
                GraphicsPipelineCreateInfo {
                    stages: stages.into_iter().collect(),
                    vertex_input_state: Some(vertex_input_state),
                    input_assembly_state: Some(InputAssemblyState::default()),
                    viewport_state: Some(ViewportState::default()),
                    rasterization_state: Some(RasterizationState {
                        polygon_mode: PolygonMode::Fill,
                        cull_mode: CullMode::Back,
                        ..Default::default()
                    }),
                    depth_stencil_state: Some(DepthStencilState {
                        depth: Some(DepthState::simple()),
                        ..Default::default()
                    }),
                    multisample_state: Some(MultisampleState::default()),
                    color_blend_state: Some(ColorBlendState::with_attachment_states(
                        subpass.num_color_attachments(),
                        ColorBlendAttachmentState {
                            blend: Some(AttachmentBlend::alpha()),
                            ..Default::default()
                        },
                    )),
                    subpass: Some(subpass.into()),
                    dynamic_state: [DynamicState::Viewport].into_iter().collect(),
                    ..GraphicsPipelineCreateInfo::layout(layout)
                },
            ).unwrap()
        };
        let uniform_buffers = swapchain.create_framevec(|| {
            Buffer::new_sized::<V::Uniform>(allocator.clone(), BufferCreateInfo {
                usage: BufferUsage::UNIFORM_BUFFER,
                ..Default::default()
            }, AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            }).unwrap()
        });
        let descriptor_sets = uniform_buffers.create_new_attached(|buffer| {
            PersistentDescriptorSet::new(
                ds_allocator,
                pipeline.layout().set_layouts()[0].clone(),
                binding_maker(buffer),
                [],
            ).unwrap()
        });
        let vertex_buffer = Buffer::from_iter(
            allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            vertices,
        ).unwrap();
        Self {
            uniform_buffers,
            descriptor_sets,
            pipeline,
            vertex_buffer,
        }
    }

    pub fn write_uniform(&self, uniform: V::Uniform, swapchain: &SwapChain) {
        *self.uniform_buffers.get(swapchain).write().unwrap() = uniform;
    }
}