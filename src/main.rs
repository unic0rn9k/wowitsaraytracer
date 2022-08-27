//! # A Gay and proud ray tracer, written in rust.
//! based on https://raytracing.github.io/books/RayTracingInOneWeekend.html

use anyhow::*;
use minifb::{self, Key, Window, WindowOptions};

const ASPECT_RATION: f32 = 16. / 19.;

const WIDTH: usize = 400;
const HEIGHT: usize = (WIDTH as f32 / ASPECT_RATION) as usize;

mod vec3;
use vec3::Vec3;

macro_rules! vec3 {
    ($x: expr, $y: expr, $z: expr $(,)?) => {
        Vec3::from_array([($x) as f32, ($y) as f32, ($z) as f32])
    };
    ($fill: expr) => {
        Vec3::from_array([($fill) as f32; 3])
    };
}

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    fn background_color(&self) -> u32 {
        let t = self.intersects_sphere(vec3![0, 0, -1], 0.5);
        if t > 0. {
            return (0.5 * ((self.at(t) - vec3![0, 0, -1]).unit_vector() + 1.)).to_color();
        }
        let t = 0.5 * (self.direction.unit_vector().y + 1.);
        (vec3![1] * (1. - t) + vec3![0.5, 0.7, 1] * t).to_color()
    }

    fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    fn intersects_sphere(&self, center: Vec3, radious: f32) -> f32 {
        let oc = self.origin - center;
        let a = self.direction.length_square();

        let b = 2. * self.direction.dot(&oc);
        let c = oc.length_square() - radious.powi(2);

        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            0.
        } else {
            (-b - discriminant.sqrt()) / (2. * a)
        }
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
