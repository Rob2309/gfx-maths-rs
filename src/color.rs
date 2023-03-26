use std::fmt::Display;

use auto_ops::impl_op_ex;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Color {
    /// Red component [0.0, 1.0]
    pub r: f32,
    /// Green component [0.0, 1.0]
    pub g: f32,
    /// Blue component [0.0, 1.0]
    pub b: f32,
    /// Alpha component [0.0, 1.0]
    pub a: f32,
}

impl Default for Color {
    /// Creates a 'empty' color
    fn default() -> Self {
        Self::BLACK
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { r, g, b, a } = self;
        write!(f, "({r}, {g}, {b}, {a})")
    }
}

impl Color {
    // The basic CSS colors, for quick prototyping (https://www.w3.org/wiki/CSS/Properties/color/keywords)
    /// <div style="background-color:rgb(0%, 0%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    /// <div style="background-color:rgb(75%, 75%, 75%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const SILVER: Self = Self::new(0.75, 0.75, 0.75, 1.0);
    /// <div style="background-color:rgb(50%, 50%, 50%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const GRAY: Self = Self::new(0.5, 0.5, 0.5, 1.0);
    /// <div style="background-color:rgb(100%, 100%, 100%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);
    /// <div style="background-color:rgb(50%, 0%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const MAROON: Self = Self::new(0.5, 0.0, 0.0, 1.0);
    /// <div style="background-color:rgb(100%, 0%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const RED: Self = Self::new(1.0, 0.0, 0.0, 1.0);
    /// <div style="background-color:rgb(50%, 0%, 50%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const PURPLE: Self = Self::new(0.5, 0.0, 0.5, 1.0);
    /// <div style="background-color:rgb(100%, 0%, 100%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const FUCHSIA: Self = Self::new(1.0, 0.0, 1.0, 1.0);
    /// <div style="background-color:rgb(0%, 50%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const GREEN: Self = Self::new(0.0, 0.5, 0.0, 1.0);
    /// <div style="background-color:rgb(0%, 100%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const LIME: Self = Self::new(0.0, 1.0, 0.0, 1.0);
    /// <div style="background-color:rgb(50%, 50%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const OLIVE: Self = Self::new(0.5, 0.5, 0.0, 1.0);
    /// <div style="background-color:rgb(100%, 100%, 0%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const YELLOW: Self = Self::new(1.0, 1.0, 0.0, 1.0);
    /// <div style="background-color:rgb(0%, 0%, 50%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const NAVI: Self = Self::new(0.0, 0.0, 0.5, 1.0);
    /// <div style="background-color:rgb(0%, 0%, 100%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0, 1.0);
    /// <div style="background-color:rgb(0%, 50%, 50%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const TEAL: Self = Self::new(0.0, 0.5, 0.5, 1.0);
    /// <div style="background-color:rgb(0%, 100%, 100%); width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const AQUA: Self = Self::new(0.0, 1.0, 1.0, 1.0);

    /// Creates a Color from new components
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Creates a Color from a hex value.
    /// The format of `hex` is `0xRRGGBB`, the MSB is discarded.
    /// The `a` component is always set to 1.0.
    pub fn from_hex_rgb(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let b = (hex & 0xFF) as f32 / 255.0;
        Self::new(r, g, b, 1.0)
    }

    /// Creates a Color from a hex value.
    /// The format of `hex` is `0xBBGGRR`, the MSB is discarded.
    /// The `a` component is always set to 1.0.
    pub fn from_hex_bgr(hex: u32) -> Self {
        let b = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let r = (hex & 0xFF) as f32 / 255.0;
        Self::new(r, g, b, 1.0)
    }

