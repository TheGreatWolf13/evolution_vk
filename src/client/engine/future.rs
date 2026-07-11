use crate::client::engine::queue::Queue;
use crate::client::engine::swapchain::FrameArray;
use crate::client::engine::GraphicsEngine;
use std::sync::Arc;
use vulkano::command_buffer::PrimaryAutoCommandBuffer;
use vulkano::device::Device;
use vulkano::swapchain::SwapchainPresentInfo;
use vulkano::sync::fence::{Fence, FenceCreateFlags, FenceCreateInfo};
use vulkano::sync::GpuFuture;
use vulkano::{sync, Validated, VulkanError};

pub(super) struct ExecutionFuture {
    inner: Option<Box<dyn GpuFuture>>,
    last_queue: Option<Queue>,
    fences: FrameArray<Fence, { GraphicsEngine::FRAMES_IN_FLIGHT as usize }>,
}

impl ExecutionFuture {
    pub(super) fn now(device: Arc<Device>) -> Self {
        Self {
            inner: Some(sync::now(device.clone()).boxed()),
            last_queue: None,
            fences: FrameArray::new(|| Fence::new(device.clone(), FenceCreateInfo {
                flags: FenceCreateFlags::SIGNALED,
                ..FenceCreateInfo::default()
            }).unwrap()),
        }
    }

    pub(super) fn join(&mut self, cb: Arc<PrimaryAutoCommandBuffer>, queue: &Queue) -> &mut Self {
        if let Some(last_queue) = &self.last_queue {
            if last_queue == queue {
                self.inner = Some(self.inner.take().unwrap().join(queue.execute(cb)).boxed());
            } //
            else {
                self.inner = Some(queue.then_execute(self.inner.take().unwrap(), cb));
                self.last_queue = Some(queue.clone());
            }
        } //
        else {
            self.inner = Some(self.inner.take().unwrap().join(queue.execute(cb)).boxed());
            self.last_queue = Some(queue.clone());
        }
        self
    }

    pub(super) fn then_execute(&mut self, cb: Arc<PrimaryAutoCommandBuffer>, queue: &Queue) -> &mut Self {
        self.inner = Some(queue.then_execute(self.inner.take().unwrap(), cb));
        self.last_queue = Some(queue.clone());
        self
    }

    pub(super) fn cleanup_finished(&mut self) {
        self.inner.as_mut().unwrap().cleanup_finished();
    }

    pub(super) fn join_future<F: GpuFuture + 'static>(&mut self, future: F) -> &mut Self {
        self.last_queue = future.queue().map(|q| q.into());
        self.inner = Some(self.inner.take().unwrap().join(future).boxed());
        self
    }

    pub(super) fn then_swapchain_present(&mut self, present_info: SwapchainPresentInfo, queue: &Queue) -> &mut Self {
        self.inner = Some(queue.swapchain_present(self.inner.take().unwrap(), present_info).boxed());
        self.last_queue = Some(queue.clone());
        self
    }

    pub(super) fn then_signal_fence_and_flush(&mut self) -> Result<(), Validated<VulkanError>> {
        self.inner = Some(self.inner.take().unwrap().then_signal_fence_and_flush()?.boxed());
        Ok(())
    }

    pub(super) fn then_signal_semaphore_and_flush(&mut self) -> Result<(), Validated<VulkanError>> {
        self.inner = Some(self.inner.take().unwrap().then_signal_semaphore_and_flush()?.boxed());
        Ok(())
    }
}