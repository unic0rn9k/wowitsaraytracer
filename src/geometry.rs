use crate::*;

fn color_from_scaler(mut scaler: f32) -> Vec3 {
    let r = scaler % 100.;
    scaler = scaler / 100.;
    let g = scaler % 100.;
    scaler = scaler / 100.;
    let b = scaler % 100.;
    vec3![r / 100., g / 100., b / 100.]
}

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
        let mut intersected_objects = 0;

        for obj in scene.iter() {
            if let Some(Intersection { normal, t, .. }) =
                obj.intersects(self, 0., 4.) && t < dist
            {
                intersected_objects += 1;
                dist=t;
                min_normal = normal
            }
        }
        if intersected_objects > 0 {
            return (vec3![min_normal.dot(&vec3![0.3, 0.4, 0.3]), 0, 0] + color_from_scaler(dist))
                .to_color();
        }
        0
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
        while dist > 0.001 {
            let d = self.0.distance(ray.at(t));
            //if d > dist || t > t_max {
            //    return None;
            //}
            if t > t_max {
                return None;
            }
            dist = d;
            t += dist * 0.3;
        }
        Some(Intersection {
            point: ray.at(t),
            normal: vec3![0],
            t,
            is_front_facing: false,
        })
    }
}