    /// Creates a Color from a hex value.
    /// The format of `hex` is `0xRRGGBBAA`.
    pub fn from_hex_rgba(hex: u32) -> Self {
        let r = ((hex >> 24) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let b = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let a = (hex & 0xFF) as f32 / 255.0;
        Self::new(r, g, b, a)
    }

    /// Creates a Color from a hex value.
    /// The format of `hex` is `0xAARRGGBB`.
    pub fn from_hex_argb(hex: u32) -> Self {
        let a = ((hex >> 24) & 0xFF) as f32 / 255.0;
        let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let b = (hex & 0xFF) as f32 / 255.0;
        Self::new(r, g, b, a)
    }

    swizzle!(r, r, r);
    swizzle!(r, r, g);
    swizzle!(r, r, b);
    swizzle!(r, r, a);
    swizzle!(r, g, r);
    swizzle!(r, g, g);
    swizzle!(r, g, b);
    swizzle!(r, g, a);
    swizzle!(r, b, r);
    swizzle!(r, b, g);
    swizzle!(r, b, b);
    swizzle!(r, b, a);
    swizzle!(r, a, r);
    swizzle!(r, a, g);
    swizzle!(r, a, b);
    swizzle!(r, a, a);
    swizzle!(g, r, r);
    swizzle!(g, r, g);
    swizzle!(g, r, b);
    swizzle!(g, r, a);
    swizzle!(g, g, r);
    swizzle!(g, g, g);
    swizzle!(g, g, b);
    swizzle!(g, g, a);
    swizzle!(g, b, r);
    swizzle!(g, b, g);
    swizzle!(g, b, b);
    swizzle!(g, b, a);
    swizzle!(g, a, r);
    swizzle!(g, a, g);
    swizzle!(g, a, b);
    swizzle!(g, a, a);
    swizzle!(b, r, r);
    swizzle!(b, r, g);
    swizzle!(b, r, b);
    swizzle!(b, r, a);
    swizzle!(b, g, r);
    swizzle!(b, g, g);
    swizzle!(b, g, b);
    swizzle!(b, g, a);
    swizzle!(b, b, r);
    swizzle!(b, b, g);
    swizzle!(b, b, b);
    swizzle!(b, b, a);
    swizzle!(b, a, r);
    swizzle!(b, a, g);
    swizzle!(b, a, b);
    swizzle!(b, a, a);
    swizzle!(a, r, r);
    swizzle!(a, r, g);
    swizzle!(a, r, b);
    swizzle!(a, r, a);
    swizzle!(a, g, r);
    swizzle!(a, g, g);
    swizzle!(a, g, b);
    swizzle!(a, g, a);
    swizzle!(a, b, r);
    swizzle!(a, b, g);
    swizzle!(a, b, b);
    swizzle!(a, b, a);
    swizzle!(a, a, r);
    swizzle!(a, a, g);
    swizzle!(a, a, b);
    swizzle!(a, a, a);

    swizzle!(r, r, r, r);
    swizzle!(r, r, r, g);
    swizzle!(r, r, r, b);
    swizzle!(r, r, r, a);
    swizzle!(r, r, g, r);
    swizzle!(r, r, g, g);
    swizzle!(r, r, g, b);
    swizzle!(r, r, g, a);
    swizzle!(r, r, b, r);
    swizzle!(r, r, b, g);
    swizzle!(r, r, b, b);
    swizzle!(r, r, b, a);
    swizzle!(r, r, a, r);
    swizzle!(r, r, a, g);
    swizzle!(r, r, a, b);
    swizzle!(r, r, a, a);
    swizzle!(r, g, r, r);
    swizzle!(r, g, r, g);
    swizzle!(r, g, r, b);
    swizzle!(r, g, r, a);
    swizzle!(r, g, g, r);
    swizzle!(r, g, g, g);
    swizzle!(r, g, g, b);
    swizzle!(r, g, g, a);
    swizzle!(r, g, b, r);
    swizzle!(r, g, b, g);
    swizzle!(r, g, b, b);
    swizzle!(r, g, b, a);
    swizzle!(r, g, a, r);
    swizzle!(r, g, a, g);
    swizzle!(r, g, a, b);
    swizzle!(r, g, a, a);
    swizzle!(r, b, r, r);
    swizzle!(r, b, r, g);
    swizzle!(r, b, r, b);
    swizzle!(r, b, r, a);
    swizzle!(r, b, g, r);
    swizzle!(r, b, g, g);
    swizzle!(r, b, g, b);
    swizzle!(r, b, g, a);
    swizzle!(r, b, b, r);
    swizzle!(r, b, b, g);
    swizzle!(r, b, b, b);
    swizzle!(r, b, b, a);
    swizzle!(r, b, a, r);
    swizzle!(r, b, a, g);
    swizzle!(r, b, a, b);
    swizzle!(r, b, a, a);
    swizzle!(r, a, r, r);
    swizzle!(r, a, r, g);
    swizzle!(r, a, r, b);
    swizzle!(r, a, r, a);
    swizzle!(r, a, g, r);
    swizzle!(r, a, g, g);
    swizzle!(r, a, g, b);
    swizzle!(r, a, g, a);
    swizzle!(r, a, b, r);
    swizzle!(r, a, b, g);
    swizzle!(r, a, b, b);
    swizzle!(r, a, b, a);
    swizzle!(r, a, a, r);
    swizzle!(r, a, a, g);
    swizzle!(r, a, a, b);
    swizzle!(r, a, a, a);
    swizzle!(g, r, r, r);
    swizzle!(g, r, r, g);
    swizzle!(g, r, r, b);
    swizzle!(g, r, r, a);
    swizzle!(g, r, g, r);
    swizzle!(g, r, g, g);
    swizzle!(g, r, g, b);
    swizzle!(g, r, g, a);
    swizzle!(g, r, b, r);
    swizzle!(g, r, b, g);
    swizzle!(g, r, b, b);
    swizzle!(g, r, b, a);
    swizzle!(g, r, a, r);
    swizzle!(g, r, a, g);
    swizzle!(g, r, a, b);
    swizzle!(g, r, a, a);
    swizzle!(g, g, r, r);
    swizzle!(g, g, r, g);
    swizzle!(g, g, r, b);
    swizzle!(g, g, r, a);
    swizzle!(g, g, g, r);
    swizzle!(g, g, g, g);
    swizzle!(g, g, g, b);
    swizzle!(g, g, g, a);
    swizzle!(g, g, b, r);
    swizzle!(g, g, b, g);
    swizzle!(g, g, b, b);
    swizzle!(g, g, b, a);
    swizzle!(g, g, a, r);
    swizzle!(g, g, a, g);
    swizzle!(g, g, a, b);
    swizzle!(g, g, a, a);
    swizzle!(g, b, r, r);
    swizzle!(g, b, r, g);
    swizzle!(g, b, r, b);
    swizzle!(g, b, r, a);
    swizzle!(g, b, g, r);
    swizzle!(g, b, g, g);
    swizzle!(g, b, g, b);
    swizzle!(g, b, g, a);
    swizzle!(g, b, b, r);
    swizzle!(g, b, b, g);
    swizzle!(g, b, b, b);
    swizzle!(g, b, b, a);
    swizzle!(g, b, a, r);
    swizzle!(g, b, a, g);
    swizzle!(g, b, a, b);
    swizzle!(g, b, a, a);
    swizzle!(g, a, r, r);
    swizzle!(g, a, r, g);
    swizzle!(g, a, r, b);
    swizzle!(g, a, r, a);
    swizzle!(g, a, g, r);
    swizzle!(g, a, g, g);
    swizzle!(g, a, g, b);
    swizzle!(g, a, g, a);
    swizzle!(g, a, b, r);
    swizzle!(g, a, b, g);
    swizzle!(g, a, b, b);
    swizzle!(g, a, b, a);
    swizzle!(g, a, a, r);
    swizzle!(g, a, a, g);
    swizzle!(g, a, a, b);
    swizzle!(g, a, a, a);
    swizzle!(b, r, r, r);
    swizzle!(b, r, r, g);
    swizzle!(b, r, r, b);
    swizzle!(b, r, r, a);
    swizzle!(b, r, g, r);
    swizzle!(b, r, g, g);
    swizzle!(b, r, g, b);
    swizzle!(b, r, g, a);
    swizzle!(b, r, b, r);
    swizzle!(b, r, b, g);
    swizzle!(b, r, b, b);
    swizzle!(b, r, b, a);
    swizzle!(b, r, a, r);
    swizzle!(b, r, a, g);
    swizzle!(b, r, a, b);
    swizzle!(b, r, a, a);
    swizzle!(b, g, r, r);
    swizzle!(b, g, r, g);
    swizzle!(b, g, r, b);
    swizzle!(b, g, r, a);
    swizzle!(b, g, g, r);
    swizzle!(b, g, g, g);
    swizzle!(b, g, g, b);
    swizzle!(b, g, g, a);
    swizzle!(b, g, b, r);
    swizzle!(b, g, b, g);
    swizzle!(b, g, b, b);
    swizzle!(b, g, b, a);
    swizzle!(b, g, a, r);
    swizzle!(b, g, a, g);
    swizzle!(b, g, a, b);
    swizzle!(b, g, a, a);
    swizzle!(b, b, r, r);
    swizzle!(b, b, r, g);
    swizzle!(b, b, r, b);
    swizzle!(b, b, r, a);
    swizzle!(b, b, g, r);
    swizzle!(b, b, g, g);
    swizzle!(b, b, g, b);
    swizzle!(b, b, g, a);
    swizzle!(b, b, b, r);
    swizzle!(b, b, b, g);
    swizzle!(b, b, b, b);
    swizzle!(b, b, b, a);
    swizzle!(b, b, a, r);
    swizzle!(b, b, a, g);
    swizzle!(b, b, a, b);
    swizzle!(b, b, a, a);
    swizzle!(b, a, r, r);
    swizzle!(b, a, r, g);
    swizzle!(b, a, r, b);
    swizzle!(b, a, r, a);
    swizzle!(b, a, g, r);
    swizzle!(b, a, g, g);
    swizzle!(b, a, g, b);
    swizzle!(b, a, g, a);
    swizzle!(b, a, b, r);
    swizzle!(b, a, b, g);
    swizzle!(b, a, b, b);
    swizzle!(b, a, b, a);
    swizzle!(b, a, a, r);
    swizzle!(b, a, a, g);
    swizzle!(b, a, a, b);
    swizzle!(b, a, a, a);
    swizzle!(a, r, r, r);
    swizzle!(a, r, r, g);
    swizzle!(a, r, r, b);
    swizzle!(a, r, r, a);
    swizzle!(a, r, g, r);
    swizzle!(a, r, g, g);
    swizzle!(a, r, g, b);
    swizzle!(a, r, g, a);
    swizzle!(a, r, b, r);
    swizzle!(a, r, b, g);
    swizzle!(a, r, b, b);
    swizzle!(a, r, b, a);
    swizzle!(a, r, a, r);
    swizzle!(a, r, a, g);
    swizzle!(a, r, a, b);
    swizzle!(a, r, a, a);
    swizzle!(a, g, r, r);
    swizzle!(a, g, r, g);
    swizzle!(a, g, r, b);
    swizzle!(a, g, r, a);
    swizzle!(a, g, g, r);
    swizzle!(a, g, g, g);
    swizzle!(a, g, g, b);
    swizzle!(a, g, g, a);
    swizzle!(a, g, b, r);
    swizzle!(a, g, b, g);
    swizzle!(a, g, b, b);
    swizzle!(a, g, b, a);
    swizzle!(a, g, a, r);
    swizzle!(a, g, a, g);
    swizzle!(a, g, a, b);
    swizzle!(a, g, a, a);
    swizzle!(a, b, r, r);
    swizzle!(a, b, r, g);
    swizzle!(a, b, r, b);
    swizzle!(a, b, r, a);
    swizzle!(a, b, g, r);
    swizzle!(a, b, g, g);
    swizzle!(a, b, g, b);
    swizzle!(a, b, g, a);
    swizzle!(a, b, b, r);
    swizzle!(a, b, b, g);
    swizzle!(a, b, b, b);
    swizzle!(a, b, b, a);
    swizzle!(a, b, a, r);
    swizzle!(a, b, a, g);
    swizzle!(a, b, a, b);
    swizzle!(a, b, a, a);
    swizzle!(a, a, r, r);
    swizzle!(a, a, r, g);
    swizzle!(a, a, r, b);
    swizzle!(a, a, r, a);
    swizzle!(a, a, g, r);
    swizzle!(a, a, g, g);
    swizzle!(a, a, g, b);
    swizzle!(a, a, g, a);
    swizzle!(a, a, b, r);
    swizzle!(a, a, b, g);
    swizzle!(a, a, b, b);
    swizzle!(a, a, b, a);
    swizzle!(a, a, a, r);
    swizzle!(a, a, a, g);
    swizzle!(a, a, a, b);
    swizzle!(a, a, a, a);
}

impl_op_ex!(+= |a: &mut Color, b: &Color| { a.r += b.r; a.g += b.g; a.b += b.b; a.a += b.a; });
impl_op_ex!(-= |a: &mut Color, b: &Color| { a.r -= b.r; a.g -= b.g; a.b -= b.b; a.a -= b.a; });

impl_op_ex!(+ |a: &Color, b: &Color| -> Color { Color{r: a.r + b.r, g: a.g + b.g, b: a.b + b.b, a: a.a + b.a } });
impl_op_ex!(-|a: &Color, b: &Color| -> Color {
    Color {
        r: a.r - b.r,
        g: a.g - b.g,
        b: a.b - b.b,
        a: a.a - b.a,
    }
});

impl From<[f32; 3]> for Color {
    fn from(d: [f32; 3]) -> Self {
        Self {
            r: d[0],
            g: d[1],
            b: d[2],
            a: 1.0,
        }
    }
}

impl From<Color> for [f32; 3] {
    fn from(value: Color) -> Self {
        [value.r, value.g, value.b]
    }
}

impl From<[f32; 4]> for Color {
    fn from(d: [f32; 4]) -> Self {
        Self {
            r: d[0],
            g: d[1],
            b: d[2],
            a: d[3],
        }
    }
}

impl From<Color> for [f32; 4] {
    fn from(value: Color) -> Self {
        [value.r, value.g, value.b, value.a]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_hex_rgb() {
        assert_eq!(
            Color::from_hex_rgb(0xFF0000),
            Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            }
        );

        assert_eq!(
            Color::from_hex_rgb(0x00FF00),
            Color {
                r: 0.0,
                g: 1.0,
                b: 0.0,
                a: 1.0,
            }
        );

        assert_eq!(
            Color::from_hex_rgb(0x0000FF),
            Color {
                r: 0.0,
                g: 0.0,
                b: 1.0,
                a: 1.0,
            }
        );

        assert_eq!(
            Color::from_hex_rgb(0xFFFFFF),
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            }
        );
    }

