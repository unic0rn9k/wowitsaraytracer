use crate::*;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn render(&self, scene: &[Box<dyn RaytracedGeometry>]) -> u32 {
        let mut min_normal = vec3![1];
        let mut dist = std::f32::INFINITY;
        let mut intersected_balls = 0;

        for obj in scene.iter() {
            if let Some(Intersection { normal, t, .. }) =
                obj.intersects(self, 0., 2.) && t < dist
            {
                intersected_balls += 1;
                dist=t;
                min_normal = normal
            }
        }
        if intersected_balls > 0 {
            return (vec3![min_normal.dot(&vec3![0.2, 0.2, 0.2]), 0, 0]
                + (dist.powi(2) * vec3![0.1, 0.2, 0.7]))
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

pub trait RaytracedGeometry {
    fn intersects(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection>;
}

pub trait RaymarchedGeometry {
    fn distance(&self, point: Vec3) -> f32;
}

pub struct FakeRaytrace<T: RaymarchedGeometry>(pub T);

impl<T: RaymarchedGeometry> RaytracedGeometry for FakeRaytrace<T> {
    fn intersects(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        let mut dist = std::f32::INFINITY;
        let mut t = t_min;
        while dist > 0.001 {
            let d = self.0.distance(ray.at(t));
            if d > dist || t > t_max {
                return None;
            }
            dist = d;
            t += 0.001;
        }
        Some(Intersection {
            point: ray.at(t),
            normal: vec3![0],
            t,
            is_front_facing: false,
        })
    }
}
