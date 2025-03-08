use std::{
    fmt::Display,
    ops::{Add, Mul},
};

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
            (hex_rgb.r as u8),
            (hex_rgb.g as u8),
            (hex_rgb.b as u8)
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
impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
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
            r: Self::map_to_hex(r) / 255.,
            g: Self::map_to_hex(g) / 255.,
            b: Self::map_to_hex(b) / 255.,
        }
    }
    fn map_to_hex(v: f64) -> f64 {
        if v > 255. {
            return 255.;
        }
        v
    }
    pub fn to_hex(&self) -> Self {
        Self {
            r: Self::map_to_hex(self.r * 255.999),
            g: Self::map_to_hex(self.g * 255.999),
            b: Self::map_to_hex(self.b * 255.999),
        }
    }
}
