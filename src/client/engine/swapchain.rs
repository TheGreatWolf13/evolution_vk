use std::sync::Arc;
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::Device;
use vulkano::format::Format;
use vulkano::image::view::ImageView;
use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage};
use vulkano::memory::allocator::{AllocationCreateInfo, StandardMemoryAllocator};
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass};
use vulkano::swapchain::{PresentMode, Surface, Swapchain, SwapchainAcquireFuture, SwapchainCreateInfo, SwapchainPresentInfo};
use vulkano::{swapchain, Validated, VulkanError};
use winit::dpi::PhysicalSize;

pub(super) struct SwapChain {
    swapchain: Arc<Swapchain>,
    framebuffers: FrameVec<Arc<Framebuffer>>,
    image_index: u32,
    needs_recreate: bool,
}

pub(super) struct FrameVec<T>(Vec<T>);

impl<T> FrameVec<T> {
    pub fn get(&self, swap: &SwapChain) -> &T {
        &self.0[swap.image_index as usize]
    }

    pub fn create_new_attached<U>(&self, f: impl FnMut(&T) -> U) -> FrameVec<U> {
        FrameVec(self.0.iter().map(f).collect::<Vec<_>>())
    }
}

impl SwapChain {
    pub fn new(window_size: PhysicalSize<u32>, image_format: Format, physical_device: Arc<PhysicalDevice>, device: Arc<Device>, surface: Arc<Surface>, render_pass: Arc<RenderPass>, allocator: Arc<StandardMemoryAllocator>) -> Self {
        let caps = physical_device.surface_capabilities(&surface, Default::default()).expect("failed to get surface capabilities");
        let composite_alpha = caps.supported_composite_alpha.into_iter().next().unwrap();
        let (swapchain, images) = Swapchain::new(
            device.clone(),
            surface,
            SwapchainCreateInfo {
                min_image_count: caps.min_image_count,
                image_format,
                image_extent: window_size.into(),
                image_usage: ImageUsage::COLOR_ATTACHMENT,
                composite_alpha,
                present_mode: PresentMode::Immediate,
                ..Default::default()
            },
        ).unwrap();
        let framebuffers = Self::create_framebuffers(images, render_pass, allocator);
        Self {
            swapchain,
            framebuffers,
            image_index: 0,
            needs_recreate: false,
        }
    }

    pub fn swap_buffers(&mut self, after_swap: impl FnOnce(&mut Self, SwapchainAcquireFuture, Arc<Framebuffer>, SwapchainPresentInfo)) {
        let (image_index, suboptimal, acquire_future) = match swapchain::acquire_next_image(self.swapchain.clone(), None).map_err(Validated::unwrap) {
            Ok(r) => r,
            Err(VulkanError::OutOfDate) => {
                self.needs_recreate = true;
                return;
            }
            Err(e) => panic!("failed to acquire next image: {e}"),
        };
        self.image_index = image_index;
        if suboptimal {
            self.needs_recreate = true;
        }
        after_swap(
            self,
            acquire_future,
            self.framebuffers.get(self).clone(),
            SwapchainPresentInfo::swapchain_image_index(self.swapchain.clone(), image_index),
        );
    }

    pub fn needs_recreate(&self) -> bool {
        self.needs_recreate
    }

    pub fn set_needs_recreate(&mut self) {
        self.needs_recreate = true;
    }

    pub fn recreate(&mut self, new_dimensions: PhysicalSize<u32>, render_pass: Arc<RenderPass>, allocator: Arc<StandardMemoryAllocator>) {
        let (swapchain, images) = self.swapchain.recreate(SwapchainCreateInfo {
            image_extent: new_dimensions.into(),
            ..self.swapchain.create_info()
        }).unwrap();
        self.swapchain = swapchain;
        self.framebuffers = Self::create_framebuffers(images, render_pass, allocator);
        self.needs_recreate = false;
    }

    pub fn create_framevec<T>(&self, mut f: impl FnMut() -> T) -> FrameVec<T> {
        FrameVec((0..self.swapchain.image_count()).map(|_| f()).collect::<Vec<_>>())
    }

    fn create_framebuffers(images: Vec<Arc<Image>>, render_pass: Arc<RenderPass>, allocator: Arc<StandardMemoryAllocator>) -> FrameVec<Arc<Framebuffer>> {
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
        FrameVec(images.iter().map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(render_pass.clone(), FramebufferCreateInfo {
                attachments: vec![view, depth_buffer.clone()],
                ..Default::default()
            }).unwrap()
        }).collect())
    }
}