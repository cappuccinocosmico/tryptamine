fn oklch_to_rgb(L: f64, C: f64, H: f64) -> (f64, f64, f64) {
    // Convert LCH to Lab
    let a = C * H.cos();
    let b = C * H.sin();
    let lab_l = L;
    let lab_a = a;
    let lab_b = b;

    // Convert Lab to linear sRGB
    let l_ = lab_l + 0.3963377774 * lab_a + 0.2158037573 * lab_b;
    let m_ = lab_l - 0.1055613458 * lab_a - 0.0638541728 * lab_b;
    let s_ = lab_l - 0.0894841775 * lab_a - 1.291485548 * lab_b;

    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;

    let r = 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s;
    let g = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s;
    let b = -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s;

    // Apply gamma correction
    fn linear_to_srgb(c: f64) -> f64 {
        if c >= 1.0 {
            1.0
        } else if c <= 0.0 {
            0.0
        } else if c <= 0.0031308 {
            12.92 * c
        } else {
            1.055 * c.powf(1.0 / 2.4) - 0.055
        }
    }

    return (linear_to_srgb(r), linear_to_srgb(g), linear_to_srgb(b));
}
