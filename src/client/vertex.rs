use crate::math::mat4::Mat4;
use std::fmt::Debug;
use std::sync::Arc;
use vulkano::buffer::BufferContents;
use vulkano::device::Device;
use vulkano::pipeline::graphics::vertex_input::Vertex as VertexLayout;
use vulkano::shader::ShaderModule;

pub trait VertexFormat: BufferContents + VertexLayout + Copy + Debug {
    type SSBOType: BufferContents + Copy;
    type Uniform: BufferContents + Copy;

    fn load_shaders(device: Arc<Device>) -> (Arc<ShaderModule>, Arc<ShaderModule>);

    fn transform(self, matrix: Mat4) -> Self;
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

    pub fn pos(self, pos: impl Into<[f32; 3]>) -> VertexPos {
        VertexPos {
            pos: pos.into()
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

    pub fn uv(self, uv: impl Into<[f32; 2]>) -> VertexPosTex {
        VertexPosTex {
            pos: self.pos,
            uv: uv.into(),
        }
    }
}

mod vpc {
    use crate::client::camera::CameraUniform;
    use crate::client::vertex::{VertexFormat, VertexPosCol};
    use crate::math::mat4::Mat4;
    use std::sync::Arc;
    use vulkano::device::Device;
    use vulkano::shader::ShaderModule;

    impl VertexFormat for VertexPosCol {
        type SSBOType = vs::Transform;
        type Uniform = vs::Camera;

        fn load_shaders(device: Arc<Device>) -> (Arc<ShaderModule>, Arc<ShaderModule>) {
            (vs::load(device.clone()).unwrap(), fs::load(device).unwrap())
        }

        fn transform(self, matrix: Mat4) -> Self {
            Self {
                pos: matrix.transform(self.pos.into()).into(),
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

    impl Into<vs::Camera> for CameraUniform {
        fn into(self) -> vs::Camera {
            vs::Camera {
                view: self.view.into(),
                proj: self.proj.into(),
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
    use crate::client::camera::CameraUniform;
    use crate::client::vertex::{VertexFormat, VertexPosTex};
    use crate::math::mat4::Mat4;
    use std::sync::Arc;
    use vulkano::device::Device;
    use vulkano::shader::ShaderModule;

    impl VertexFormat for VertexPosTex {
        type SSBOType = vs::Transform;
        type Uniform = vs::Camera;

        fn load_shaders(device: Arc<Device>) -> (Arc<ShaderModule>, Arc<ShaderModule>) {
            (vs::load(device.clone()).unwrap(), fs::load(device).unwrap())
        }

        fn transform(self, matrix: Mat4) -> Self {
            Self {
                pos: matrix.transform(self.pos.into()).into(),
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

    impl Into<vs::Camera> for CameraUniform {
        fn into(self) -> vs::Camera {
            vs::Camera {
                view: self.view.into(),
                proj: self.proj.into(),
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

                //layout(push_constant) uniform Transform {
                //    mat4 world;
                //} transform;

                struct Transform {
                    mat4 world;
                };

                layout(set = 1, binding = 0) readonly buffer TransformBuffer {
                    Transform[] data;
                } transforms;

                layout(location = 0) in vec3 pos;
                layout(location = 1) in vec2 uv;

                layout(location = 0) out vec2 v_uv;

                void main() {
                    gl_Position = camera.proj * camera.view * transforms.data[gl_InstanceIndex].world * vec4(pos, 1.0);
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