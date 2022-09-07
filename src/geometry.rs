use crate::*;

fn spectral_response(n: f32, channel: f32, width: f32) -> f32 {
    1. - ((n - channel) / width).powi(2)
}

fn color_from_scaler(n: f32) -> Vec3 {
    let r = spectral_response(n, 0., 1.5);
    let g = spectral_response(n, 0.1, 4.) * 0.3;
    vec3![r, g, 0]
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn render(&self, scene: &[Box<dyn RaytracedGeometry>]) -> Vec3 {
        let mut min_normal = vec3![1];
        let mut dist = std::f32::INFINITY;
        let mut intersected_objects = 0;

        for obj in scene.iter() {
            if let Some(Intersection { normal, t, .. }) =
                obj.intersects(self, 0., 2.) && t < dist
            {
                intersected_objects += 1;
                dist=t;
                min_normal = normal
            }
        }
        if intersected_objects > 0 {
            color_from_scaler(dist) * 0.1
                + color_from_scaler(dist) * min_normal
                + min_normal * vec3![0, 0, 1] * 0.1
        } else {
            vec3![0]
        }
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

#[derive(Clone, Copy)]
pub struct FakeRaytrace<T: RaymarchedGeometry + Copy>(pub T);

impl<T: RaymarchedGeometry + Copy> RaytracedGeometry for FakeRaytrace<T> {
    fn intersects(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        let mut dist = t_max;
        let mut t = t_min;
        let mut not_a_normal = 0.5;
        let mut highlight = 1.;
        while dist > 0.0005 {
            let d = self.0.distance(ray.at(t));
            if d > dist {
                not_a_normal *= 0.8;
                highlight *= 0.01;
            }
            if t > t_max {
                return None;
            }

            dist = d;
            t += dist * 0.4;
        }

        Some(Intersection {
            point: ray.at(t),
            normal: vec3![not_a_normal, not_a_normal, highlight],
            t,
            is_front_facing: false,
        })
    }
}
