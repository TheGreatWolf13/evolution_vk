use crate::client::engine::future::ExecutionFuture;
use crate::client::engine::queue::Queue;
use crate::client::mesh::SectionMesh;
use crate::client::vertex::{VertexFormat, VertexPosTex};
use core::fmt::Debug;
use log::debug;
use std::any::type_name;
use std::sync::Arc;
use vulkano::buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferInfo, PrimaryAutoCommandBuffer};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::{DeviceSize, ValidationError};

pub struct SectionBuffers<V: BufferContents> {
    vertex_buffer: GlobalBuffer<V>,
    index_buffer: GlobalBuffer<u32>,
}

struct GlobalBuffer<V: BufferContents> {
    buffer: Subbuffer<[V]>,
    capacity: usize,
    free_regions: Vec<AllocRegion>,
}

#[derive(Copy, Clone)]
struct AllocRegion {
    start: usize,
    capacity: usize,
}

#[derive(Clone, Debug)]
pub struct MappedRegion {
    vertex: BufferRegion,
    index: BufferRegion,
}

#[derive(Clone, Debug)]
struct BufferRegion {
    start: usize,
    capacity: usize,
    used: usize,
}

pub trait BufferConsumer<V: BufferContents> {
    fn bind_buffers(&mut self, buffers: &SectionBuffers<V>) -> Result<&mut Self, Box<ValidationError>>;
}

impl BufferConsumer<VertexPosTex> for AutoCommandBufferBuilder<PrimaryAutoCommandBuffer> {
    fn bind_buffers(&mut self, buffers: &SectionBuffers<VertexPosTex>) -> Result<&mut Self, Box<ValidationError>> {
        self
            .bind_vertex_buffers(0, buffers.vertex_buffer.buffer.clone())?
            .bind_index_buffer(buffers.index_buffer.buffer.clone())?;
        Ok(self)
    }
}

impl<V: VertexFormat> SectionBuffers<V> {
    pub(super) fn new(allocator: Arc<StandardMemoryAllocator>) -> Self {
        Self {
            vertex_buffer: GlobalBuffer::new(1024, BufferUsage::VERTEX_BUFFER, allocator.clone()),
            index_buffer: GlobalBuffer::new(1024, BufferUsage::INDEX_BUFFER, allocator),
        }
    }

    pub fn enforce_section_allocation(&mut self, section: &mut SectionMesh<V>, allocator: &Arc<StandardMemoryAllocator>) {
        match section {
            SectionMesh::Empty => {}
            SectionMesh::Mesh(data) => {
                let vertex_count = data.get_vertex_count();
                let index_count = data.get_index_count();
                if let Some(region) = data.get_region_mut() {
                    if vertex_count > region.vertex.capacity {
                        region.vertex = self.vertex_buffer.realloc(vertex_count, &region.vertex, allocator);
                    }
                    if index_count > region.index.capacity {
                        region.index = self.index_buffer.realloc(index_count, &region.index, allocator);
                    }
                } //
                else {
                    let vertex_region = self.vertex_buffer.malloc(vertex_count);
                    let index_region = self.index_buffer.malloc(index_count);
                    *data.get_region_mut() = Some(MappedRegion {
                        vertex: vertex_region,
                        index: index_region,
                    });
                }
            }
        }
    }

    pub fn submit(&mut self, mesh: SectionMesh<V>, allocator: &Arc<StandardMemoryAllocator>, commands: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>) {
        if let SectionMesh::Mesh(mut data) = mesh {
            data.get_region_mut().as_mut().unwrap().vertex.used = self.vertex_buffer.copy(data.get_vertices().iter().copied(), &data.get_region().unwrap().vertex, allocator, commands);
            data.get_region_mut().as_mut().unwrap().index.used = self.index_buffer.copy(data.get_indices().iter().copied(), &data.get_region().unwrap().index, allocator, commands);
        }
    }

    pub(super) fn reallocate_if_needed(&mut self, allocator: &Arc<StandardMemoryAllocator>, cb_allocator: &StandardCommandBufferAllocator, queue: &Queue, exec_future: &mut ExecutionFuture) {
        self.vertex_buffer.realloc_buffer_if_needed(allocator, cb_allocator, queue, exec_future);
        self.index_buffer.realloc_buffer_if_needed(allocator, cb_allocator, queue, exec_future);
    }
}

