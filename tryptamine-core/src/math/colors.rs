use palette::{IntoColor, Oklch, Srgb, num::Round};
fn srgb_to_rgbvals(srgb: Srgb<f32>) -> CustomRgb {
    CustomRgb {
        red: (srgb.red * 256.0).floor() as u8,
        green: (srgb.green * 256.0).floor() as u8,
        blue: (srgb.blue * 256.0).floor() as u8,
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CustomRgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub trait TrypColor {
    fn lerp(&self, other: &Self, val: f32) -> Self;
    fn to_rgb(&self) -> CustomRgb;
}

impl TrypColor for CustomRgb {
    fn to_rgb(&self) -> CustomRgb {
        *self
    }
    fn lerp(&self, other: &Self, val: f32) -> Self {
        // Perform linear interpolation on each color channel with rounding
        let red = (self.red as f32 + val * (other.red as f32 - self.red as f32)).round() as u8;
        let green =
            (self.green as f32 + val * (other.green as f32 - self.green as f32)).round() as u8;
        let blue = (self.blue as f32 + val * (other.blue as f32 - self.blue as f32)).round() as u8;

        CustomRgb { red, green, blue }
    }
}
impl TrypColor for Oklch {
    fn lerp(&self, other: &Self, val: f32) -> Self {
        // Calculate shortest angular difference for hue interpolation
        let h1 = self.hue.into_positive_degrees();
        let h2 = other.hue.into_positive_degrees();
        let delta = h2 - h1;

        // Wrap delta to the range [-180, 180] to find shortest path
        let shortest_delta = if delta.abs() > 180.0 {
            if delta > 0.0 {
                delta - 360.0
            } else {
                delta + 360.0
            }
        } else {
            delta
        };

        fn lerp(a: f32, b: f32, c: f32) -> f32 {
            a * (1.0 - c) + b * c
        }

        Oklch::new(
            lerp(self.l, other.l, val),
            lerp(self.chroma, other.chroma, val),
            (h1 + shortest_delta * val).rem_euclid(360.0),
        )
    }
    fn to_rgb(&self) -> CustomRgb {
        let srgb_color_float: Srgb<f32> = (*self).into_color();
        let im_rgb = srgb_to_rgbvals(srgb_color_float);
        return im_rgb;
    }
}

pub type RgbColorScheme = Vec<CustomRgb>;
fn generate_generic_gradient<C: TrypColor>(
    size: usize,
    color_generator: impl Fn(usize) -> C,
) -> RgbColorScheme {
    let mut array = Vec::new();
    for i in 0..size {
        let output = color_generator(i).to_rgb();
        array.push(output);
    }
    array
}

pub fn generate_rainbow_gradient_constant_chroma(size: usize) -> Vec<CustomRgb> {
    generate_generic_gradient(size, |i| {
        Oklch::new(0.7, 0.16, ((i + 4) * (360 / size) % 360) as f32)
    })
}

pub fn generate_rainbow_gradient(size: usize) -> Vec<CustomRgb> {
    generate_generic_gradient(size, |i| {
        let hue = ((i + 4) * (360 / size) % 360) as f32;
        // Increase luminance for blue hues (~220 to 260 degrees), decrease for others
        // let luminance = if (220.0..=260.0).contains(&hue) {
        //     0.6 // higher luminance for blue colors
        // } else {
        //     0.8 // lower luminance for non-blue colors
        // };
        let luminance = 0.7;
        Oklch::new(luminance, 0.2, hue)
    })
}
// oklch(70.17% 0.322 328.36)
pub fn generate_lerp_gradient(size: usize, color_points: &[impl TrypColor]) -> Vec<CustomRgb> {
    let num_colors = color_points.len();
    if num_colors == 0 {
        return vec![CustomRgb::default(); size];
    }
    let colors_iterator = (0..size).map(|i| {
        let bottom_color_index = ((i as f32 / size as f32) * num_colors as f32).floor() as usize;
        let lerp_val = ((i as f32 / size as f32) * num_colors as f32).fract();
        let bottom_color = &color_points[bottom_color_index];
        let upper_color = &color_points[(bottom_color_index + 1) % num_colors];
        let intermediate = bottom_color.lerp(upper_color, lerp_val);
        intermediate.to_rgb()
    });
    colors_iterator.collect()
}
// oklch(40.7 0.1 297)
// oklch!(.407, 0.1, 297)

macro_rules! oklch {
    ($l:expr, $c:expr, $h:expr) => {
        Oklch::new($l as f32, $c as f32, $h as f32)
    };
}
pub fn generate_circular_ocean_blues(size: usize) -> Vec<CustomRgb> {
    // Generate a smooth ocean blue gradient using multiple lerp points
    let colors = [
        oklch!(0.5, 0.09, 220),  // deep blue
        oklch!(0.6, 0.10, 200),  // medium blue
        oklch!(0.75, 0.13, 185), // lighter blue
        oklch!(0.85, 0.12, 170), // pale blue
        oklch!(0.75, 0.13, 185), // lighter blue
        oklch!(0.6, 0.10, 200),  // medium blue
    ];
    generate_lerp_gradient(size, &colors)
}
pub fn generate_circular_sunset_orange(size: usize) -> Vec<CustomRgb> {
    let colors = [
        oklch!(0.7, 0.19, 30),  // warm orange
        oklch!(0.6, 0.2, 20),   // soft orange
        oklch!(0.55, 0.15, 10), // light peach
        oklch!(0.5, 0.13, 5),   // pale peach
        oklch!(0.6, 0.2, 20),   // soft orange
    ];
    generate_lerp_gradient(size, &colors)
}

pub fn generate_circular_forest_greens(size: usize) -> Vec<CustomRgb> {
    let colors = [
        oklch!(0.45, 0.1, 160), // dark green
        oklch!(0.5, 0.1, 140),  // medium green
        oklch!(0.65, 0.1, 120), // bright green
        oklch!(0.8, 0.1, 90),   // light green
        oklch!(0.65, 0.1, 120), // bright green
        oklch!(0.5, 0.1, 140),  // medium green
    ];
    generate_lerp_gradient(size, &colors)
}

pub fn generate_circular_purple_dream(size: usize) -> Vec<CustomRgb> {
    let colors = [
        oklch!(0.5, 0.25, 290), // deep purple
        oklch!(0.6, 0.21, 275), // violet
        oklch!(0.7, 0.15, 260), // lavender
        oklch!(0.6, 0.15, 245), // soft violet
        oklch!(0.7, 0.15, 260), // lavender
        oklch!(0.6, 0.21, 275), // violet
    ];
    generate_lerp_gradient(size, &colors)
}
