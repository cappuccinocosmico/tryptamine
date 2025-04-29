extern crate uom;

use uom::si::area::square_meter;
use uom::si::f64::{Area, Length, Ratio};
use uom::si::length::{centimeter, meter};
use uom::si::ratio::ratio;

fn entropy_of_black_hole_in_cm(cm_of_black_hole: f64) {
    // Physical constants
    let pi = std::f64::consts::PI;
    // Calculate Planck length using √(ℏG/c³)
    // let G = uom::si::f64::COULOMB_CONSTANT; // Gravitational constant (m³·kg⁻¹·s⁻²)
    // let ħ = uom::si::f64::PLANCK_CONSTANT; // Reduced Planck constant (J·s)
    // let c = uom::si::f64::SPEED_OF_LIGHT; // Speed of light (m/s)

    // let lp = (G * ħ / c.powi(3))
    //     .sqrt()
    //     .expect("Physics constants should lead to valid Planck length");
    //
    let planklength_meters = 1.1616 * 10_f64.powf(-35_f64);
    let lp = Length::new::<meter>(planklength_meters);
    let ln2 = std::f64::consts::LN_2;

    // Black hole parameters
    let radius = Length::new::<centimeter>(cm_of_black_hole);

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
