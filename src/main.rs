mod utils;
use std::f64;
use utils::{color::*, constants::*, shapes::*, vector::*};

fn main() {
    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u: Vec3 = VIEWPORT_U / f64::from(IMAGE_WIDTH);
    let pixel_delta_v: Vec3 = VIEWPORT_V / f64::from(IMAGE_HEIGHT);

    let viewport_upper_left =
        CAMERA_CENTER - Vec3::new(0., 0., FOCAL_LENGTH) - (VIEWPORT_U / 2.) - (VIEWPORT_V / 2.);

    let pixel00_location = viewport_upper_left + ((pixel_delta_u + pixel_delta_v) * 0.5);

    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");
    for h_pos in 0..IMAGE_HEIGHT {
        for w_pos in 0..IMAGE_WIDTH {
            let pixel_center = pixel00_location
                + ((pixel_delta_u * f64::from(w_pos)) + (pixel_delta_v * f64::from(h_pos)));
            let ray_direction = pixel_center - CAMERA_CENTER;
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
                let pixel_color = ray.hit_color(&sphere);
                println!("{pixel_color}")
            }
        }
    }
}
