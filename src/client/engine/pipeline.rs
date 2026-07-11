use crate::client::engine::swapchain::{FrameArray, PerFrameStorage, SwapChain};
use crate::client::mesh::Mesh;
use crate::client::vertex::VertexFormat;
use crate::if_else;
use log::{debug, warn};
use std::sync::Arc;
use tuple_map::TupleMap2;
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::{DescriptorSet, PersistentDescriptorSet, WriteDescriptorSet};
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
use vulkano::{DeviceSize, ValidationError};

pub(super) struct Pipeline<V: VertexFormat> {
    pipeline: Arc<GraphicsPipeline>,
    uniform_buffers: FrameArray<Subbuffer<V::Uniform>>,
    storage_buffers: FrameArray<Subbuffer<[V::SSBOType]>>,
    descriptor_sets: FrameArray<Arc<PersistentDescriptorSet>>,
    storage_descriptor_sets: FrameArray<Arc<PersistentDescriptorSet>>,
}

pub trait PipelineConsumer {
    fn render<'a, V: VertexFormat>(&mut self, pipeline: &mut Pipeline<V>, swapchain: &SwapChain, meshes: impl IntoIterator<Item = &'a Mesh<V>>) -> Result<&mut Self, Box<ValidationError>>;

    fn bind_pipeline<V: VertexFormat>(&mut self, pipeline: &mut Pipeline<V>, swap_chain: &SwapChain) -> Result<&mut Self, Box<ValidationError>>;

    // fn push_constant<V: VertexFormat>(&mut self, pipeline: &Pipeline<V>, pc: impl Into<V::PushConstant>) -> Result<&mut Self, Box<ValidationError>>;
}

impl PipelineConsumer for AutoCommandBufferBuilder<PrimaryAutoCommandBuffer> {
    fn render<'a, V: VertexFormat>(&mut self, pipeline: &mut Pipeline<V>, swapchain: &SwapChain, meshes: impl IntoIterator<Item = &'a Mesh<V>>) -> Result<&mut Self, Box<ValidationError>> {
        self
            .bind_pipeline_graphics(pipeline.pipeline.clone())?
            .bind_descriptor_sets(PipelineBindPoint::Graphics, pipeline.pipeline.layout().clone(), 0, pipeline.descriptor_sets.get(&swapchain).clone())?;
        for mesh in meshes {
            mesh.draw(self)?;
        }
        Ok(self)
    }

    fn bind_pipeline<V: VertexFormat>(&mut self, pipeline: &mut Pipeline<V>, swap_chain: &SwapChain) -> Result<&mut Self, Box<ValidationError>> {
        self
            .bind_pipeline_graphics(pipeline.pipeline.clone())?
            .bind_descriptor_sets(PipelineBindPoint::Graphics, pipeline.pipeline.layout().clone(), 0, pipeline.descriptor_sets.get(&swap_chain).clone())?
            .bind_descriptor_sets(PipelineBindPoint::Graphics, pipeline.pipeline.layout().clone(), 1, pipeline.storage_descriptor_sets.get(&swap_chain).clone())?;
        Ok(self)
    }

    // fn push_constant<V: VertexFormat>(&mut self, pipeline: &Pipeline<V>, pc: impl Into<V::PushConstant>) -> Result<&mut Self, Box<ValidationError>> {
    //     self.push_constants(pipeline.pipeline.layout().clone(), 0, pc.into())?;
    //     Ok(self)
    // }
}

impl<V: VertexFormat> Pipeline<V> {
    pub fn new<R1: IntoIterator<Item = WriteDescriptorSet>, R2: IntoIterator<Item = WriteDescriptorSet>>(allocator: Arc<StandardMemoryAllocator>, ds_allocator: &Arc<StandardDescriptorSetAllocator>, render_pass: Arc<RenderPass>, mut uniform_binding_maker: impl FnMut(&Subbuffer<V::Uniform>) -> R1, mut storage_binding_maker: impl FnMut(&Subbuffer<[V::SSBOType]>) -> R2) -> Pipeline<V> {
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
        let uniform_buffers = FrameArray::new(|| {
            Buffer::new_sized::<V::Uniform>(
                allocator.clone(),
                BufferCreateInfo {
                    usage: BufferUsage::UNIFORM_BUFFER,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default()
                },
            ).unwrap()
        });
        let storage_buffers = FrameArray::new(|| {
            Buffer::new_slice::<V::SSBOType>(
                allocator.clone(),
                BufferCreateInfo {
                    usage: BufferUsage::STORAGE_BUFFER,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default()
                },
                16,
            ).unwrap()
        });
        let descriptor_sets = uniform_buffers.create_attached(|buffer| {
            PersistentDescriptorSet::new(
                ds_allocator,
                pipeline.layout().set_layouts()[0].clone(),
                uniform_binding_maker(buffer),
                [],
            ).unwrap()
        });
        let storage_descriptor_sets = storage_buffers.create_attached(|buffer| {
            PersistentDescriptorSet::new(
                ds_allocator,
                pipeline.layout().set_layouts()[1].clone(),
                storage_binding_maker(buffer),
                [],
            ).unwrap()
        });
        Self {
            uniform_buffers,
            storage_buffers,
            descriptor_sets,
            storage_descriptor_sets,
            pipeline,
        }
    }

