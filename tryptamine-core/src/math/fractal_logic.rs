use core::slice;

use crate::math::{
    colors::{generate_circular_purple_dream, generate_rainbow_gradient},
    fractal_definitions::{Compl, ComplexFatouFractal, FatouBasins, RealType},
};
use image::Rgb;
use num_complex::Complex;

use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use tracing::debug;

use super::colors::{CustomRgb, RgbColorScheme};

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
            };
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
    pub pixel_ratio: f64,
    pub center_cord: Compl,
    pub window_diagonal: RealType,
}

impl Default for ImageSchema {
    fn default() -> Self {
        ImageSchema {
            resolution_x: 2000,
            resolution_y: 1000,
            pixel_ratio: 1.0,
            center_cord: Complex::new(0.0, 0.0),
            window_diagonal: 4.0,
        }
    }
}

impl ImageSchema {
    fn index_to_val(&self, index: u32) -> Compl {
        let pixel_distance_x = self.window_diagonal
            / ((self.resolution_x as f64).powi(2)
                + (self.resolution_y as f64 * self.pixel_ratio).powi(2))
            .sqrt();
        let top_corner = Complex::new(
            self.center_cord.re - (pixel_distance_x * (self.resolution_x as RealType) / 2.0),
            self.center_cord.im
                + (pixel_distance_x * self.pixel_ratio * (self.resolution_y as RealType) / 2.0),
        );
        // let top_corner = Complex::new(self.center_cord.re, self.center_cord.im);
        let x = index % self.resolution_x;
        let y = index / self.resolution_x;
        let re = x as RealType * pixel_distance_x + top_corner.re;
        let im = y as RealType * pixel_distance_x * self.pixel_ratio - top_corner.im;

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
        // generate_circular_ocean_blues(color_size),
        generate_circular_purple_dream(color_size),
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
    debug!("{basins:?}");
    // Create the image buffer with parallel iterator
    let buff_length = (image_info.get_resolution() * 3) as usize; // Since there are 3 colors and stuff.
    let mut buff: Vec<u8> = vec![0; buff_length];
    let scary_buffs_list = split_vec_into_mutable_sized_chunks(&mut buff, 3).unwrap();
    let duration = start.elapsed();
    debug!("Initialization took: {duration:?}");
    let start = std::time::Instant::now();
    let _result: Vec<()> = scary_buffs_list
        .into_par_iter()
        .enumerate()
        .map(|(index, buff)| iteratior_mutate((index as u32, buff)))
        .collect();

    let duration = start.elapsed();
    debug!("Fractal Mathematics took: {duration:?}");

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
            debug!("WebP encoding took: {:?}", duration_webp);
            let duration = start.elapsed();
            debug!("Total Image generation took: {:?}", duration);
            Ok(webp)
        }
        ImageType::Jpeg => {
            let start_png = std::time::Instant::now();
            let png: Vec<u8> = image_buffer_to_jpeg_bytes(img);
            let duration_png = start_png.elapsed();
            debug!("Jpeg encoding took: {:?}", duration_png);
            let duration = start.elapsed();
            debug!("Total Image generation took: {:?}", duration);
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
