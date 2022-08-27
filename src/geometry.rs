use crate::*;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn background_color(&self) -> u32 {
        if let Some(Intersection { normal, .. }) =
            Sphere::new(vec3![0, 0, -1], 0.5).intersects(self, 0., 1.)
        {
            return vec3![
                normal.dot(&vec3![0, 0, 0.5]),
                normal.dot(&vec3![0, 0.2, 0.5]),
                normal.dot(&vec3![0, 0.3, 0.5]),
            ]
            .to_color();
        }
        let t = 0.5 * (self.direction.unit_vector().y + 1.);
        (vec3![1] * (1. - t) + vec3![0.5, 0.7, 1] * t).to_color()
    }

    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Intersection {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub is_front_facing: bool,
}

pub trait Geometry {
    fn intersects(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection>;
}
