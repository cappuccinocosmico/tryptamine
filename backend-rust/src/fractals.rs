pub mod images_fractal {

    use num_complex::Complex;
    use palette::{LinSrgb, Srgb};

    fn generate_color_gradient(size: i32) -> Vec<image::Rgb<u8>> {
        let mut array = Vec::new();
        for i in 0..size {
            array.push(image::Rgb([0, 0, 0]));
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
        let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

        // A redundant loop to demonstrate reading image data
        for x in 0..imgx {
            for y in 0..imgy {
                let cx = y as f32 * scalex - 1.5;
                let cy = x as f32 * scaley - 1.5;

                let c = seed_value;
                let mut z = num_complex::Complex::new(cx, cy);

                let mut i = 0;
                while i < 300 && z.norm() <= 2.0 {
                    z = z * z + c;
                    i += 1;
                }
                fn render_iterations(iterator: i32, basin: u8) -> image::Rgb<u8> {
                    if iterator == 300 {
                        image::Rgb([0, 0, 0])
                    } else {
                        image::Rgb([(10 * iterator) as u8, 0, 0])
                    }
                }

                let pixel = imgbuf.get_pixel_mut(x, y);
                let image::Rgb(data) = *pixel;
                *pixel = render_iterations(i, 0);
            }
        }
        imgbuf
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
