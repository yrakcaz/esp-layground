pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    #[must_use]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl From<&Rgb> for u32 {
    /// Convert RGB to u32 color value
    ///
    /// e.g. rgb: (1,2,4)
    /// G        R        B
    /// 7      0 7      0 7      0
    /// 00000010 00000001 00000100
    fn from(rgb: &Rgb) -> Self {
        (u32::from(rgb.g) << 16) | (u32::from(rgb.r) << 8) | u32::from(rgb.b)
    }
}

const BRIGHTNESS: u8 = 25;
pub const BLACK: Rgb = Rgb { r: 0, g: 0, b: 0 };
pub const GREEN: Rgb = Rgb {
    r: 0,
    g: BRIGHTNESS,
    b: 0,
};
pub const RED: Rgb = Rgb {
    r: BRIGHTNESS,
    g: 0,
    b: 0,
};
