use crate::client::texture::atlas::{RawTextureInfo, TextureId, TextureInfo};
use crate::math::uvec2::UVec2;
use crate::math::vec2::Vec2;

#[derive(Copy, Clone)]
struct Rect(UVec2, UVec2, u32);

impl Rect {
    const fn width(&self) -> u32 {
        self.1.x() - self.0.x()
    }

    const fn height(&self) -> u32 {
        self.1.y() - self.0.y()
    }

    fn try_merge(&self, other: &Self) -> Option<Self> {
        if self.0.x() == other.0.x() && self.width() == other.width() && self.1.y() == other.0.y() {
            Some(Rect(self.0, self.1 + (0, other.height()), self.2))
        } //
        else if self.0.y() == other.0.y() && self.height() == other.height() && self.1.x() == other.0.x() {
            Some(Rect(self.0, self.1 + (other.width(), 0), self.2))
        } //
        else {
            None
        }
    }
}

impl Eq for Rect {}

impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.2 == other.2
    }
}

pub(super) struct UVConstructor {
    size: UVec2,
}

impl UVConstructor {
    pub fn get_uv(&self, id: TextureId, info: RawTextureInfo) -> TextureInfo {
        TextureInfo::new(
            id,
            Vec2::new(info.0.x() as f32 / self.size.x() as f32, info.0.y() as f32 / self.size.y() as f32),
            Vec2::new(info.1.x() as f32 / self.size.x() as f32, info.1.y() as f32 / self.size.y() as f32),
        )
    }

    pub fn get_size(&self) -> UVec2 {
        self.size
    }
}

pub(super) struct TexturePlacer {
    size: UVec2,
    free_rects: Vec<Rect>,
    last_id: u32,
}

impl TexturePlacer {
    pub fn new(starting_size: impl Into<UVec2>) -> Self {
        let starting_size = starting_size.into();
        if starting_size.x() & 1 != 0 || starting_size.y() & 1 != 0 {
            panic!("Atlas cannot have non power of two dimensions!");
        }
        Self {
            size: starting_size,
            free_rects: vec![Rect(UVec2::ZERO, starting_size, 0)],
            last_id: 0,
        }
    }

    fn next_id(&mut self) -> u32 {
        self.last_id += 1;
        self.last_id
    }

    pub fn place(&mut self, size: impl Into<UVec2>) -> RawTextureInfo {
        let size = size.into();
        loop {
            if let Some(rect) = self.find_best_rect(size) {
                self.free_rects.retain(|r| r != &rect);
                self.split_rect(rect, size);
                self.merge_free_rects();
                break RawTextureInfo(rect.0, rect.0 + size);
            } //
            else {
                self.duplicate_size();
            }
        }
    }

    pub fn get_uv(self) -> UVConstructor {
        UVConstructor {
            size: self.size,
        }
    }

    fn duplicate_size(&mut self) {
        let old_size = self.size;
        self.size *= 2;
        let right = Rect(UVec2::new(old_size.x(), 0), UVec2::new(self.size.x(), old_size.y()), self.next_id());
        let bottom = Rect(UVec2::new(0, old_size.y()), self.size, self.next_id());
        self.free_rects.push(right);
        self.free_rects.push(bottom);
        while self.merge_free_rects() {}
    }

    fn find_best_rect(&self, size: impl Into<UVec2>) -> Option<Rect> {
        let size = size.into();
        self.free_rects
            .iter()
            .filter(|&rect| rect.width() >= size.x() && rect.height() >= size.y())
            .min_by_key(|&rect| rect.width() * rect.height())
            .copied()
    }

    fn split_rect(&mut self, rect: Rect, size: impl Into<UVec2>) {
        let size = size.into();
        let (right, bottom) = if rect.width() <= rect.height() {
            (
                Rect(rect.0 + (size.x(), 0), rect.1 - (0, rect.height() - size.y()), self.next_id()),
                Rect(rect.0 + (0, size.y()), rect.1, self.next_id())
            )
        } //
        else {
            (
                Rect(rect.0 + (size.x(), 0), rect.1, self.next_id()),
                Rect(rect.0 + (0, size.y()), rect.1 - (rect.width() - size.x(), 0), self.next_id())
            )
        };
        if right.width() > 0 && right.height() > 0 {
            self.free_rects.push(right);
        }
        if bottom.width() > 0 && bottom.height() > 0 {
            self.free_rects.push(bottom);
        }
    }

    fn merge_free_rects(&mut self) -> bool {
        let mut i = 0;
        let mut merged_any = false;
        while i < self.free_rects.len() {
            let mut merged = false;
            let rect1 = self.free_rects[i];
            let mut j = i + 1;
            while j < self.free_rects.len() {
                let rect2 = self.free_rects[j];
                if let Some(merged_rect) = rect1.try_merge(&rect2) {
                    self.free_rects[i] = merged_rect;
                    self.free_rects.swap_remove(j);
                    merged = true;
                    merged_any = true;
                    break;
                }
                j += 1;
            }
            if !merged {
                i += 1;
            }
        }
        merged_any
    }
}