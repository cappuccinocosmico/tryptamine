use std::ops::{Add, Div, Mul, Sub};

use crate::math::colors::{generate_rainbow_gradient, generate_warm_reds};
use image::Rgb;
use num_complex::Complex;
use num_traits::{Num, Zero};
use palette::encoding::gamma::Number;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use smallvec::{SmallVec, smallvec};

use super::colors::{CustomRgb, RgbColorScheme};

type RealType = f64;
type Compl = Complex<RealType>;
#[derive(Clone, Copy, Debug)]
struct FiniteFatouBasin {
    basin: Compl,
    neighborhood_sqr: RealType,
}

const SMALL_SIZE: usize = 5;
#[derive(Clone, Debug)]
struct FatouBasins {
    infinte_basin_radius_sqr: Option<RealType>,
    finite_basins: SmallVec<[FiniteFatouBasin; SMALL_SIZE]>,
}

trait ComplexFatouFractal {
    fn generate_fatou_basins(&self) -> FatouBasins;
    fn iterate_mut(&self, collector: &mut Compl);
}

struct RegularJuliaSet {
    c: Compl,
}

impl ComplexFatouFractal for RegularJuliaSet {
    fn iterate_mut(&self, collector: &mut Compl) {
        *collector = *collector * *collector + self.c
    }

    fn generate_fatou_basins(&self) -> FatouBasins {
        let mut finite_basins = smallvec![];
        // x+ = x^2 + c
        // 0 = x^2 -x + c
        // 0 = (x^2-x+1/4)-1/4+c
        // 0 = (x-1/2)^2-1/4+c
        // 1/4 -c = (x-1/2)^2
        // \pm sqrt(1/4-c) = x-1/2
        // x = 1/2 \pm sqrt(1/4 - c)
        let val1 = 1.0 / 2.0 + (1.0 / 4.0 - self.c).sqrt();
        let val2 = 1.0 / 2.0 - (1.0 / 4.0 - self.c).sqrt();
        fn validate_basin(
            val: Complex<RealType>,
            basins: &mut SmallVec<[FiniteFatouBasin; SMALL_SIZE]>,
        ) {
            println!("validating basin: {}", val);
            let valprime = 2.0 * val;
            println!(
                "basin has derivative: {}, with norm: {}",
                valprime,
                valprime.norm()
            );
            let is_valid = valprime.norm() <= 1.0;
            if is_valid {
                basins.push(FiniteFatouBasin {
                    basin: val,
                    neighborhood_sqr: 0.001,
                });
            }
        }
        validate_basin(val1, &mut finite_basins);
        validate_basin(val2, &mut finite_basins);
        FatouBasins {
            infinte_basin_radius_sqr: Some(4.0),
            finite_basins,
        }
    }
}

enum BasinIndex {
    Infinite,
    Finite(usize),
}
impl From<BasinIndex> for usize {
    fn from(value: BasinIndex) -> Self {
        match value {
            BasinIndex::Infinite => 0,
            BasinIndex::Finite(val) => val + 1,
        }
    }
}

fn generate_basins_conditional(basins: &FatouBasins) -> impl Fn(Compl) -> Option<BasinIndex> {
    // Create a single closure that directly evaluates all conditions
    move |z: Compl| {
        // Check escape condition first (most likely to be true in typical Julia sets)
        if let Some(escape_radius) = basins.infinte_basin_radius_sqr {
            if z.norm_sqr() >= escape_radius {
                return Some(BasinIndex::Infinite);
            };
        };

        // Then check all attraction basins
        for (i, basin) in basins.finite_basins.iter().enumerate() {
            let basin_val = basin.basin;
            if (z - basin_val).norm_sqr() <= basin.neighborhood_sqr {
                return Some(BasinIndex::Finite(i));
            }
        }
        return None;
    }
}

