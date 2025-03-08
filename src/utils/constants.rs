use crate::utils::vector::*;
// Image
pub const ASPECT_RATIO: f64 = 16. / 9.;
pub const IMAGE_WIDTH: i32 = 400;
pub const IMAGE_HEIGHT: i32 = {
    let h: f64 = (IMAGE_WIDTH as f64) / ASPECT_RATIO;
    if h < 1. {
        1
    } else {
        h as i32
    }
};
// Camera
pub const FOCAL_LENGTH: f64 = 1.;
pub const VIEWPORT_HEIGHT: f64 = 2.;
pub const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ((IMAGE_WIDTH as f64) / (IMAGE_HEIGHT as f64));
pub const CAMERA_CENTER: Point3 = Point3 {
    x: 0.,
    y: 0.,
    z: 0.,
};

// Calculate the vectors across the horizontal and down the vertical viewport edges.
pub const VIEWPORT_U: Vec3 = Vec3 {
    x: VIEWPORT_WIDTH,
    y: 0.,
    z: 0.,
};
pub const VIEWPORT_V: Vec3 = Vec3 {
    x: 0.,
    y: -VIEWPORT_HEIGHT,
    z: 0.,
};
