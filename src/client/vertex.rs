use vulkano::buffer::BufferContents;
use vulkano::pipeline::graphics::vertex_input::Vertex as VertexLayout;

#[derive(Copy, Clone)]
pub struct Vertex;

#[derive(BufferContents, VertexLayout, Copy, Clone)]
#[repr(C)]
pub struct VertexPos {
    #[format(R32G32B32_SFLOAT)]
    position: [f32; 3],
}

#[derive(BufferContents, VertexLayout, Copy, Clone)]
#[repr(C)]
pub struct VertexPosCol {
    #[format(R32G32B32_SFLOAT)]
    position: [f32; 3],
    #[format(R32G32B32_SFLOAT)]
    color: [f32; 3],
}

impl Vertex {
    pub fn new() -> Self {
        Self
    }

    pub fn pos(self, x: f32, y: f32, z: f32) -> VertexPos {
        VertexPos {
            position: [x, y, z]
        }
    }
}

impl VertexPos {
    pub fn color(self, r: f32, g: f32, b: f32) -> VertexPosCol {
        VertexPosCol {
            position: self.position,
            color: [r, g, b],
        }
    }
}