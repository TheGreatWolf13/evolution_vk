use crate::math::mat4::Mat4;
use std::fmt::Debug;
use std::sync::Arc;
use vulkano::buffer::BufferContents;
use vulkano::device::Device;
use vulkano::pipeline::graphics::vertex_input::Vertex as VertexLayout;
use vulkano::shader::ShaderModule;

pub trait VertexFormat: BufferContents + VertexLayout + Copy + Debug {
    type PushConstantInput: Into<Self::PushConstant> + Transform;
    type PushConstant: BufferContents + Copy;
    type UniformInput: Into<Self::Uniform>;
    type Uniform: BufferContents + Copy;

    fn load_shaders(device: Arc<Device>) -> (Arc<ShaderModule>, Arc<ShaderModule>);

    fn transform_and_untransform(&self, transform: Self::PushConstantInput, untransform: Self::PushConstantInput) -> Self;

    fn transform(&self, transform: Self::PushConstantInput) -> Self;
}

pub trait Transform: Copy {
    fn identity() -> Self;
}

impl Transform for Mat4 {
    fn identity() -> Self {
        Mat4::IDENTITY
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vertex;

#[derive(BufferContents, VertexLayout, Copy, Clone, Debug)]
#[repr(C)]
pub struct VertexPos {
    #[format(R32G32B32_SFLOAT)]
    pos: [f32; 3],
}

#[derive(BufferContents, VertexLayout, Copy, Clone, Debug)]
#[repr(C)]
pub struct VertexPosCol {
    #[format(R32G32B32_SFLOAT)]
    pos: [f32; 3],
    #[format(R32G32B32_SFLOAT)]
    color: [f32; 3],
}

#[derive(BufferContents, VertexLayout, Copy, Clone, Debug)]
#[repr(C)]
pub struct VertexPosTex {
    #[format(R32G32B32_SFLOAT)]
    pos: [f32; 3],
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
            pos: [x, y, z]
        }
    }
}

//Position

impl VertexPos {
    pub fn color(self, r: f32, g: f32, b: f32) -> VertexPosCol {
        VertexPosCol {
            pos: self.pos,
            color: [r, g, b],
        }
    }

    pub fn uv(self, u: f32, v: f32) -> VertexPosTex {
        VertexPosTex {
            pos: self.pos,
            uv: [u, v],
        }
    }
}

mod vpc {
    use crate::client::vertex::{VertexFormat, VertexPosCol};
    use crate::math::mat4::Mat4;
    use crate::math::vec3::Vec3;
    use std::sync::Arc;
    use vulkano::device::Device;
    use vulkano::shader::ShaderModule;

    impl VertexFormat for VertexPosCol {
        type PushConstantInput = Mat4;
        type PushConstant = vs::Transform;
        type UniformInput = (Mat4, Mat4);
        type Uniform = vs::Camera;

        fn load_shaders(device: Arc<Device>) -> (Arc<ShaderModule>, Arc<ShaderModule>) {
            (vs::load(device.clone()).unwrap(), fs::load(device).unwrap())
        }

        fn transform_and_untransform(&self, transform: Self::PushConstantInput, untransform: Self::PushConstantInput) -> Self {
            Self {
                pos: (untransform.inverse() * transform).transform(Vec3::from(self.pos)).into(),
                color: self.color,
            }
        }

        fn transform(&self, transform: Self::PushConstantInput) -> Self {
            Self {
                pos: transform.transform(Vec3::from(self.pos)).into(),
                color: self.color,
            }
        }
    }

    impl Into<vs::Transform> for Mat4 {
        fn into(self) -> vs::Transform {
            vs::Transform {
                world: self.into()
            }
        }
    }

    impl Into<vs::Camera> for (Mat4, Mat4) {
        fn into(self) -> vs::Camera {
            vs::Camera {
                view: self.0.into(),
                proj: self.1.into(),
            }
        }
    }

    mod vs {
        vulkano_shaders::shader! {
        ty: "vertex",
        src: r"
            #version 460

            layout(set = 0, binding = 0) uniform Camera {
                mat4 view;
                mat4 proj;
            } camera;

            layout(push_constant) uniform Transform {
                mat4 world;
            } transform;

            layout(location = 0) in vec3 pos;
            layout(location = 1) in vec3 color;

            layout(location = 0) out vec3 v_color;

            void main() {
                gl_Position = camera.proj * camera.view * transform.world * vec4(pos, 1.0);
                v_color = color;
            }
        ",
    }
    }

    mod fs {
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
}

mod vpt {
    use crate::client::vertex::{VertexFormat, VertexPosTex};
    use crate::math::mat4::Mat4;
    use crate::math::vec3::Vec3;
    use std::sync::Arc;
    use vulkano::device::Device;
    use vulkano::shader::ShaderModule;

    impl VertexFormat for VertexPosTex {
        type PushConstantInput = Mat4;
        type PushConstant = vs::Transform;
        type UniformInput = (Mat4, Mat4);
        type Uniform = vs::Camera;

        fn load_shaders(device: Arc<Device>) -> (Arc<ShaderModule>, Arc<ShaderModule>) {
            (vs::load(device.clone()).unwrap(), fs::load(device).unwrap())
        }

        fn transform_and_untransform(&self, transform: Self::PushConstantInput, untransform: Self::PushConstantInput) -> Self {
            Self {
                pos: (untransform.inverse() * transform).transform(Vec3::from(self.pos)).into(),
                uv: self.uv,
            }
        }

        fn transform(&self, transform: Self::PushConstantInput) -> Self {
            Self {
                pos: transform.transform(Vec3::from(self.pos)).into(),
                uv: self.uv,
            }
        }
    }

    impl Into<vs::Transform> for Mat4 {
        fn into(self) -> vs::Transform {
            vs::Transform {
                world: self.into(),
            }
        }
    }

    impl Into<vs::Camera> for (Mat4, Mat4) {
        fn into(self) -> vs::Camera {
            vs::Camera {
                view: self.0.into(),
                proj: self.1.into(),
            }
        }
    }

    mod vs {
        vulkano_shaders::shader! {
        ty: "vertex",
        src: r"
            #version 460

            layout(set = 0, binding = 0) uniform Camera {
                mat4 view;
                mat4 proj;
            } camera;

            layout(push_constant) uniform Transform {
                mat4 world;
            } transform;

            layout(location = 0) in vec3 pos;
            layout(location = 1) in vec2 uv;

            layout(location = 0) out vec2 v_uv;

            void main() {
                gl_Position = camera.proj * camera.view * transform.world * vec4(pos, 1.0);
                v_uv = uv;
            }
        ",
    }
    }

    mod fs {
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
}