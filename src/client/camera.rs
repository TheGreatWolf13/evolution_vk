use crate::math::angle::{Angle, AngleDeg, Rot3};
use crate::math::lerp::{LerpMode, PaP};
use crate::math::mat4::Mat4F32;
use crate::math::vec2f::Vec2F32;
use crate::math::vec3f::Vec3F32;
use crate::math::Vector2;

pub struct Camera {
    pos: PaP<Vec3F32>,
    rot: PaP<Rot3<AngleDeg>>,
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
            pos: PaP::new(Vec3F32::new(0.0, 0.0, 2.0)),
            rot: PaP::new(Rot3::zero()),
            view: Mat4F32::IDENTITY,
            proj: Mat4F32::IDENTITY,
        }
    }

    pub fn adjust(&mut self, window_size: impl Into<Vec2F32>, partial_tick: f32) {
        let window_size = window_size.into();
        let quat = self.rot.get(partial_tick).to_quat();
        self.view = Mat4F32::look_to(self.pos.get(partial_tick), quat * Vec3F32::Y, quat * Vec3F32::Z);
        self.proj = Mat4F32::perspective(AngleDeg::new(60.0), window_size.x() / window_size.y(), 0.0625, 1024.0);
    }

    pub fn r#move(&mut self, delta: impl Into<Vec3F32>) {
        let quat = self.rot.get_present().to_quat();
        self.pos.update(self.pos.get_present() + quat * delta.into(), LerpMode::Interpolate);
    }

    pub fn rotate<A: Angle>(&mut self, angle_deltas: (A, A, A)) {
        let mut rot = self.rot.get_present();
        *rot.x_mut() += angle_deltas.0.to_degrees();
        *rot.y_mut() += angle_deltas.1.to_degrees();
        *rot.z_mut() += angle_deltas.2.to_degrees();
        self.rotate_to(rot, LerpMode::Interpolate);
    }

    pub fn rotate_to(&mut self, rot: Rot3<AngleDeg>, lerp_mode: LerpMode) {
        self.rot.update(rot, lerp_mode);
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

    pub fn get_pos(&self, partial_tick: f32) -> Vec3F32 {
        self.pos.get(partial_tick)
    }
}