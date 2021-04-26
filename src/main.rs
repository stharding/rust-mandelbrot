use clap::{App, Arg};
use image::png::PngEncoder;
use image::ColorType;
use num::Complex;
use std::fs::File;

fn affine(floor_in: f64, val: f64, ceil_in: f64, floor_out: f64, ceil_out: f64) -> f64 {
    ((val - floor_in) / (ceil_in - floor_in)) * (ceil_out - floor_out) + floor_out
}

fn point_z_value(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1);
    for row in 0..bounds.0 {
        for column in 0..bounds.1 {
            let point = Complex {
                re: affine(
                    0.0,
                    column as f64,
                    bounds.0 as f64,
                    upper_left.im,
                    lower_right.im,
                ),
                im: affine(
                    0.0,
                    row as f64,
                    bounds.1 as f64,
                    upper_left.re,
                    lower_right.re,
                ),
            };
            pixels[row * bounds.1 + column] = match point_z_value(point, 255) {
                None => 0,
                Some(z_value) => 255 - z_value as u8,
            }
        }
    }
}

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), image::ImageError> {
    let output = File::create(filename)?;
    let encoder = PngEncoder::new(output);
    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::L8)?;
    Ok(())
}

fn main() {
    let matches = App::new("Mandelbrot Generator")
        .version("0.1")
        .author("Stephen Harding")
        .about(("Generates a PNG image of the mandlebrot set"))
        .arg(
            Arg::with_name("dimensions")
                .short("d")
                .long("dimensions")
                .default_value("1000x1000"),
        )
        .arg(
            Arg::with_name("upper-left")
                .short("l")
                .long("upper-left")
                .default_value("-2.0+1.5i"),
        )
        .arg(
            Arg::with_name("lower-right")
                .short("r")
                .long("lower-right")
                .default_value("1.5-1.5i"),
        )
        .get_matches();
    println!("{:?}", matches);

    let dimensions = (1000, 1000);
    let mut pixels = vec![0; dimensions.0 * dimensions.1];
    let ul = Complex { re: 1.2, im: 0.35 };
    let lr = Complex { re: 1.0, im: 0.2 };
    render(&mut pixels, dimensions, ul, lr);
    write_image("out.png", &pixels, dimensions).unwrap();
}
