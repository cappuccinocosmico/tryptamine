extern crate uom;

use uom::si::area::square_meter;
use uom::si::f64::{Area, Length, Ratio};
use uom::si::length::{centimeter, planck_length};
use uom::si::ratio::ratio;

fn entropy_of_black_hole_in_cm(cm_of_black_hole: f64) {
    // Physical constants
    let pi = std::f64::consts::PI;
    let lp = Length::new::<planck_length>(cm_of_black_hole); // Planck length
    let ln2 = std::f64::consts::LN_2;

    // Black hole parameters
    let radius = Length::new::<centimeter>(0.5); // 0.5 cm

    // Area of event horizon: 4 * π * r^2
    let area = 4.0 * pi * radius * radius;

    // Planck area (lp^2)
    let lp2 = lp * lp;

    // Bekenstein-Hawking entropy (in units of k_B)
    let entropy = area / (4.0 * lp2);

    // Number of bits: entropy / ln(2)
    let bits = entropy / ln2;

    // Print results with unit checks
    println!("Event horizon area: {:.5e} m²", area.get::<square_meter>());
    println!(
        "Bekenstein-Hawking entropy (in units of k_B): {:.3e}",
        entropy.get::<ratio>()
    );
    println!("Number of bits: {:.3e}", bits.get::<ratio>());
}
