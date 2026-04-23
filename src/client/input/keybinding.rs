use enum_map::Enum;
use winit::event::MouseButton;
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Copy, Clone, PartialEq, Eq, Enum)]
pub(super) enum BindingType {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    ToggleGrabMouse,
    ToggleWireframe,
}

pub struct Keybinding {
    binding: Binding,
    click_count: u16,
    is_down: bool,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub(super) enum Binding {
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

impl PartialEq<MouseButton> for Binding {
    fn eq(&self, other: &MouseButton) -> bool {
        if let Binding::Mouse(button) = self {
            button == other
        } //
        else {
            false
        }
    }
}

impl Keybinding {
    pub(super) fn new(binding: impl Into<Binding>) -> Keybinding {
        let binding = binding.into();
        Self {
            binding,
            click_count: 0,
            is_down: false,
        }
    }

    pub(super) fn consume_click(&mut self) -> bool {
        if self.click_count > 0 {
            self.click_count -= 1;
            return true;
        }
        false
    }

    pub(super) fn consume_all_clicks(&mut self) -> bool {
        if self.click_count > 0 {
            self.click_count = 0;
            return true;
        }
        false
    }

    pub fn is_bound_to_key(&self, physical_key: PhysicalKey) -> bool {
        self.binding == physical_key
    }

    pub fn is_bound_to_mouse(&self, mouse_button: MouseButton) -> bool {
        self.binding == mouse_button
    }

    pub fn is_down(&self) -> bool {
        self.is_down
    }

    #[allow(clippy::wrong_self_convention)]
    pub(super) fn is_down_and_reset(&mut self) -> bool {
        self.click_count = 0;
        self.is_down
    }

    pub(super) fn press(&mut self) {
        self.click_count += 1;
        self.is_down = true;
    }

    pub(super) fn release(&mut self) {
        self.is_down = false;
    }
}

impl From<KeyCode> for Binding {
    fn from(binding: KeyCode) -> Self {
        Binding::Keyboard(PhysicalKey::Code(binding))
    }
}