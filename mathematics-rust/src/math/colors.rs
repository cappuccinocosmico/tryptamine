use palette::{IntoColor, Oklch, Srgb, num::Round};
fn srgb_to_rgbvals(srgb: Srgb<f32>) -> [u8; 3] {
    [
        (srgb.red * 256.0).floor() as u8,
        (srgb.green * 256.0).floor() as u8,
        (srgb.blue * 256.0).floor() as u8,
    ]
}

fn generate_generic_gradient(
    size: usize,
    color_generator: impl Fn(usize) -> Oklch,
) -> Vec<[u8; 3]> {
    let mut array = Vec::new();
    for i in 0..size {
        let oklch_color = color_generator(i);
        let srgb_color_float: Srgb<f32> = oklch_color.into_color();
        let im_rgb = srgb_to_rgbvals(srgb_color_float);
        array.push(im_rgb);
    }
    array
}

pub fn generate_rainbow_gradient(size: usize) -> Vec<[u8; 3]> {
    generate_generic_gradient(size, |i| {
        Oklch::new(0.7, 0.16, ((i + 4) * (360 / size) % 360) as f32)
    })
}

pub fn generate_warm_reds(size: usize) -> Vec<[u8; 3]> {
    generate_generic_gradient(size, |i| {
        Oklch::new(
            0.8 - ((i + 4) as f32 * 0.03) % size as f32,
            0.2,
            20.0 + (i as f32 * 10.0) % (10.0 * size as f32),
        )
    })
}

pub fn generate_forest_greens(size: usize) -> Vec<[u8; 3]> {
    generate_generic_gradient(size, |i| {
        Oklch::new(
            0.55 + (i as f32 * 0.03),
            0.15,
            140.0 + (i as f32 * 10.0) % 40.0,
        )
    })
}

pub fn generate_royal_violets(size: usize) -> Vec<[u8; 3]> {
    generate_generic_gradient(size, |i| {
        Oklch::new(
            0.6,
            0.18 + (i as f32 * 0.01),
            280.0 + (i as f32 * 15.0) % 40.0,
        )
    })
}

pub fn generate_ocean_blues(size: usize) -> Vec<[u8; 3]> {
    generate_generic_gradient(size, |i| {
        Oklch::new(
            0.7 - (i as f32 * 0.02),
            0.12 + (i as f32 * 0.01),
            220.0 + (i as f32 * 20.0) % 40.0,
        )
    })
}
