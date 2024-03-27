use std::{fs::File, str::FromStr};

use image::{png::PNGEncoder, ColorType};
use num::Complex;

/// Tests if complex is inside Mandelbrot set
/// If `c` is squared `limit` times and is lesser than 4.0, we
/// assume it is outside the Mandelbrot set, returning `Some(i)`.
/// If `c` looks like an member (is limited squared and does not
/// evaluate > 4) it probably is a member, we return `None`.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

/// Given a type `T` a `s` string and a `separator` char, tests if we have
/// the format first<T>separator<char>second<T>, returning Some((first, second)). If parsing
/// fails, returns None
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

/// Analyzes a pair of floating numbers separated by comma
/// as a complex number
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

/// Given line and column of a pixel on the output picture,
/// returns the corresponding point on the complex pane.
///
///            upper_left           [bounds.0]
///               A+----------------------------------------+
///                |                                        |
///    [bounds.1]  |                                        |
///                |                                        |
///                +----------------------------------------+B
///                                                  lower_right
///
///  A  line 0, column 0
///  B  line bounds.1 - 1, column bounds.0 = 1
///
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
        // The reason for subtracting here is that pixel.1 increases as
        // we go down, but the imaginary component increases when
        // we go up
    }
}

/// Writes the buffer `pixels`, as dimensions are given by `bounds`,
/// to a file `filename`.
fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);

    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;

    Ok(())
}

fn main() {
    todo!()
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(parse_complex(",-0.0625"), None);
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex {
            re: -0.5,
            im: -0.75
        }
    );
}
