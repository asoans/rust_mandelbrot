extern crate image;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct MandelbrotParams {
    pub width: u32,
    pub height: u32,
    pub cx: f32,
    pub cy: f32,
}

pub fn mandelbrot_generate(params: &MandelbrotParams) -> Vec<u8> {
    let mut ret = Vec::<u8>::new();
    let iterations = 280;

    for y in 0..params.height {
        for x in 0..params.width {
            let inner_height = params.height as f32;
            let inner_width = params.width as f32;
            let inner_y = y as f32;
            let inner_x = x as f32;

            // Convert pixel coordinate to complex number
            let cx = 3.0 * (inner_x - 0.5 * inner_width) / inner_width + params.cx;
            let cy = 2.0 * (inner_y - 0.5 * inner_height) / inner_height + params.cy;

            let mut zx = 0.0; // Start at 0 for Mandelbrot
            let mut zy = 0.0; // Start at 0 for Mandelbrot

            let mut i = iterations;

            while zx * zx + zy * zy < 4.0 && i > 1 {
                let tmp = zx * zx - zy * zy + cx; // Mandelbrot iteration
                zy = 2.0 * zx * zy + cy;          // Mandelbrot iteration
                zx = tmp;
                i -= 1;
            }

            // Color calculation remains the same
            ret.push((i << 1) as u8);
            ret.push((i << 2) as u8);
            ret.push((i << 3) as u8);
            ret.push(255);
        }
    }

    ret
}
#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};
    use std::time::{Instant};
    use rand::{distributions::Uniform, Rng};

    fn save_image(rgb: Vec<u8>, width: u32, height: u32, fname: String) -> bool {
        let mut img = ImageBuffer::new(width as u32, height as u32);
        let mut idx:usize = 0;
        for y in 0..height {
            for x in 0..width {
                let pixel = Rgb([
                    rgb[idx] as u8, rgb[idx+1] as u8, rgb[idx+2] as u8
                ]);
                idx += 4; // its encoded as rgba
                img.put_pixel(x as u32, y as u32, pixel);
            }
        }
        img.save(fname).is_ok()
    }

    #[test]
    fn mandelbrot_save_png() {
        let start = Instant::now();

        let params = MandelbrotParams {
            width: 800,
            height: 500,
            cx: -0.758089028239593,
            cy: 0.05543246580171968,
        };
        let v = mandelbrot_generate(&params);
        println!("Mandelbrot set generation: cx: {:.3}, cy: {:.3}, time: {:.2?}", 
            params.cx, params.cy, start.elapsed());

        assert!(save_image(v, params.width, params.height, "mandelbrot-set.png".to_string()));
    }

    #[test]
    fn mandelbrot_speed() {

        let mut params = MandelbrotParams {
            width: 800,
            height: 500,
            cx: 0.0,
            cy: 0.0
        };

        let mut rng = rand::thread_rng();
        let cx_range = Uniform::new(-0.9, 0.9);
        let cy_range = Uniform::new(-0.9, 0.9);

        for _i in 0..10 {
            params.cx = rng.sample(&cx_range);
            params.cy = rng.sample(&cy_range);
            let start = Instant::now();
            let _v = mandelbrot_generate(&params);
            println!("Mandelbrot set generation: cx: {:.3}, cy: {:.3}, time: {:.2?}", 
                params.cx, params.cy, start.elapsed());
        }

        assert!(true);
    }
}