    pub fn write_uniform(&mut self, uniform: impl Into<V::Uniform>, swapchain: &SwapChain) {
        let (uniform_buffer) = self.uniform_buffers.get(swapchain);
        match uniform_buffer.write() {
            Ok(mut guard) => *guard = uniform.into(),
            Err(e) => warn!("Failed to write to uniform buffer! {e}"),
        }
    }

    pub fn write_storate<I: Into<V::SSBOType>, It: IntoIterator<Item = I, IntoIter = impl ExactSizeIterator<Item = I>>>(&mut self, items: It, swapchain: &SwapChain) {
        let iter = items.into_iter();
        let len = iter.len();
        if len as DeviceSize > self.storage_buffers.get(swapchain).len() {
            panic!("Data does not fit into storage buffer: Buffer Size = {}, Data length = {}", self.storage_buffers.get(swapchain).len(), len);
        }
        match self.storage_buffers.get(swapchain).write() {
            Ok(mut guard) => {
                for (i, item) in iter.enumerate() {
                    guard[i] = item.into();
                }
            }
            Err(e) => warn!("Failed to write to storage buffer! {e}"),
        }
    }

    pub fn realloc_storage_if_needed<R: IntoIterator<Item = WriteDescriptorSet>>(&mut self, len: usize, swapchain: &SwapChain, allocator: &Arc<StandardMemoryAllocator>, ds_allocator: &Arc<StandardDescriptorSetAllocator>, mut storage_binding_maker: impl FnMut(&Subbuffer<[V::SSBOType]>) -> R) {
        if len as DeviceSize > self.storage_buffers.get(swapchain).len() {
            let new_len = self.storage_buffers.get(swapchain).len().max(len as DeviceSize);
            debug!("Reallocated storage buffers from {} to {}", self.storage_buffers.get(swapchain).len(), new_len);
            self.storage_buffers = FrameArray::new(|| {
                Buffer::new_slice::<V::SSBOType>(
                    allocator.clone(),
                    BufferCreateInfo {
                        usage: BufferUsage::STORAGE_BUFFER,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                        ..Default::default()
                    },
                    new_len,
                ).unwrap()
            });
            self.storage_descriptor_sets = self.storage_buffers.create_attached(|buffer| {
                PersistentDescriptorSet::new(
                    ds_allocator,
                    self.storage_descriptor_sets.get(swapchain).layout().clone(),
                    storage_binding_maker(buffer),
                    [],
                ).unwrap()
            });
        }
    }

    pub fn set_wireframe(&mut self, wireframe: bool) {
        let device = self.pipeline.device();
        let vertex_input_state = self.pipeline.vertex_input_state().clone();
        let layout = self.pipeline.layout();
        let subpass = self.pipeline.subpass().clone();
        let number_of_color_attachments = self.pipeline.color_blend_state().unwrap().attachments.len() as u32;
        let (vs, fs) = V::load_shaders(device.clone());
        let (vs_entry, fs_entry) = (vs, fs).map(|s| s.entry_point("main").unwrap());
        let stages = [
            PipelineShaderStageCreateInfo::new(vs_entry),
            PipelineShaderStageCreateInfo::new(fs_entry),
        ];
        self.pipeline = GraphicsPipeline::new(
            device.clone(),
            None,
            GraphicsPipelineCreateInfo {
                stages: stages.into_iter().collect(),
                vertex_input_state: Some(vertex_input_state),
                input_assembly_state: Some(InputAssemblyState::default()),
                viewport_state: Some(ViewportState::default()),
                rasterization_state: Some(RasterizationState {
                    polygon_mode: if_else!(wireframe => PolygonMode::Line ; PolygonMode::Fill),
                    cull_mode: CullMode::Back,
                    ..Default::default()
                }),
                depth_stencil_state: Some(DepthStencilState {
                    depth: Some(DepthState::simple()),
                    ..Default::default()
                }),
                multisample_state: Some(MultisampleState::default()),
                color_blend_state: Some(ColorBlendState::with_attachment_states(
                    number_of_color_attachments,
                    ColorBlendAttachmentState {
                        blend: Some(AttachmentBlend::alpha()),
                        ..Default::default()
                    },
                )),
                subpass: Some(subpass),
                dynamic_state: [DynamicState::Viewport].into_iter().collect(),
                ..GraphicsPipelineCreateInfo::layout(layout.clone())
            },
        ).unwrap();
    }
}