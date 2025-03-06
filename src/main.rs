use std::{f64, fmt::Display};
struct Color {
    r: f64,
    g: f64,
    b: f64,
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
impl Color {
    const RED: Color = Color {
        r: 1.,
        g: 0.,
        b: 0.,
    };
    const GREEN: Color = Color {
        r: 0.,
        g: 1.,
        b: 0.,
    };
    const BLUE: Color = Color {
        r: 0.,
        g: 0.,
        b: 1.,
    };
    const BLACK: Color = Color {
        r: 0.,
        g: 0.,
        b: 0.,
    };
    const WHITE: Color = Color {
        r: 1.,
        g: 1.,
        b: 1.,
    };
    const GRAY: Color = Color {
        r: 0.5,
        g: 0.5,
        b: 0.5,
    };

    fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }
    fn new_from_hex(r: f64, g: f64, b: f64) -> Self {
        Self {
            r: r * 255.999,
            g: g * 255.999,
            b: b * 255.999,
        }
    }
    fn to_u8(v: f64) -> u8 {
        if v > 255. {
            return 255;
        }
        v as u8
    }
    fn to_hex(&self) -> Self {
        Self {
            r: self.r * 255.999,
            g: self.g * 255.999,
            b: self.b * 255.999,
        }
    }
    fn add(&self, v: Self) -> Self {
        Self {
            r: self.r + v.r,
            g: self.g + v.g,
            b: self.b + v.b,
        }
    }
    fn mul(&self, s: f64) -> Self {
        Self {
            r: self.r * s,
            g: self.g * s,
            b: self.b * s,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}
type Point3 = Vec3;
impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {} {}", self.x, self.y, self.z))
    }
}
impl Point3 {
    fn is_in_neighbour(&self, v: Self, t: f64) -> bool {
        v.sub(*self).magnitude() <= t
    }
}
impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    fn add(&self, v: Self) -> Self {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
    fn sub(&self, v: Self) -> Self {
        self.add(v.neg())
    }
    fn neg(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
    fn dot(&self, v: Self) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
    fn _cross(&self, v: Self) -> Self {
        Self {
            x: (self.y * v.z) - (self.z * v.y),
            y: (self.z * v.x) - (self.x * v.z),
            z: (self.x * v.y) - (self.y * v.x),
        }
    }
    fn mul(&self, s: f64) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
    fn div(&self, s: f64) -> Self {
        self.mul(1. / s)
    }
    fn magnitude(&self) -> f64 {
        self.length_squared().sqrt()
    }
    fn length_squared(&self) -> f64 {
        self.dot(*self)
    }
    fn _normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            *self
        } else {
            self.div(mag)
        }
    }
    fn unit_vector(&self) -> Self {
        self.div(self.magnitude())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Ray {
    origin: Point3,
    direction: Vec3,
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
    fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
    fn at(&self, t: f64) -> Vec3 {
        self.origin.add(self.direction.mul(t))
    }
    fn hit_color(&self, hit_object: impl Hittable) -> Color {
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

trait Hittable {
    fn hit(&self, ray: Ray) -> f64;
    fn normal(&self, ray: Ray, t: f64) -> Vec3;
}

struct Sphere {
    center: Point3,
    radius: f64,
}
impl Hittable for Sphere {
    fn hit(&self, ray: Ray) -> f64 {
        let oc = self.center.sub(ray.origin);
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = h.powi(2) - (a * c);
        if discriminant < 0. {
            -1.
        } else {
            (h - discriminant.sqrt()) / a
        }
    }
    fn normal(&self, ray: Ray, t: f64) -> Vec3 {
        (ray.at(t)).sub(self.center).unit_vector()
    }
}

// Image
const ASPECT_RATIO: f64 = 16. / 9.;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = {
    let h: f64 = (IMAGE_WIDTH as f64) / ASPECT_RATIO;
    if h < 1. {
        1
    } else {
        h as i32
    }
};
// Camera
const FOCAL_LENGTH: f64 = 1.;
const VIEWPORT_HEIGHT: f64 = 2.;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ((IMAGE_WIDTH as f64) / (IMAGE_HEIGHT as f64));
const CAMERA_CENTER: Point3 = Point3 {
    x: 0.,
    y: 0.,
    z: 0.,
};

// Calculate the vectors across the horizontal and down the vertical viewport edges.
const VIEWPORT_U: Vec3 = Vec3 {
    x: VIEWPORT_WIDTH,
    y: 0.,
    z: 0.,
};
const VIEWPORT_V: Vec3 = Vec3 {
    x: 0.,
    y: -VIEWPORT_HEIGHT,
    z: 0.,
};

fn main() {
    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u: Vec3 = VIEWPORT_U.div(IMAGE_WIDTH as f64);
    let pixel_delta_v: Vec3 = VIEWPORT_V.div(IMAGE_HEIGHT as f64);

    let viewport_upper_left = CAMERA_CENTER
        .sub(Vec3::new(0., 0., FOCAL_LENGTH))
        .sub(VIEWPORT_U.div(2.))
        .sub(VIEWPORT_V.div(2.));

    let pixel00_location = viewport_upper_left.add(pixel_delta_u.add(pixel_delta_v).mul(0.5));

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");
    for h_pos in 0..IMAGE_HEIGHT {
        for w_pos in 0..IMAGE_WIDTH {
            let pixel_center = pixel00_location.add(
                pixel_delta_u
                    .mul(w_pos as f64)
                    .add(pixel_delta_v.mul(h_pos as f64)),
            );
            let ray_direction = pixel_center.sub(CAMERA_CENTER);
            let ray = Ray::new(CAMERA_CENTER, ray_direction);
            let sphere_center = Point3 {
                x: 0.,
                y: 0.,
                z: -1.,
            };
            let sphere = Sphere {
                center: sphere_center,
                radius: 0.5,
            };
            if pixel_center.is_in_neighbour(Point3::new(0., 0., -1.), 0.01) {
                println!("{}", Color::RED)
            } else {
                let pixel_color = ray.hit_color(sphere);
                println!("{pixel_color}")
            }
        }
    }
}
