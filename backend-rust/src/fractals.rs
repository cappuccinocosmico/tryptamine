pub mod images_fractal {

    use image::Rgb as ImRgb;
    use num_complex::Complex;
    use palette::{num::Round, IntoColor, Oklch, Srgb};
    fn srgb_to_imrgb(srgb: Srgb<f32>) -> ImRgb<u8> {
        ImRgb([
            (srgb.red * 256.0).floor() as u8,
            (srgb.green * 256.0).floor() as u8,
            (srgb.blue * 256.0).floor() as u8,
        ])
    }

    fn generate_color_gradient(size: i32) -> Vec<ImRgb<u8>> {
        let mut array = Vec::new();
        for i in 0..size {
            let oklch_color = Oklch::new(0.5, 0.2, ((i * 10) % 360) as f32);
            let srgb_color_float: Srgb<f32> = oklch_color.into_color();
            let im_rgb: ImRgb<u8> = srgb_to_imrgb(srgb_color_float);
            array.push(im_rgb);
        }
        array
    }

    fn generate_julia_image(
        imgx: u32,
        imgy: u32,
        seed_value: Complex<f32>,
    ) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        //! An example of generating julia fractals.

        let scalex = 3.0 / imgx as f32;
        let scaley = 3.0 / imgy as f32;

        // Create a new ImgBuf with width: imgx and height: imgy
        // Move render_iterations outside the loop
        fn render_iterations(iterator: i32, colors: &[ImRgb<u8>]) -> image::Rgb<u8> {
            if iterator == 300 {
                image::Rgb([0, 0, 0])
            } else {
                colors[iterator as usize % colors.len()]
            }
        }

        let colors = generate_color_gradient(10);

        // Create the image buffer with parallel iterator
        image::ImageBuffer::from_fn(imgx, imgy, |x, y| {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let mut z = Complex::new(cx, cy);

            let mut i = 0;
            while i < 300 && z.norm() <= 2.0 {
                z = z * z + seed_value;
                i += 1;
            }

            render_iterations(i, &colors)
        })
    }

    fn image_buffer_to_webp_bytes(buffer: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        buffer
            .write_to(
                &mut std::io::Cursor::new(&mut bytes),
                image::ImageFormat::WebP,
            )
            .expect("Failed to encode image as PNG");
        bytes
    }
    pub fn test_webp() -> Vec<u8> {
        let img = generate_julia_image(600, 600, Complex::new(-0.3, 0.4));
        image_buffer_to_webp_bytes(img)
    }
}