fn render_iterations(
    iterator: u32,
    basin: BasinIndex,
    color_schemes: &[RgbColorScheme],
) -> CustomRgb {
    if iterator == 300 {
        CustomRgb {
            red: 0,
            green: 0,
            blue: 0,
        }
    } else {
        let basin_usize: usize = basin.into();
        let scheme = &color_schemes[basin_usize];
        scheme[iterator as usize % scheme.len()]
    }
}
pub fn generate_julia_image(
    fractal: impl ComplexFatouFractal,
    imgx: u32,
    imgy: u32,
) -> Result<image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, String> {
    //! An example of generating julia fractals.
    let start = std::time::Instant::now();

    let scalex = 3.0 / imgx as RealType;
    let scaley = 3.0 / imgy as RealType;

    // Create a new ImgBuf with width: imgx and height: imgy
    // Move render_iterations outside the loop
    let color_size: usize = 10;
    let color_schemes = [
        generate_rainbow_gradient(color_size),
        generate_warm_reds(color_size),
    ];
    let basins = fractal.generate_fatou_basins();
    let basin_conditional = generate_basins_conditional(&basins);

    let iterator = |index: u32| -> CustomRgb {
        let x = index % imgx;
        let y = index / imgx;
        let cx = y as RealType * scalex - 1.5;
        let cy = x as RealType * scaley - 1.5;

        let mut z = Complex::new(cx, cy);

        let mut i = 0;

        while i < 300 {
            fractal.iterate_mut(&mut z);
            i += 1;
            if basin_conditional(z).is_some() {
                break;
            }
        }
        let basin_id =
            basin_conditional(z).expect("Just computed this previously and got a some variant");
        if i == 300 {
            println!("Pixel ({z}) ended iterator at no escape point");
        }
        render_iterations(i, basin_id, &color_schemes)
    };

    let iteratior_mutate = |(index, output_buf): (u32, &mut [u8; 3])| {
        let color = iterator(index);
        output_buf[0] = color.red;
        output_buf[1] = color.green;
        output_buf[2] = color.blue;
    };
    println!("{basins:?}");
    // Create the image buffer with parallel iterator
    let buff_length = (imgx * imgy * 3) as usize; // Since there are 3 colors and stuff.
    let mut buff: Vec<u8> = vec![0; buff_length];
    // I want to go ahead and split up this vec into an iterator that will throw out imgx*imgy mutable buffers of size 3, as well as an index that will denote what number the pixel is at. Then call iteratior_mutate on the tuple. If you could use par iters from rayon to speed this computation up that would also be really helpful.
    //
    buff.par_chunks_exact_mut(3)
        .enumerate()
        .for_each(|(index, chunk)| {
            let output_buf: &mut [u8; 3] = chunk.try_into().unwrap();
            iteratior_mutate((index as u32, output_buf));
        });
    let duration = start.elapsed();
    println!("Initialization took: {:?}", duration);

    let start = std::time::Instant::now();
    let duration = start.elapsed();
    println!("Fractal Mathematics took: {:?}", duration);

    // Calculate expected buffer size
    let expected_size = (imgx * imgy * 3) as usize;

    // Validate buffer size
    if buff.len() != expected_size {
        return Err(format!(
            "Buffer size mismatch. Expected {} bytes ({}x{}x3), got {} bytes",
            expected_size,
            imgx,
            imgy,
            buff.len()
        ));
    }

    let start = std::time::Instant::now();
    let img_option = image::ImageBuffer::from_vec(imgx, imgy, buff);
    let img: image::ImageBuffer<Rgb<u8>, Vec<u8>> = match img_option {
        Some(img) => img,
        None => {
            return Err(format!(
                "Failed to create image buffer with dimensions {}x{}",
                imgx, imgy
            ));
        }
    };
    let duration = start.elapsed();
    println!("Image buffer creation took: {:?}", duration);
    Ok(img)
}

