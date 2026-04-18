use crate::math::angle::AngleDeg;
use crate::math::mat4::Mat4;
use crate::math::vec2::Vec2;
use crate::math::vec3::Vec3;

pub struct Camera {
    pos: Vec3,
    view: Mat4,
    proj: Mat4,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: Vec3::ZERO,
            view: Mat4::IDENTITY,
            proj: Mat4::IDENTITY,
        }
    }

    pub fn adjust(&mut self, window_size: impl Into<Vec2>) {
        let window_size = window_size.into();
        self.view = Mat4::look_to_rh(self.pos, Vec3::Z, Vec3::Y);
        self.proj = Mat4::perspective(AngleDeg::new(60.0), window_size.x() / window_size.y(), 0.0625, 1024.0);
    }

    pub fn set_pos(&mut self, pos: impl Into<Vec3>) {
        self.pos = pos.into();
    }

    pub fn r#move(&mut self, delta: impl Into<Vec3>) {
        self.pos += delta.into();
    }

    pub fn get_view(&self) -> Mat4 {
        self.view
    }

    pub fn get_proj(&self) -> Mat4 {
        self.proj
    }
}