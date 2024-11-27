pub mod images_fractal {

    use num_complex::Complex;
    use palette::{num::Round, IntoColor, Oklch, Srgb};
    use rayon::iter::{IntoParallelIterator, ParallelIterator};
    use std::num;
    fn srgb_to_rgbvals(srgb: Srgb<f32>) -> [u8; 3] {
        [
            (srgb.red * 256.0).floor() as u8,
            (srgb.green * 256.0).floor() as u8,
            (srgb.blue * 256.0).floor() as u8,
        ]
    }

    fn generate_color_gradient(size: i32) -> Vec<[u8; 3]> {
        let mut array = Vec::new();
        for i in 0..size {
            let oklch_color = Oklch::new(0.7, 0.2, ((i * 30) % 360) as f32);
            let srgb_color_float: Srgb<f32> = oklch_color.into_color();
            let im_rgb = srgb_to_rgbvals(srgb_color_float);
            array.push(im_rgb);
        }
        array
    }

    struct JuliaBasin {
        basin: Option<Complex<f32>>,
        neighborhood: f32,
    }

    fn generate_julia_basins(c: Complex<f32>) -> Vec<JuliaBasin> {
        let mut basins = vec![JuliaBasin {
            basin: None,
            neighborhood: 2.0,
        }];
        // x+ = x^2 + c
        // 0 = x^2 -x + c
        // 0 = (x^2-x+1/4)-1/4+c
        // 0 = (x-1/2)^2-1/4+c
        // 1/4 -c = (x-1/2)^2
        // \pm sqrt(1/4-c) = x-1/2
        // x = 1/2 \pm sqrt(1/4 - c)
        let val1 = 1.0 / 2.0 + (1.0 / 4.0 - c).sqrt();
        let val2 = 1.0 / 2.0 - (1.0 / 4.0 - c).sqrt();
        fn validate_basin(val: Complex<f32>, basins: &mut Vec<JuliaBasin>) {
            let valprime = 2.0 * val - 1.0;
            let is_valid = valprime.norm() <= 1.0;
            if is_valid {
                basins.push(JuliaBasin {
                    basin: Some(val),
                    neighborhood: 0.001,
                });
            }
        }
        validate_basin(val1, &mut basins);
        validate_basin(val2, &mut basins);
        basins
    }
    fn generate_basins_conditional( basins : &Vec<JuliaBasin>) ->  {}

    fn generate_julia_image(
        imgx: u32,
        imgy: u32,
        seed_value: Complex<f32>,
    ) -> Result<Vec<u8>, String> {
        //! An example of generating julia fractals.

        let scalex = 3.0 / imgx as f32;
        let scaley = 3.0 / imgy as f32;

        // Create a new ImgBuf with width: imgx and height: imgy
        // Move render_iterations outside the loop
        let colors = generate_color_gradient(10);
        let render_iterations = |iterator: i32, basin: u8| -> [u8; 3] {
            if iterator == 300 {
                [0, 0, 0]
            } else {
                colors[iterator as usize % colors.len()]
            }
        };
        let basins = generate_julia_basins(seed_value);

        let iterator = |index: u32| -> [u8; 3] {
            let x = index % imgx;
            let y = index / imgx;
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;


            let mut z = Complex::new(cx, cy);

            let mut i = 0;
            
            while i < 300 {
                z = z * z + seed_value;
                i += 1;
                for basin in &basins {
                    if z.norm() <= basin.neighborhood {
                        return render_iterations(i, 0);
                    }
                }
            }

            render_iterations(i, 0)
        };
        // Create the image buffer with parallel iterator
        let start = std::time::Instant::now();
        let buff_pixels: Vec<[u8; 3]> = (0..imgx * imgy).into_par_iter().map(iterator).collect();
        let duration = start.elapsed();
        println!("Fractal Mathematics took: {:?}", duration);

        let start = std::time::Instant::now();
        let buff: Vec<u8> = buff_pixels.iter().flat_map(|x| x.iter()).copied().collect();
        let duration = start.elapsed();
        println!("Buffer flattening took: {:?}", duration);

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
        let img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = match img_option {
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

        let start = std::time::Instant::now();
        let webp: Vec<u8> = image_buffer_to_webp_bytes(img);
        let duration = start.elapsed();
        println!("WebP encoding took: {:?}", duration);
        Ok(webp)
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
    pub fn test_webp() -> Result<Vec<u8>, String> {
        let start = std::time::Instant::now();
        let result = generate_julia_image(3000, 3000, Complex::new(-0.3, 0.4));
        let duration = start.elapsed();
        println!("Total Image generation took: {:?}", duration);
        result
    }
}
