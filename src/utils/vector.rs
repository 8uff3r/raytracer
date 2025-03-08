use crate::utils::{color::*, shapes::*};
use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
pub type Point3 = Vec3;
impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {} {}", self.x, self.y, self.z))
    }
}
impl Point3 {
    pub fn is_in_neighbour(&self, v: Self, t: f64) -> bool {
        v.sub(*self).magnitude() <= t
    }
}
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn add(&self, v: Self) -> Self {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
    pub fn sub(&self, v: Self) -> Self {
        self.add(v.neg())
    }
    pub fn neg(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
    pub fn dot(&self, v: Self) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
    pub fn _cross(&self, v: Self) -> Self {
        Self {
            x: (self.y * v.z) - (self.z * v.y),
            y: (self.z * v.x) - (self.x * v.z),
            z: (self.x * v.y) - (self.y * v.x),
        }
    }
    pub fn mul(&self, s: f64) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
    pub fn div(&self, s: f64) -> Self {
        self.mul(1. / s)
    }
    pub fn magnitude(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.dot(*self)
    }
    pub fn _normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            *self
        } else {
            self.div(mag)
        }
    }
    pub fn unit_vector(&self) -> Self {
        self.div(self.magnitude())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}
impl Display for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "origin: ({}), direction: ({})",
            self.origin, self.direction
        ))
    }
}
impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin.add(self.direction.mul(t))
    }
    pub fn hit_color(&self, hit_object: impl Hittable) -> Color {
        let t = hit_object.hit(*self);
        if t > 0. {
            let normal = hit_object.normal(*self, t);
            Color::new(normal.x + 1., normal.y + 1., normal.z + 1.).mul(0.5)
        } else {
            let unit_drection = self.direction.unit_vector();
            let alpha = 0.5 * (unit_drection.y + 1.);
            Color::WHITE
                .mul(1. - alpha)
                .add(Color::new(0.5, 0.7, 1.).mul(alpha))
        }
    }
}
