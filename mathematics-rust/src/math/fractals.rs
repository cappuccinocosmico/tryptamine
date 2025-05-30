use core::slice;
use std::ops::{Add, Div, Mul, Sub};

use crate::math::colors::{generate_rainbow_gradient, generate_warm_reds};
use image::Rgb;
use num_complex::Complex;
use num_traits::{Num, Zero};
use palette::encoding::gamma::Number;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
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
pub struct FatouBasins {
    infinte_basin_radius_sqr: Option<RealType>,
    finite_basins: SmallVec<[FiniteFatouBasin; SMALL_SIZE]>,
}

pub trait ComplexFatouFractal: Copy + Sync {
    fn generate_fatou_basins(&self) -> FatouBasins;
    fn iterate_mut(&self, collector: &mut Compl);
}

#[derive(Debug, Clone, Copy)]
pub struct RegularJuliaSet {
    pub c: Compl,
}

impl Default for RegularJuliaSet {
    fn default() -> Self {
        Self {
            c: Complex::new(0.2, 0.3),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SinJuliaSet {
    pub c: Compl,
}

impl Default for SinJuliaSet {
    fn default() -> Self {
        Self {
            c: Complex::new(0.2, 0.3),
        }
    }
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
impl ComplexFatouFractal for SinJuliaSet {
    fn iterate_mut(&self, collector: &mut Compl) {
        *collector = (*collector).sin() + self.c
    }

    fn generate_fatou_basins(&self) -> FatouBasins {
        FatouBasins {
            infinte_basin_radius_sqr: Some(8.0),
            // TODO: Write code to generate interior basins
            finite_basins: smallvec![],
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
        None
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
pub fn generate_julia_image<F: ComplexFatouFractal>(
    fractal: F,
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

    let iteratior_mutate = |(index, output_buf): (u32, &mut [u8])| {
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
    let scary_buff = split_vec_into_mutable_sized_chunks(&mut buff, 3).unwrap();
    let _ = scary_buff
        .into_par_iter()
        .enumerate()
        .map(|(index, buff)| iteratior_mutate((index as u32, buff)));
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
    chunk_size: usize,
) -> Result<Vec<&mut [T]>, String> {
    if list.len() % chunk_size != 0 && chunk_size != 0 {
        return Err("List of improper size".to_string());
    };
    let capacity = list.len() / chunk_size;
    let mut result = Vec::with_capacity(capacity);
    let ptr = list.as_mut_ptr();
    unsafe {
        for i in 0..capacity {
            let mut_refrence: &mut [T] =
                slice::from_raw_parts_mut(ptr.add(i * chunk_size), chunk_size - 1);
            result.push(mut_refrence)
        }
    }
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
pub fn test_image<F: ComplexFatouFractal>(
    resolution: u32,
    image_type: ImageType,
    fractal_config: F,
) -> Result<Vec<u8>, String> {
    let start = std::time::Instant::now();
    let img = generate_julia_image(fractal_config, resolution, resolution)?;
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
