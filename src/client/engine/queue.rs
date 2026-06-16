use std::sync::Arc;
use vulkano::command_buffer::{CommandBufferExecFuture, PrimaryAutoCommandBuffer, PrimaryCommandBufferAbstract};
use vulkano::device::Queue as Q;
use vulkano::swapchain::{PresentFuture, SwapchainPresentInfo};
use vulkano::sync::future::NowFuture;
use vulkano::sync::GpuFuture;

#[derive(Clone)]
pub(super) struct Queue {
    inner: Arc<Q>,
}

impl Queue {
    pub(super) fn new(mut queues: impl ExactSizeIterator<Item = Arc<Q>>) -> (Self, Self) {
        let g = queues.next().unwrap();
        let t = queues.next().unwrap_or_else(|| g.clone());
        let t = g.clone();
        // let t = g.clone();
        (
            Self {
                inner: g,
            },
            Self {
                inner: t,
            }
        )
    }

    pub(super) fn get_family_index(&self) -> u32 {
        self.inner.queue_family_index()
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

impl From<Arc<Q>> for Queue {
    fn from(value: Arc<Q>) -> Self {
        Self {
            inner: value.clone(),
        }
    }
}

impl PartialEq for Queue {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for Queue {}