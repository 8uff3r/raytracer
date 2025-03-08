mod utils;
use std::f64;
use utils::{color::*, constants::*, shapes::*, vector::*};

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
