use crate::client::camera::Camera;
use crate::if_else;
use crate::math::angle::{AngleDeg, Rot3Deg};
use enum_map::{enum_map, Enum, EnumMap};
use winit::event::{KeyEvent, MouseButton};
use winit::keyboard::{KeyCode, PhysicalKey};

pub struct Input {
    bindings: EnumMap<BindingType, Keybinding>,
}

#[derive(Copy, Clone, PartialEq, Eq, Enum)]
enum BindingType {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
    CameraLeft,
    CameraRight,
    CameraUp,
    CameraDown,
}

pub struct Keybinding {
    binding: Binding,
    is_down: bool,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Binding {
    Keyboard(PhysicalKey),
    Mouse(MouseButton),
}

impl PartialEq<PhysicalKey> for Binding {
    fn eq(&self, other: &PhysicalKey) -> bool {
        if let Binding::Keyboard(key) = self {
            key == other
        } //
        else {
            false
        }
    }
}

impl Keybinding {
    fn new(binding: impl Into<Binding>) -> Keybinding {
        let binding = binding.into();
        Self {
            binding,
            is_down: false,
        }
    }
}

impl From<KeyCode> for Binding {
    fn from(binding: KeyCode) -> Self {
        Binding::Keyboard(PhysicalKey::Code(binding))
    }
}

impl Input {
    pub fn new() -> Self {
        Self {
            bindings: enum_map! {
                BindingType::Forward => Keybinding::new(KeyCode::KeyW),
                BindingType::Backward => Keybinding::new(KeyCode::KeyS),
                BindingType::Left => Keybinding::new(KeyCode::KeyA),
                BindingType::Right => Keybinding::new(KeyCode::KeyD),
                BindingType::Up => Keybinding::new(KeyCode::Space),
                BindingType::Down => Keybinding::new(KeyCode::ControlLeft),
                BindingType::CameraLeft => Keybinding::new(KeyCode::KeyQ),
                BindingType::CameraRight => Keybinding::new(KeyCode::KeyE),
                BindingType::CameraUp => Keybinding::new(KeyCode::KeyR),
                BindingType::CameraDown => Keybinding::new(KeyCode::KeyF),
            },
        }
    }

    pub fn tick(&mut self, camera: &mut Camera) {
        const SPEED: f32 = 0.025;
        const SENSITIVITY: f32 = 2.0;
        let mut forward = if_else!(self.bindings[BindingType::Forward].is_down => 1.0 ; 0.0);
        forward += if_else!(self.bindings[BindingType::Backward].is_down => -1.0 ; 0.0);
        forward *= SPEED;
        let mut left = if_else!(self.bindings[BindingType::Left].is_down => 1.0 ; 0.0);
        left += if_else!(self.bindings[BindingType::Right].is_down => -1.0 ; 0.0);
        left *= SPEED;
        let mut up = if_else!(self.bindings[BindingType::Up].is_down => 1.0 ; 0.0);
        up += if_else!(self.bindings[BindingType::Down].is_down => -1.0 ; 0.0);
        up *= SPEED;
        camera.r#move((left, up, forward));
        let mut y_rot = if_else!(self.bindings[BindingType::CameraLeft].is_down => 1.0 ; 0.0);
        y_rot += if_else!(self.bindings[BindingType::CameraRight].is_down => -1.0 ; 0.0);
        y_rot *= SENSITIVITY;
        let mut x_rot = if_else!(self.bindings[BindingType::CameraUp].is_down => 1.0 ; 0.0);
        x_rot += if_else!(self.bindings[BindingType::CameraDown].is_down => -1.0 ; 0.0);
        x_rot *= SENSITIVITY;
        camera.rotate(Rot3Deg::new(AngleDeg::new(x_rot), AngleDeg::new(y_rot), AngleDeg::ZERO));
    }

    pub fn process_input(&mut self, event: KeyEvent) {
        self.bindings.values_mut().filter(|b| b.binding == event.physical_key).for_each(|b| b.is_down = event.state.is_pressed());
    }
}