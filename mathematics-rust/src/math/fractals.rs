use core::slice;

use crate::math::colors::{generate_ocean_blues, generate_rainbow_gradient};
use image::Rgb;
use num_complex::Complex;
use num_traits::real::Real;
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

const DEFAULT_MAX_ITERATIONS: u32 = 300;
pub trait ComplexFatouFractal: Copy + Sync {
    fn generate_fatou_basins(&self) -> FatouBasins;
    fn iterate_mut(&self, collector: &mut Compl, original: &Compl);
    fn get_iterations(&self) -> u32;
}

#[derive(Debug, Clone, Copy)]
pub struct RegularJuliaSet {
    pub c: Compl,
    pub iterations: u32,
}

impl Default for RegularJuliaSet {
    fn default() -> Self {
        Self {
            c: Complex::new(0.2, 0.3),
            iterations: DEFAULT_MAX_ITERATIONS,
        }
    }
}
impl ComplexFatouFractal for RegularJuliaSet {
    fn iterate_mut(&self, collector: &mut Compl, _: &Compl) {
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
    fn get_iterations(&self) -> u32 {
        self.iterations
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SinJuliaSet {
    pub c: Compl,
    pub iterations: u32,
}

impl Default for SinJuliaSet {
    fn default() -> Self {
        Self {
            c: Complex::new(1.0, 0.1),
            iterations: DEFAULT_MAX_ITERATIONS,
        }
    }
}
impl ComplexFatouFractal for SinJuliaSet {
    fn iterate_mut(&self, collector: &mut Compl, _: &Compl) {
        *collector = (*collector).sin() * self.c
    }

    fn generate_fatou_basins(&self) -> FatouBasins {
        FatouBasins {
            infinte_basin_radius_sqr: Some(64.0),
            // TODO: Write code to generate interior basins
            finite_basins: smallvec![],
        }
    }
    fn get_iterations(&self) -> u32 {
        self.iterations
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MandelbrotSet {
    pub iterations: u32,
}
impl Default for MandelbrotSet {
    fn default() -> Self {
        MandelbrotSet {
            iterations: DEFAULT_MAX_ITERATIONS,
        }
    }
}

impl ComplexFatouFractal for MandelbrotSet {
    fn iterate_mut(&self, collector: &mut Compl, original: &Compl) {
        *collector = *collector * *collector + original;
    }
    fn generate_fatou_basins(&self) -> FatouBasins {
        FatouBasins {
            infinte_basin_radius_sqr: Some(8.0),
            // TODO: Write code to generate interior basins
            finite_basins: smallvec![],
        }
    }
    fn get_iterations(&self) -> u32 {
        self.iterations
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
    optional_basin: Option<BasinIndex>,
    color_schemes: &[RgbColorScheme],
) -> CustomRgb {
    let Some(basin) = optional_basin else {
        return CustomRgb {
            red: 0,
            green: 0,
            blue: 0,
        };
    };
    let basin_usize: usize = basin.into();
    let scheme = &color_schemes[basin_usize];
    scheme[iterator as usize % scheme.len()]
}

#[derive(Debug)]
pub struct ImageSchema {
    pub resolution_x: u32,
    pub resolution_y: u32,
    pub center_cord: Compl,
    pub window_diagonal: RealType,
}

impl Default for ImageSchema {
    fn default() -> Self {
        ImageSchema {
            resolution_x: 2000,
            resolution_y: 1000,
            center_cord: Complex::new(0.0, 0.0),
            window_diagonal: 0.5,
        }
    }
}

impl ImageSchema {
    fn index_to_val(&self, index: u32) -> Compl {
        let pixel_distance = self.window_diagonal
            / ((self.resolution_x.pow(2) + self.resolution_y.pow(2)) as RealType).sqrt();
        let top_corner = Complex::new(
            self.center_cord.re - (pixel_distance * (self.resolution_x as RealType) / 2.0),
            self.center_cord.im + (pixel_distance * (self.resolution_y as RealType) / 2.0),
        );
        // let top_corner = Complex::new(self.center_cord.re, self.center_cord.im);
        let x = index % self.resolution_x;
        let y = index / self.resolution_x;
        let re = x as RealType * pixel_distance + top_corner.re;
        let im = y as RealType * pixel_distance - top_corner.im;

        Complex::new(re, im)
    }
    fn get_resolution(&self) -> u32 {
        self.resolution_x * self.resolution_y
    }
}

pub fn generate_raw_image_buffer<F: ComplexFatouFractal>(
    fractal: &F,
    image_info: &ImageSchema,
) -> Vec<u8> {
    //! An example of generating julia fractals.
    let start = std::time::Instant::now();

    // Create a new ImgBuf with width: imgx and height: imgy
    // Move render_iterations outside the loop
    let color_size: usize = 10;
    let color_schemes = [
        generate_rainbow_gradient(color_size),
        generate_ocean_blues(color_size),
    ];
    let basins = fractal.generate_fatou_basins();
    let basin_conditional = generate_basins_conditional(&basins);

    let iterator = |index: u32| -> CustomRgb {
        // Complex implements copy if the underlying elements implement it, otherwise this would require a .clone()
        let original = image_info.index_to_val(index);
        let mut z = original;

        let mut i = 0;

        let max_iter = fractal.get_iterations();

        while i < max_iter {
            fractal.iterate_mut(&mut z, &original);
            i += 1;
            let cond = basin_conditional(z);
            if cond.is_some() {
                return render_iterations(i, cond, &color_schemes);
            };
        }
        render_iterations(max_iter, None, &color_schemes)
    };

    let iteratior_mutate = |(index, output_buf): (u32, &mut [u8])| {
        let color = iterator(index);
        output_buf[0] = color.red;
        output_buf[1] = color.green;
        output_buf[2] = color.blue;
    };
    println!("{basins:?}");
    // Create the image buffer with parallel iterator
    let buff_length = (image_info.get_resolution() * 3) as usize; // Since there are 3 colors and stuff.
    let mut buff: Vec<u8> = vec![0; buff_length];
    let scary_buffs_list = split_vec_into_mutable_sized_chunks(&mut buff, 3).unwrap();
    let duration = start.elapsed();
    println!("Initialization took: {:?}", duration);
    let start = std::time::Instant::now();
    let _result: Vec<()> = scary_buffs_list
        .into_par_iter()
        .enumerate()
        .map(|(index, buff)| iteratior_mutate((index as u32, buff)))
        .collect();

    let duration = start.elapsed();
    println!("Fractal Mathematics took: {:?}", duration);

    buff
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
                slice::from_raw_parts_mut(ptr.add(i * chunk_size), chunk_size);
            result.push(mut_refrence)
        }
    }
    Ok(result)
}

pub enum ImageType {
    Jpeg,
    Webp,
}
pub fn generate_image_bytes<F: ComplexFatouFractal>(
    resolution: u32,
    image_type: ImageType,
    fractal_config: F,
) -> Result<Vec<u8>, String> {
    let start = std::time::Instant::now();
    let image_info = ImageSchema {
        resolution_x: 2 * resolution,
        resolution_y: resolution,
        ..ImageSchema::default()
    };
    let buff = generate_raw_image_buffer(&fractal_config, &image_info);

    let img: image::ImageBuffer<Rgb<u8>, Vec<u8>> = match image::ImageBuffer::from_vec(
        image_info.resolution_x,
        image_info.resolution_y,
        buff,
    ) {
        Some(img) => img,
        None => {
            return Err(format!("Failed to create image buffer"));
        }
    };
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
