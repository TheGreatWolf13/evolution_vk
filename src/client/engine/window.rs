use crate::math::vec2u::Vec2U32;
use crate::math::Vector2;

pub(super) struct WindowParams {
    size: Vec2U32,
    is_minimized: bool,
    should_resize: bool,
    window_focused: bool,
}

impl WindowParams {
    pub(super) fn new(size: impl Into<Vec2U32>) -> Self {
        Self {
            size: size.into(),
            is_minimized: false,
            should_resize: false,
            window_focused: true,
        }
    }

    pub(super) fn changed_size(&mut self, size: impl Into<Vec2U32>) {
        let size = size.into();
        if size.x() == 0 || size.y() == 0 {
            self.is_minimized = true;
        } //
        else {
            self.is_minimized = false;
            if size != self.size {
                self.size = size;
                self.should_resize = true;
            }
        }
    }

    pub(super) fn is_window_focused(&self) -> bool {
        self.window_focused
    }

    pub(super) fn is_window_minimized(&self) -> bool {
        self.is_minimized
    }

    pub(super) fn should_resize(&self) -> bool {
        self.should_resize && !self.is_minimized
    }

    pub(super) fn set_resized(&mut self) {
        self.should_resize = false;
    }

    pub(super) fn set_focused(&mut self, focused: bool) {
        self.window_focused = focused;
    }
}