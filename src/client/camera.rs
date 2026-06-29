use crate::math::angle::{AngleDeg, Rot3Deg};
use crate::math::mat4::Mat4F32;
use crate::math::vec2f::Vec2F32;
use crate::math::vec3f::Vec3F32;
use crate::math::{PaP, Vector2};

pub struct Camera {
    pos: PaP<Vec3F32>,
    rot: PaP<Rot3Deg>,
    view: Mat4F32,
    proj: Mat4F32,
}

#[derive(Copy, Clone)]
pub struct CameraUniform {
    pub view: Mat4F32,
    pub proj: Mat4F32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: PaP::new(Vec3F32::new(0.0, 2.0, 0.0)),
            rot: PaP::new(Rot3Deg::ZERO),
            view: Mat4F32::IDENTITY,
            proj: Mat4F32::IDENTITY,
        }
    }

    pub fn adjust(&mut self, window_size: impl Into<Vec2F32>, partial_tick: f32) {
        let window_size = window_size.into();
        let quat = self.rot.lerp(partial_tick).to_quat();
        self.view = Mat4F32::look_to(self.pos.lerp(partial_tick), quat * -Vec3F32::Z, quat * Vec3F32::Y);
        self.proj = Mat4F32::perspective(AngleDeg::new(60.0), window_size.x() / window_size.y(), 0.0625, 1024.0);
    }

    pub fn r#move(&mut self, delta: impl Into<Vec3F32>) {
        self.pos.0 = self.pos.1;
        let quat = self.rot.1.to_quat();
        self.pos.1 += quat * delta.into();
    }

    pub fn rotate(&mut self, rot: Rot3Deg) {
        self.rot.0 = self.rot.1;
        self.rot.1 += rot;
    }

    pub fn get_view(&self) -> Mat4F32 {
        self.view
    }

    pub fn get_proj(&self) -> Mat4F32 {
        self.proj
    }

    pub fn get_uniform(&self) -> CameraUniform {
        CameraUniform {
            view: self.view,
            proj: self.proj,
        }
    }

    pub fn get_pos(&self) -> Vec3F32 {
        self.pos.1
    }
}