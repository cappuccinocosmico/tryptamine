mod julia {
    fn generate_julia_image() -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        //! An example of generating julia fractals.
        let imgx = 800;
        let imgy = 800;

        let scalex = 3.0 / imgx as f32;
        let scaley = 3.0 / imgy as f32;

        // Create a new ImgBuf with width: imgx and height: imgy
        let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let r = (0.3 * x as f32) as u8;
            let b = (0.3 * y as f32) as u8;
            *pixel = image::Rgb([r, 0, b]);
        }

        // A redundant loop to demonstrate reading image data
        for x in 0..imgx {
            for y in 0..imgy {
                let cx = y as f32 * scalex - 1.5;
                let cy = x as f32 * scaley - 1.5;

                let c = num_complex::Complex::new(-0.4, 0.6);
                let mut z = num_complex::Complex::new(cx, cy);

                let mut i = 0;
                while i < 255 && z.norm() <= 2.0 {
                    z = z * z + c;
                    i += 1;
                }

                let pixel = imgbuf.get_pixel_mut(x, y);
                let image::Rgb(data) = *pixel;
                *pixel = image::Rgb([data[0], i as u8, data[2]]);
            }
        }
        imgbuf
    }
}

pub mod images_fractal {
    fn generate_test_image() -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let width = 3000;
        let height = 3000;

        let mut imgbuf = image::ImageBuffer::new(width, height);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let intensity =
                ((x as f32 / width as f32 + y as f32 / height as f32) / 2.0 * 255.0) as u8;
            *pixel = image::Rgb([intensity, intensity, intensity]);
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
        let img = generate_test_image();
        image_buffer_to_webp_bytes(img)
    }
}
