//! # A Gay and proud ray tracer, written in rust.
//! based on https://raytracing.github.io/books/RayTracingInOneWeekend.html
//!
//! ## Screenshots
//! ![](https://raw.githubusercontent.com/unic0rn9k/wowitsaraytracer/master/screenshot.jpeg)

use anyhow::*;
use geometry::{Intersection, Ray, RaymarchedGeometry, RaytracedGeometry};
use minifb::{self, Key, Window, WindowOptions};

const ASPECT_RATION: f32 = 16. / 19.;

const WIDTH: usize = 400;
const HEIGHT: usize = (WIDTH as f32 / ASPECT_RATION) as usize;

mod vec3;
use vec3::Vec3;

use crate::geometry::FakeRaytrace;

mod geometry;

struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub const fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl RaytracedGeometry for Sphere {
    fn intersects(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_square();

        let half_b = ray.direction.dot(&oc);
        let c = oc.length_square() - self.radius.powi(2);

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut t = (-half_b - sqrtd) / a;
        if t < t_min || t_max < t {
            t = (-half_b + sqrtd) / a;
            if t < t_min || t_max < t {
                return None;
            }
        }

        let point = ray.at(t);
        Some(Intersection {
            point,
            t,
            normal: (point - self.center) / self.radius,
            is_front_facing: true,
        })
    }
}

impl RaymarchedGeometry for Sphere {
    fn distance(&self, point: Vec3) -> f32 {
        (self.center - point).length() - self.radius
    }
}

struct MandelBulb;

impl RaymarchedGeometry for MandelBulb {
    fn distance(&self, point: Vec3) -> f32 {
        const POWER: i32 = 10;
        let mut z = point;
        let mut dr = 1.;
        let mut r = 0.;
        for _ in 0..10 {
            r = z.length();
            if r > 2. {
                break;
            }
            let theta = (z.z / r).acos();
            let phi = (z.y / z.x).atan();
            dr = r.powi(POWER - 1) * POWER as f32 * dr + 1.;
            let zr = r.powi(POWER);
            let theta = theta * POWER as f32;
            let phi = phi * POWER as f32;
            z = zr
                * vec3![
                    theta.sin() * phi.cos(),
                    phi.sin() * theta.sin(),
                    theta.cos()
                ];
            z += point;
        }
        0.5 * r.ln() * r / dr
    }
}

macro_rules! scene{
    ($($obj: expr),* $(,)?) => {{
        let tmp: Vec<Box<dyn RaytracedGeometry>> = vec![$(Box::new($obj)),*];
        tmp
    }}
}

fn main() -> Result<()> {
    let scene = scene![
        Sphere::new(vec3![0, 1, -1], 0.5),
        //FakeRaytrace(Sphere::new(vec3![0.2, 0.1, -0.8], 0.3)),
        //Sphere::new(vec3![0.2, 0.1, -0.8], 0.3),
        FakeRaytrace(MandelBulb)
    ];

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Wow. It's a raytracer!",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .context("minifb was unable to crate window")?;

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let viewport_height = 2.;
    let viewport_width = viewport_height * ASPECT_RATION;
    let focal_length = 3.;

    let origin = vec3![0, 0.5, 3];
    let horizontal = vec3![viewport_width, 0, 0];
    let vertical = vec3![0, viewport_height, 0];

    let lower_left_corner = origin - horizontal / 2. - vertical / 2. - vec3![0, 0, focal_length];

    while window.is_open() && !window.is_key_down(Key::Q) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let u = x as f32 / (WIDTH as f32 - 1.);
                let v = (HEIGHT - y - 1) as f32 / (HEIGHT as f32 - 1.);

                let relative_dir = lower_left_corner + u * horizontal + v * vertical - origin;
                let r = Ray::new(origin, relative_dir); //* (vec3![0] - origin));

                buffer[x + y * WIDTH] = r.render(&scene);
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }

    Ok(())
}
