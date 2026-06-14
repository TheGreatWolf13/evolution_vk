use std::sync::Arc;
use vulkano::command_buffer::{CommandBufferExecFuture, PrimaryAutoCommandBuffer, PrimaryCommandBufferAbstract};
use vulkano::device::Queue as Q;
use vulkano::swapchain::{PresentFuture, SwapchainPresentInfo};
use vulkano::sync::future::NowFuture;
use vulkano::sync::GpuFuture;

pub(super) struct Queue {
    inner: Arc<Q>,
    ty: QueueType,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(super) enum QueueType {
    Graphics,
    Transfer,
    Dummy,
}

impl Queue {
    pub(super) fn new(mut queues: impl ExactSizeIterator<Item=Arc<Q>>) -> (Self, Self) {
        let g = queues.next().unwrap();
        let t = queues.next().unwrap_or_else(|| g.clone());
        let t = g.clone();
        // let t = g.clone();
        (
            Self {
                inner: g,
                ty: QueueType::Graphics,
            },
            Self {
                inner: t,
                ty: QueueType::Transfer,
            }
        )
    }

    pub(super) fn get_family_index(&self) -> u32 {
        self.inner.queue_family_index()
    }

    pub(super) fn get_type(&self) -> QueueType {
        self.ty
    }

    pub(super) fn execute(&self, cb: Arc<PrimaryAutoCommandBuffer>) -> CommandBufferExecFuture<NowFuture> {
        cb.execute(self.inner.clone()).unwrap()
    }

    pub(super) fn then_execute(&self, future: Box<dyn GpuFuture>, cb: Arc<PrimaryAutoCommandBuffer>) -> Box<dyn GpuFuture> {
        future.then_execute(self.inner.clone(), cb).unwrap().boxed()
    }

    pub(super) fn swapchain_present(&self, future: Box<dyn GpuFuture>, present_info: SwapchainPresentInfo) -> PresentFuture<Box<dyn GpuFuture>> {
        future.then_swapchain_present(self.inner.clone(), present_info)
    }
}