fn split_vec_into_mutable_sized_chunks<T>(
    list: &mut [T],
    size: usize,
) -> Result<Vec<(u32, &mut [T])>, String> {
    if list.len() % size != 0 && size != 0 {
        return Err("List of improper size".to_string());
    };
    let capacity = list.len() / size;
    let mut result = Vec::with_capacity(capacity);
    for i in 0..capacity {
        result[i] = (i as u32, &mut list[size * i..size * (i + 1) - 1])
    }

    Ok(result)
}

fn split_vec_into_immutable_sized_chunks<T: Default>(
    list: &mut [T],
    size: usize,
) -> Result<Vec<(u32, &[T])>, String> {
    if list.len() % size != 0 && size != 0 {
        return Err("List of improper size".to_string());
    };
    let capacity = list.len() / size;
    let mut result = Vec::with_capacity(capacity);
    for i in 0..capacity {
        result[i] = (i as u32, &list[size * i..size * (i + 1) - 1])
    }
    list[1] = T::default();

    Ok(result)
}

pub enum ImageType {
    Jpeg,
    Webp,
}
pub fn str_image_extension(image_type_str: &str) -> Option<ImageType> {
    match image_type_str {
        "jpeg" => Some(ImageType::Jpeg),
        "webp" => Some(ImageType::Webp),
        _ => None,
    }
}

fn image_buffer_to_webp_bytes(buffer: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    buffer
        .write_to(
            &mut std::io::Cursor::new(&mut bytes),
            image::ImageFormat::WebP,
        )
        .expect("Failed to encode image as Webp");
    bytes
}

fn image_buffer_to_jpeg_bytes(buffer: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    buffer
        .write_to(
            &mut std::io::Cursor::new(&mut bytes),
            image::ImageFormat::Jpeg,
        )
        .expect("Failed to encode image as Jpeg");
    bytes
}
pub fn test_image(resolution: u32, image_type: ImageType) -> Result<Vec<u8>, String> {
    let start = std::time::Instant::now();
    let fractal = RegularJuliaSet {
        c: Complex::new(-0.3, 0.4),
    };
    let img = generate_julia_image(fractal, resolution, resolution)?;
    match image_type {
        ImageType::Webp => {
            let start_webp = std::time::Instant::now();
            let webp: Vec<u8> = image_buffer_to_webp_bytes(img);
            let duration_webp = start_webp.elapsed();
            println!("WebP encoding took: {:?}", duration_webp);
            let duration = start.elapsed();
            println!("Total Image generation took: {:?}", duration);
            Ok(webp)
        }
        ImageType::Jpeg => {
            let start_png = std::time::Instant::now();
            let png: Vec<u8> = image_buffer_to_jpeg_bytes(img);
            let duration_png = start_png.elapsed();
            println!("Jpeg encoding took: {:?}", duration_png);
            let duration = start.elapsed();
            println!("Total Image generation took: {:?}", duration);
            Ok(png)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ComplexSph<T> {
    Infinity,
    Number(Complex<T>),
}

impl<T: Num + Add + Clone> Add for ComplexSph<T> {
    type Output = ComplexSph<T>;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(sn), Self::Number(rhsn)) => Self::Number(sn + rhsn),
            (_, _) => Self::Infinity,
        }
    }
}

impl<T: Num + Sub + Clone> Sub for ComplexSph<T> {
    type Output = ComplexSph<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(sn), Self::Number(rhsn)) => Self::Number(sn - rhsn),
            (_, _) => Self::Infinity,
        }
    }
}

impl<T: Num + Mul + Clone + Zero> Mul for ComplexSph<T> {
    type Output = ComplexSph<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(sn), Self::Number(rhsn)) => Self::Number(sn * rhsn),
            (_, _) => Self::Infinity,
        }
    }
}

impl<T: Num + Div + Clone + Zero> Div for ComplexSph<T> {
    type Output = ComplexSph<T>;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Number(sn), Self::Number(rhsn)) => Self::Number(sn / rhsn),
            (Self::Number(_), Self::Infinity) => Self::Number(Complex::<T>::zero()),
            (_, _) => Self::Infinity,
        }
    }
}
