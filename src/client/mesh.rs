use crate::client::vertex::{Vertex, VertexFormat, VertexPosTex};
use std::sync::Arc;
use vulkano::buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::pipeline::PipelineLayout;
use vulkano::ValidationError;

pub struct Mesh<V: VertexFormat> {
    vertex_buffer: Subbuffer<[V]>,
    index_buffer: Subbuffer<[u32]>,
    transform: V::PushConstantInput,
}

pub struct MeshBuilder<V: VertexFormat> {
    transform: V::PushConstantInput,
    vertex_buffer: Vec<V>,
    index_buffer: Vec<u32>,
}

impl<V: VertexFormat> Mesh<V> {
    pub fn draw<'a>(&self, builder: &'a mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>, layout: Arc<PipelineLayout>) -> Result<&'a mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>, Box<ValidationError>> {
        Ok(
            builder
                .push_constants(layout, 0, self.transform.into())?
                .bind_vertex_buffers(0, self.vertex_buffer.clone())?
                .bind_index_buffer(self.index_buffer.clone())?
                .draw_indexed(self.index_buffer.len() as u32, 1, 0, 0, 0)?
        )
    }

    pub fn get_transform(&self) -> V::PushConstantInput {
        self.transform
    }
}

impl<V: VertexFormat> MeshBuilder<V> {
    pub fn new(transform: V::PushConstantInput) -> Self {
        Self {
            transform,
            vertex_buffer: vec![],
            index_buffer: vec![],
        }
    }

    pub fn triangle(mut self, vertices: [V; 3]) -> Self {
        let index = self.vertex_buffer.len() as u32;
        self.vertex_buffer.extend(vertices);
        self.index_buffer.extend([index, index + 1, index + 2]);
        self
    }

    pub fn quad(mut self, vertices: [V; 4]) -> Self {
        let index = self.vertex_buffer.len() as u32;
        self.vertex_buffer.extend(vertices);
        self.index_buffer.extend([index, index + 1, index + 2, index, index + 2, index + 3]);
        self
    }

    pub fn build(self, allocator: Arc<StandardMemoryAllocator>) -> Option<Mesh<V>> {
        if self.vertex_buffer.is_empty() {
            return None;
        }
        Some(Mesh {
            transform: self.transform,
            vertex_buffer: Self::create_buffer(BufferUsage::VERTEX_BUFFER, self.vertex_buffer, allocator.clone()),
            index_buffer: Self::create_buffer(BufferUsage::INDEX_BUFFER, self.index_buffer, allocator),
        })
    }

    pub fn merge(mut self, mesh: MeshBuilder<V>) -> Self {
        let last_index = self.vertex_buffer.len() as u32;
        self.index_buffer.extend(mesh.index_buffer.iter().map(|i| i + last_index));
        self.vertex_buffer.extend(mesh.vertex_buffer.iter().map(|v| v.transform(mesh.transform, self.transform)));
        self
    }

    fn create_buffer<T: BufferContents>(usage: BufferUsage, content: Vec<T>, allocator: Arc<StandardMemoryAllocator>) -> Subbuffer<[T]> {
        Buffer::from_iter(
            allocator,
            BufferCreateInfo {
                usage,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            content,
        ).unwrap()
    }
}

impl MeshBuilder<VertexPosTex> {
    pub fn cube(mut self, x: f32, y: f32, z: f32) -> Self {
        self.vertex_buffer.reserve(4 * 6);
        self.index_buffer.reserve(6 * 6);
        self.quad([
            //Down
            Vertex::new().pos(x, y, z).uv(0.0, 1.0),
            Vertex::new().pos(x + 1.0, y, z).uv(1.0, 1.0),
            Vertex::new().pos(x + 1.0, y, z + 1.0).uv(1.0, 0.0),
            Vertex::new().pos(x, y, z + 1.0).uv(0.0, 0.0),
        ]).quad([
            //Up
            Vertex::new().pos(x, y + 1.0, z).uv(0.0, 0.0),
            Vertex::new().pos(x, y + 1.0, z + 1.0).uv(0.0, 1.0),
            Vertex::new().pos(x + 1.0, y + 1.0, z + 1.0).uv(1.0, 1.0),
            Vertex::new().pos(x + 1.0, y + 1.0, z).uv(1.0, 0.0),
        ]).quad([
            //South
            Vertex::new().pos(x, y, z + 1.0).uv(0.0, 1.0),
            Vertex::new().pos(x + 1.0, y, z + 1.0).uv(1.0, 1.0),
            Vertex::new().pos(x + 1.0, y + 1.0, z + 1.0).uv(1.0, 0.0),
            Vertex::new().pos(x, y + 1.0, z + 1.0).uv(0.0, 0.0),
        ]).quad([
            //North
            Vertex::new().pos(x + 1.0, y, z).uv(0.0, 1.0),
            Vertex::new().pos(x, y, z).uv(1.0, 1.0),
            Vertex::new().pos(x, y + 1.0, z).uv(1.0, 0.0),
            Vertex::new().pos(x + 1.0, y + 1.0, z).uv(0.0, 0.0),
        ]).quad([
            //East
            Vertex::new().pos(x + 1.0, y, z + 1.0).uv(0.0, 1.0),
            Vertex::new().pos(x + 1.0, y, z).uv(1.0, 1.0),
            Vertex::new().pos(x + 1.0, y + 1.0, z).uv(1.0, 0.0),
            Vertex::new().pos(x + 1.0, y + 1.0, z + 1.0).uv(0.0, 0.0),
        ]).quad([
            //West
            Vertex::new().pos(x, y, z).uv(0.0, 1.0),
            Vertex::new().pos(x, y, z + 1.0).uv(1.0, 1.0),
            Vertex::new().pos(x, y + 1.0, z + 1.0).uv(1.0, 0.0),
            Vertex::new().pos(x, y + 1.0, z).uv(0.0, 0.0),
        ])
    }
}