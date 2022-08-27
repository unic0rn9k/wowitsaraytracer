//! # A Gay and proud ray tracer, written in rust.
//! based on https://raytracing.github.io/books/RayTracingInOneWeekend.html
//!
//! ## Screenshots
//! ![](screenshot.jepg)

use anyhow::*;
use geometry::{Geometry, Intersection, Ray};
use minifb::{self, Key, Window, WindowOptions};

const ASPECT_RATION: f32 = 16. / 19.;

const WIDTH: usize = 400;
const HEIGHT: usize = (WIDTH as f32 / ASPECT_RATION) as usize;

mod vec3;
use vec3::Vec3;

mod geometry;

struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Geometry for Sphere {
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

fn main() -> Result<()> {
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
    let focal_length = 1.;

    let origin = vec3![0];
    let horizontal = vec3![viewport_width, 0, 0];
    let vertical = vec3![0, viewport_height, 0];

    let lower_left_corner = origin - horizontal / 2. - vertical / 2. - vec3![0, 0, focal_length];

    while window.is_open() && !window.is_key_down(Key::Q) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let u = x as f32 / (WIDTH as f32 - 1.);
                let v = (HEIGHT - y - 1) as f32 / (HEIGHT as f32 - 1.);

                let r = Ray::new(
                    origin,
                    lower_left_corner + u * horizontal + v * vertical - origin,
                );

                buffer[x + y * WIDTH] = r.background_color();
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }

    Ok(())
}
