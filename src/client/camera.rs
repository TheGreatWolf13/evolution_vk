use crate::math::angle::{AngleDeg, Rot3Deg};
use crate::math::mat4::Mat4;
use crate::math::vec2::Vec2;
use crate::math::vec3::Vec3;
use crate::math::PaP;

pub struct Camera {
    pos: PaP<Vec3>,
    rot: PaP<Rot3Deg>,
    view: Mat4,
    proj: Mat4,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: PaP::new(Vec3::ZERO),
            rot: PaP::new(Rot3Deg::ZERO),
            view: Mat4::IDENTITY,
            proj: Mat4::IDENTITY,
        }
    }

    pub fn adjust(&mut self, window_size: impl Into<Vec2>, partial_tick: f32) {
        let window_size = window_size.into();
        let quat = self.rot.lerp(partial_tick).to_quat();
        self.view = Mat4::look_to(self.pos.lerp(partial_tick), quat * -Vec3::Z, quat * Vec3::Y);
        self.proj = Mat4::perspective(AngleDeg::new(60.0), window_size.x() / window_size.y(), 0.0625, 1024.0);
    }

    pub fn r#move(&mut self, delta: impl Into<Vec3>) {
        self.pos.0 = self.pos.1;
        let quat = self.rot.1.to_quat();
        self.pos.1 += quat * delta.into();
    }

    pub fn rotate(&mut self, rot: Rot3Deg) {
        self.rot.0 = self.rot.1;
        self.rot.1 += rot;
    }

    pub fn get_view(&self) -> Mat4 {
        self.view
    }

    pub fn get_proj(&self) -> Mat4 {
        self.proj
    }
}