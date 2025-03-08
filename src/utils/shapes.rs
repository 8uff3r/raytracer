use crate::utils::vector::*;

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> f64;
    fn normal(&self, ray: &Ray, t: f64) -> Vec3;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> f64 {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction * (oc);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = h.powi(2) - (a * c);
        if discriminant < 0. {
            -1.
        } else {
            (h - discriminant.sqrt()) / a
        }
    }
    fn normal(&self, ray: &Ray, t: f64) -> Vec3 {
        (ray.at(t) - self.center).unit_vector()
    }
}
