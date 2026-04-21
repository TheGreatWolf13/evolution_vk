use crate::math::mat4::Mat4;
use std::fmt::Debug;
use std::sync::Arc;
use vulkano::buffer::BufferContents;
use vulkano::device::Device;
use vulkano::pipeline::graphics::vertex_input::Vertex as VertexLayout;
use vulkano::shader::ShaderModule;

pub trait VertexFormat: BufferContents + VertexLayout + Copy + Debug {
    type UniformInput;
    type Uniform: BufferContents + Copy;

    fn load_shaders(device: Arc<Device>) -> (Arc<ShaderModule>, Arc<ShaderModule>);

    fn new_uniform(input: Self::UniformInput) -> Self::Uniform;
}

#[derive(Copy, Clone, Debug)]
pub struct Vertex;

#[derive(BufferContents, VertexLayout, Copy, Clone, Debug)]
#[repr(C)]
pub struct VertexPos {
    #[format(R32G32B32_SFLOAT)]
    position: [f32; 3],
}

#[derive(BufferContents, VertexLayout, Copy, Clone, Debug)]
#[repr(C)]
pub struct VertexPosCol {
    #[format(R32G32B32_SFLOAT)]
    position: [f32; 3],
    #[format(R32G32B32_SFLOAT)]
    color: [f32; 3],
}

#[derive(BufferContents, VertexLayout, Copy, Clone, Debug)]
#[repr(C)]
pub struct VertexPosTex {
    #[format(R32G32B32_SFLOAT)]
    position: [f32; 3],
    #[format(R32G32_SFLOAT)]
    uv: [f32; 2],
}

//Void

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

//Position

impl VertexPos {
    pub fn color(self, r: f32, g: f32, b: f32) -> VertexPosCol {
        VertexPosCol {
            position: self.position,
            color: [r, g, b],
        }
    }

    pub fn uv(self, u: f32, v: f32) -> VertexPosTex {
        VertexPosTex {
            position: self.position,
            uv: [u, v],
        }
    }
}

//Position
//Colour
impl VertexFormat for VertexPosCol {
    type UniformInput = (Mat4, Mat4, Mat4);
    type Uniform = vpc_vs::Transform;

    fn load_shaders(device: Arc<Device>) -> (Arc<ShaderModule>, Arc<ShaderModule>) {
        (vpc_vs::load(device.clone()).unwrap(), vpc_fs::load(device).unwrap())
    }

    //noinspection DuplicatedCode
    fn new_uniform(input: Self::UniformInput) -> Self::Uniform {
        Self::Uniform {
            world: input.0.into(),
            view: input.1.into(),
            proj: input.2.into(),
        }
    }
}

mod vpc_vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: r"
            #version 460

            layout(location = 0) in vec3 position;
            layout(location = 1) in vec3 color;

            layout(location = 0) out vec3 v_color;

            layout(set = 0, binding = 0) uniform Transform {
                mat4 world;
                mat4 view;
                mat4 proj;
            } uniforms;

            void main() {
                gl_Position = uniforms.proj * uniforms.view * uniforms.world * vec4(position, 1.0);
                v_color = color;
            }
        ",
    }
}

mod vpc_fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: r"
            #version 460

            layout(location = 0) in vec3 color;

            layout(location = 0) out vec4 f_color;

            void main() {
                f_color = vec4(color, 1.0);
            }
        ",
    }
}

//Position
//Texture

impl VertexFormat for VertexPosTex {
    type UniformInput = (Mat4, Mat4, Mat4);
    type Uniform = vpt_vs::Transform;

    fn load_shaders(device: Arc<Device>) -> (Arc<ShaderModule>, Arc<ShaderModule>) {
        (vpt_vs::load(device.clone()).unwrap(), vpt_fs::load(device).unwrap())
    }

    //noinspection DuplicatedCode
    fn new_uniform(input: Self::UniformInput) -> Self::Uniform {
        Self::Uniform {
            world: input.0.into(),
            view: input.1.into(),
            proj: input.2.into(),
        }
    }
}

mod vpt_vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: r"
            #version 460

            layout(location = 0) in vec3 position;
            layout(location = 1) in vec2 uv;

            layout(location = 0) out vec2 v_uv;

            layout(set = 0, binding = 0) uniform Transform {
                mat4 world;
                mat4 view;
                mat4 proj;
            } uniforms;

            void main() {
                gl_Position = uniforms.proj * uniforms.view * uniforms.world * vec4(position, 1.0);
                v_uv = uv;
            }
        ",
    }
}

mod vpt_fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: r"
            #version 460

            layout(location = 0) in vec2 uv;

            layout(location = 0) out vec4 f_color;

            layout(set = 0, binding = 1) uniform sampler s;
            layout(set = 0, binding = 2) uniform texture2D tex;

            void main() {
                f_color = texture(sampler2D(tex, s), uv);
            }
        ",
    }
}