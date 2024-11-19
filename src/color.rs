/// Represents an RGB color value.
///
/// # Fields
/// * `r` - Red component of the color.
/// * `g` - Green component of the color.
/// * `b` - Blue component of the color.
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    /// Creates a new `Rgb` instance.
    ///
    /// # Arguments
    /// * `r` - Red component of the color.
    /// * `g` - Green component of the color.
    /// * `b` - Blue component of the color.
    ///
    /// # Returns
    /// A new `Rgb` instance.
    #[must_use]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl From<&Rgb> for u32 {
    /// Converts an `Rgb` instance to a `u32` color value.
    /// e.g. rgb: (1,2,4)
    /// G        R        B
    /// 7      0 7      0 7      0
    /// 00000010 00000001 00000100
    ///
    /// # Returns
    /// A `u32` representation of the RGB color.
    fn from(rgb: &Rgb) -> Self {
        (u32::from(rgb.g) << 16) | (u32::from(rgb.r) << 8) | u32::from(rgb.b)
    }
}

/// Brightness level for predefined colors.
const BRIGHTNESS: u8 = 25;

/// Predefined black color.
pub const BLACK: Rgb = Rgb { r: 0, g: 0, b: 0 };

/// Predefined green color.
pub const GREEN: Rgb = Rgb {
    r: 0,
    g: BRIGHTNESS,
    b: 0,
};

/// Predefined red color.
pub const RED: Rgb = Rgb {
    r: BRIGHTNESS,
    g: 0,
    b: 0,
};