    #[test]
    fn from_hex_bgr() {
        assert_eq!(
            Color::from_hex_bgr(0x0000FF),
            Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            }
        );

        assert_eq!(
            Color::from_hex_bgr(0x00FF00),
            Color {
                r: 0.0,
                g: 1.0,
                b: 0.0,
                a: 1.0,
            }
        );

        assert_eq!(
            Color::from_hex_bgr(0xFF0000),
            Color {
                r: 0.0,
                g: 0.0,
                b: 1.0,
                a: 1.0,
            }
        );

        assert_eq!(
            Color::from_hex_bgr(0xFFFFFF),
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            }
        );
    }

    #[test]
    fn from_hex_rgba() {
        assert_eq!(
            Color::from_hex_rgba(0xFF000000),
            Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            }
        );

        assert_eq!(
            Color::from_hex_rgba(0x00FF0000),
            Color {
                r: 0.0,
                g: 1.0,
                b: 0.0,
                a: 0.0,
            }
        );

        assert_eq!(
            Color::from_hex_rgba(0x0000FF00),
            Color {
                r: 0.0,
                g: 0.0,
                b: 1.0,
                a: 0.0,
            }
        );

        assert_eq!(
            Color::from_hex_rgba(0x000000FF),
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            }
        );

        assert_eq!(
            Color::from_hex_rgba(0xFFFFFFFF),
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            }
        );
    }

    #[test]
    fn from_hex_argb() {
        assert_eq!(
            Color::from_hex_argb(0x00FF0000),
            Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            }
        );

        assert_eq!(
            Color::from_hex_argb(0x0000FF00),
            Color {
                r: 0.0,
                g: 1.0,
                b: 0.0,
                a: 0.0,
            }
        );

        assert_eq!(
            Color::from_hex_argb(0x000000FF),
            Color {
                r: 0.0,
                g: 0.0,
                b: 1.0,
                a: 0.0,
            }
        );

        assert_eq!(
            Color::from_hex_argb(0xFF000000),
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            }
        );

        assert_eq!(
            Color::from_hex_argb(0xFFFFFFFF),
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            }
        );
    }

    #[test]
    fn operators() {
        let a = Color::new(1.0, 0.2, 0.3, 0.5);
        let b = Color::new(0.3, 0.4, 0.5, 0.5);

        assert_eq!(
            a + b,
            Color {
                r: 1.0 + 0.3,
                g: 0.2 + 0.4,
                b: 0.3 + 0.5,
                a: 0.5 + 0.5,
            }
        );

        assert_eq!(
            a - b,
            Color {
                r: 1.0 - 0.3,
                g: 0.2 - 0.4,
                b: 0.3 - 0.5,
                a: 0.5 - 0.5,
            }
        );

        #[allow(unused_assignments)]
        let mut c = a;

        c = a;
        c += b;
        assert_eq!(c, a + b);

        c = a;
        c -= b;
        assert_eq!(c, a - b);
    }
}
