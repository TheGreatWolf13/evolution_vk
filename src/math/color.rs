#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ColorRGBA(f32, f32, f32, f32);

impl ColorRGBA {
    #[inline]
    pub const fn rgb(r: f32, g: f32, b: f32) -> ColorRGBA {
        Self::rgba(r, g, b, 1.0)
    }

    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> ColorRGBA {
        debug_assert!(r >= 0.0 && r <= 1.0, "R component out of range");
        debug_assert!(g >= 0.0 && g <= 1.0, "G component out of range");
        debug_assert!(b >= 0.0 && b <= 1.0, "B component out of range");
        debug_assert!(a >= 0.0 && a <= 1.0, "A component out of range");
        ColorRGBA(r, g, b, a)
    }

    #[inline]
    pub const fn from_hex(code_argb: u32) -> ColorRGBA {
        Self::rgba((code_argb >> 16 & 0xFF) as f32 / 255.0, (code_argb >> 8 & 0xFF) as f32 / 255.0, (code_argb & 0xFF) as f32 / 255.0, (code_argb >> 24 & 0xFF) as f32 / 255.0)
    }

    #[inline]
    pub const fn r(&self) -> f32 {
        self.0
    }

    #[inline]
    pub const fn g(&self) -> f32 {
        self.1
    }

    #[inline]
    pub const fn b(&self) -> f32 {
        self.2
    }

    #[inline]
    pub const fn a(&self) -> f32 {
        self.3
    }
}