impl<T: BufferContents + Debug> GlobalBuffer<T> {
    fn new(capacity: usize, usage: BufferUsage, allocator: Arc<StandardMemoryAllocator>) -> Self {
        Self {
            buffer: Buffer::new_slice(
                allocator.clone(),
                BufferCreateInfo {
                    usage: usage | BufferUsage::TRANSFER_SRC,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default()
                },
                capacity as DeviceSize,
            ).unwrap(),
            capacity,
            free_regions: vec![AllocRegion::new(0, capacity)],
        }
    }

    fn update_region(&mut self, index: usize, len: usize) -> BufferRegion {
        if self.free_regions[index].capacity == len {
            self.free_regions.remove(index).to_buffer_region()
        } //
        else {
            let old_region = &self.free_regions[index];
            let new_region = AllocRegion {
                start: old_region.start + len,
                capacity: old_region.capacity - len,
            };
            self.free_regions.push(new_region);
            self.free_regions.swap_remove(index).to_buffer_region()
        }
    }

    fn malloc(&mut self, len: usize) -> BufferRegion {
        let mut region_index = None;
        for (i, region) in &mut self.free_regions.iter().enumerate() {
            if region.capacity >= len {
                region_index = Some(i);
                break;
            }
        }
        if let Some(index) = region_index {
            return self.update_region(index, len);
        }
        let cap = self.capacity;
        let new_cap = (cap * 2).max(cap + len);
        self.capacity = new_cap;
        debug!("Buffer<{}> reallocated from {} to {}", type_name::<T>().split("::").last().unwrap(), cap, new_cap);
        if let Some(last) = self.free_regions.last_mut() && last.start + last.capacity == cap {
            last.capacity += new_cap - cap;
        } //
        else {
            self.free_regions.push(AllocRegion {
                start: cap,
                capacity: new_cap - cap,
            })
        }
        self.update_region(self.free_regions.len() - 1, len)
    }

    fn realloc(&mut self, len: usize, region: &BufferRegion, allocator: &Arc<StandardMemoryAllocator>) -> BufferRegion {
        todo!()
    }

    fn realloc_buffer_if_needed(&mut self, allocator: &Arc<StandardMemoryAllocator>, cb_allocator: &StandardCommandBufferAllocator, queue: &Queue, exec_future: &mut ExecutionFuture) {
        if self.capacity as DeviceSize != self.buffer.len() {
            let new_buffer = Buffer::new_slice(
                allocator.clone(),
                BufferCreateInfo {
                    usage: self.buffer.buffer().usage() | BufferUsage::TRANSFER_DST,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                    ..Default::default()
                },
                self.capacity as DeviceSize,
            ).unwrap();
            let info = CopyBufferInfo::buffers(self.buffer.clone(), new_buffer.clone());
            self.buffer = new_buffer;
            let mut cb = AutoCommandBufferBuilder::primary(
                cb_allocator,
                queue.get_family_index(),
                CommandBufferUsage::OneTimeSubmit,
            ).unwrap();
            cb.copy_buffer(info).unwrap();
            exec_future.join(cb.build().unwrap(), queue);
        }
    }

    fn copy<I: IntoIterator<Item = T, IntoIter: ExactSizeIterator>>(&mut self, data: I, region: &BufferRegion, allocator: &Arc<StandardMemoryAllocator>, uploader: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>) -> usize {
        let it = data.into_iter();
        let len = it.len();
        let vec = it.collect::<Vec<_>>();
        let upload_buffer = Buffer::from_iter(
            allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_SRC,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            vec,
        ).unwrap();
        // let mut info = CopyBufferInfo::buffers(upload_buffer, self.buffer.clone());
        // info.regions[0].dst_offset = region.start as DeviceSize;
        // uploader.copy_buffer(info).unwrap();
        self.buffer = upload_buffer;
        len
    }
}

impl AllocRegion {
    fn new(start: usize, capacity: usize) -> Self {
        Self {
            start,
            capacity,
        }
    }

    fn to_buffer_region(self) -> BufferRegion {
        BufferRegion {
            start: self.start,
            capacity: self.capacity,
            used: 0,
        }
    }
}

impl MappedRegion {
    pub fn get_index_count(&self) -> u32 {
        self.index.used as u32
    }

    pub fn get_index_start(&self) -> u32 {
        self.index.start as u32
    }

    pub fn get_vertex_start(&self) -> u32 {
        self.vertex.start as u32
    }

    pub fn get_vertex_capacity(&self) -> usize {
        self.vertex.capacity
    }

    pub fn get_index_capacity(&self) -> usize {
        self.index.capacity
    }
}