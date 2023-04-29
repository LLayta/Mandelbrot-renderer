#![allow(dead_code, unused_imports, unused_mut, unused_variables)]

use std::{
    env::args,
    error::Error,
    process::exit,
};

use rayon::prelude::*;
use num_complex::Complex;
use image::{ ImageBuffer, Rgb };

/*
 @ Brief: Calculates the number of iterations when given a complex number.
 @ Params: {
        c - Is the given complex number. 
        max_iterations - Is the maximum amount of iterations we'll apply the function to 'c'.
        capture_size - The radius of the circle in the complex plane of which we have 'z' bounded to. 
        Anything that escapes the radius is bounded towards infinity aka won't be drawn.
    }
 @ Return: The number of iterations executed. 
*/

fn compute_color(c: Complex<f64>, max_iterations: usize, capture_size: f64) -> u8 {
    let mut z = Complex::new(0.0, 0.0);

    for i in 0..max_iterations {
        z = z * z + c;

        if z.norm() > capture_size {
            return (i as f64 / max_iterations as f64 * 255.0) as u8;
        }
    }

    0
}

/*
 @ Brief: Draws the mandelbrot set.
 @ Params: {
        width: Width of the image buffer.
        height: Height of the image buffer.
        capture_size: The radius of the circle in the complex plane that we bound some 'z' to.
        scale: The size of the mandelbrot set rendering.
   }
 * Return: The image buffer state after the mandelbrot set is written.
*/

fn draw_mandelbrot(
    width: u32, 
    height: u32, 
    capture_size: f64,
    max_iterations: usize,
    scale: f64,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut image_buffer = ImageBuffer::new(width, height);
    image_buffer
        .enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel_color)| {
            let c = Complex::new(
                (x as f64 - width as f64 / capture_size) * scale,
                (y as f64 - height as f64 / capture_size) * scale
            );

            let color = compute_color(c, max_iterations, capture_size);
            *pixel_color = Rgb([color, color, color]);
        });

    image_buffer
}

/*
 @ Brief: Parses a string that looks like HxW and returns (H, W)
 @ Params: resolution_str: The string to be parsed.
 @ Return: Returns a result of the pair of dimensions or an error.
*/

fn parse_resolution<T>(resolution_str: T) -> Result<(u32, u32), &'static str> 
where T: AsRef<str> {
    let resolution_str = resolution_str.as_ref();
    let mut resolution = resolution_str.split("x");

    let height = resolution.next().ok_or("Invalid height!")?;
    let width  = resolution.next().ok_or("Invalid width!")?;

    if resolution.next().is_some() {
        return Err("Invalid resolution string!");
    }

    Ok(
        (
            height.trim().parse().map_err(|_| "Invalid height!")?,
            width.trim().parse().map_err(|_|  "Invalid width!")?
        )
    )
}

fn main() -> Result<(), Box<dyn Error + 'static>> { 
    let args: Vec<_> = args().collect();

    if args.len() != 6 {
        eprintln!("Usage: file resolution capture_size max_iterations scale\n\
                   Example: mandelbrot.png 500x500 2.0 255 3.0");
    
        exit(0);
    }

    let (height, width) = parse_resolution(&args[2])?;
    let capture_size = args[3].trim().parse::<f64>()?;
    let max_iterations = args[4].trim().parse::<usize>()?;
    let scale = args[5].trim().parse::<f64>()? / width as f64;

    let image = draw_mandelbrot(width, height, capture_size, max_iterations, scale);

    image.save(&args[1])?;

    Ok(())
}

