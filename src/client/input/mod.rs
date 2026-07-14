pub mod keybinding;

use crate::client::camera::Camera;
use crate::client::input::keybinding::{BindingType, Keybinding};
use crate::if_else;
use crate::math::angle::AngleDeg;
use crate::math::vec3f::Vec3F32;
use crate::math::Vector3;
use enum_map::{enum_map, EnumMap};
use winit::event::{ElementState, KeyEvent, MouseButton};
use winit::keyboard::KeyCode;

pub struct Input {
    bindings: EnumMap<BindingType, Keybinding>,
    rot: Vec3F32,
}

pub trait InputHandler {
    fn toggle_grab_mouse(&mut self);

    fn toggle_wireframe(&mut self);
}

impl Input {
    pub fn new() -> Self {
        Self {
            bindings: enum_map! {
                BindingType::MoveForward => Keybinding::new(KeyCode::KeyW),
                BindingType::MoveBackward => Keybinding::new(KeyCode::KeyS),
                BindingType::MoveLeft => Keybinding::new(KeyCode::KeyA),
                BindingType::MoveRight => Keybinding::new(KeyCode::KeyD),
                BindingType::MoveUp => Keybinding::new(KeyCode::Space),
                BindingType::MoveDown => Keybinding::new(KeyCode::ControlLeft),
                BindingType::ToggleGrabMouse => Keybinding::new(KeyCode::AltLeft),
                BindingType::ToggleWireframe => Keybinding::new(KeyCode::F6),
            },
            rot: Vec3F32::ZERO,
        }
    }

    pub fn tick(&mut self, camera: &mut Camera, handler: &mut impl InputHandler) {
        const SPEED: f32 = 0.1;
        const SENSITIVITY: f32 = 0.25;
        let mut move_y = if_else!(self.bindings[BindingType::MoveForward].is_down_and_reset() => 1.0 ; 0.0);
        move_y += if_else!(self.bindings[BindingType::MoveBackward].is_down_and_reset() => -1.0 ; 0.0);
        move_y *= SPEED;
        let mut move_x = if_else!(self.bindings[BindingType::MoveRight].is_down_and_reset() => 1.0 ; 0.0);
        move_x += if_else!(self.bindings[BindingType::MoveLeft].is_down_and_reset() => -1.0 ; 0.0);
        move_x *= SPEED;
        let mut move_z = if_else!(self.bindings[BindingType::MoveUp].is_down_and_reset() => 1.0 ; 0.0);
        move_z += if_else!(self.bindings[BindingType::MoveDown].is_down_and_reset() => -1.0 ; 0.0);
        move_z *= SPEED;
        camera.r#move((move_x, move_y, move_z));
        self.rot *= SENSITIVITY;
        camera.rotate((AngleDeg::new(self.rot.x()), AngleDeg::new(self.rot.y()), AngleDeg::new(self.rot.z())));
        self.rot = Vec3F32::ZERO;
        while self.bindings[BindingType::ToggleGrabMouse].consume_click() {
            handler.toggle_grab_mouse();
        }
        while self.bindings[BindingType::ToggleWireframe].consume_click() {
            handler.toggle_wireframe();
        }
    }

    pub fn process_input(&mut self, event: KeyEvent) {
        self.bindings.values_mut().filter(|b| b.is_bound_to_key(event.physical_key)).for_each(|b| if_else!(event.state.is_pressed() => b.press() ; b.release()));
    }

    pub fn process_mouse_button(&mut self, button: MouseButton, state: ElementState) {
        self.bindings.values_mut().filter(|b| b.is_bound_to_mouse(button)).for_each(|b| if_else!(state.is_pressed() => b.press() ; b.release()));
    }

    pub fn process_mouse_motion(&mut self, delta: (f64, f64)) {
        *self.rot.z_mut() -= delta.0 as f32;
        *self.rot.x_mut() -= delta.1 as f32;
    }
}