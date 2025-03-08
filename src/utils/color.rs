use std::{fmt::Display, ops::Add};

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}
impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hex_rgb = self.to_hex();
        f.write_fmt(format_args!(
            "{} {} {}",
            Color::to_u8(hex_rgb.r),
            Color::to_u8(hex_rgb.g),
            Color::to_u8(hex_rgb.b)
        ))
    }
}
impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}
impl Color {
    pub const RED: Color = Color {
        r: 1.,
        g: 0.,
        b: 0.,
    };
    pub const GREEN: Color = Color {
        r: 0.,
        g: 1.,
        b: 0.,
    };
    pub const BLUE: Color = Color {
        r: 0.,
        g: 0.,
        b: 1.,
    };
    pub const BLACK: Color = Color {
        r: 0.,
        g: 0.,
        b: 0.,
    };
    pub const WHITE: Color = Color {
        r: 1.,
        g: 1.,
        b: 1.,
    };
    pub const GRAY: Color = Color {
        r: 0.5,
        g: 0.5,
        b: 0.5,
    };

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }
    pub fn new_from_hex(r: f64, g: f64, b: f64) -> Self {
        Self {
            r: r * 255.999,
            g: g * 255.999,
            b: b * 255.999,
        }
    }
    pub fn to_u8(v: f64) -> u8 {
        if v > 255. {
            return 255;
        }
        v as u8
    }
    pub fn to_hex(&self) -> Self {
        Self {
            r: self.r * 255.999,
            g: self.g * 255.999,
            b: self.b * 255.999,
        }
    }
    pub fn muli(&self, s: f64) -> Self {
        Self {
            r: self.r * s,
            g: self.g * s,
            b: self.b * s,
        }
    }
}
