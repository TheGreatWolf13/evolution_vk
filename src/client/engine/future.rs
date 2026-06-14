use crate::client::engine::queue::{Queue, QueueType};
use std::sync::Arc;
use vulkano::command_buffer::PrimaryAutoCommandBuffer;
use vulkano::device::Device;
use vulkano::swapchain::SwapchainPresentInfo;
use vulkano::sync::GpuFuture;
use vulkano::{sync, Validated, VulkanError};

pub(super) struct ExecutionFuture {
    inner: Option<Box<dyn GpuFuture>>,
    current_queue: QueueType,
}

impl ExecutionFuture {
    pub(super) fn now(device: Arc<Device>) -> Self {
        // info!("Now {:?}", QueueType::Dummy);
        Self {
            inner: Some(sync::now(device.clone()).boxed()),
            current_queue: QueueType::Dummy,
        }
    }

    pub(super) fn join(&mut self, cb: Arc<PrimaryAutoCommandBuffer>, queue: &Queue) -> &mut Self {
        assert!(self.inner.is_some());
        if self.current_queue == QueueType::Dummy {
            // info!("Join {:?} -> {:?}", self.current_queue, queue.get_type());
            self.inner = Some(self.inner.take().unwrap().join(queue.execute(cb)).boxed());
            self.current_queue = queue.get_type();
        } //
        else if self.current_queue == queue.get_type() {
            // info!("Join {:?} -> {:?}", self.current_queue, queue.get_type());
            self.inner = Some(self.inner.take().unwrap().join(queue.execute(cb)).boxed());
        } //
        else {
            // info!("Then exec + semaphore + flush {:?} -> {:?}", self.current_queue, queue.get_type());
            self.inner = Some(queue.then_execute(self.inner.take().unwrap().then_signal_semaphore_and_flush().unwrap().boxed(), cb));
            self.current_queue = queue.get_type();
        }
        assert!(self.inner.is_some());
        self
    }

    pub(super) fn then_execute(&mut self, cb: Arc<PrimaryAutoCommandBuffer>, queue: &Queue) -> &mut Self {
        assert!(self.inner.is_some());
        if self.current_queue != queue.get_type() {
            // info!("Then exec + semaphore + flush {:?} -> {:?}", self.current_queue, queue.get_type());
            self.current_queue = queue.get_type();
            self.inner = Some(queue.then_execute(self.inner.take().unwrap().then_signal_semaphore_and_flush().unwrap().boxed(), cb));
        } //
        else {
            // info!("Then exec {:?} -> {:?}", self.current_queue, queue.get_type());
            self.inner = Some(queue.then_execute(self.inner.take().unwrap(), cb));
        }
        assert!(self.inner.is_some());
        self
    }

    pub(super) fn cleanup_finished(&mut self) {
        assert!(self.inner.is_some());
        self.inner.as_mut().unwrap().cleanup_finished();
        assert!(self.inner.is_some());
    }

    pub(super) fn join_future<F: GpuFuture + 'static>(&mut self, future: F) -> &mut Self {
        assert!(self.inner.is_some());
        self.inner = Some(self.inner.take().unwrap().join(future).boxed());
        assert!(self.inner.is_some());
        self
    }

    pub(super) fn then_swapchain_present(&mut self, present_info: SwapchainPresentInfo, queue: &Queue) -> &mut Self {
        assert!(self.inner.is_some());
        // info!("swapchain_present {:?} -> {:?}", self.current_queue, queue.get_type());
        self.inner = Some(queue.swapchain_present(self.inner.take().unwrap(), present_info).boxed());
        self.current_queue = queue.get_type();
        assert!(self.inner.is_some());
        self
    }

    pub(super) fn then_signal_fence_and_flush(&mut self) -> Result<(), Validated<VulkanError>> {
        assert!(self.inner.is_some());
        self.inner = Some(self.inner.take().unwrap().then_signal_fence_and_flush()?.boxed());
        // info!("Then signal_fence_and_flush {:?}", self.current_queue);
        assert!(self.inner.is_some());
        Ok(())
    }
}