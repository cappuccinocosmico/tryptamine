extern crate uom;

use uom::si::ratio::ratio;
use uom::si::velocity::natural_unit_of_velocity;
use uom::si::{
    action::reduced_planck_constant,
    area::square_meter,
    f64::{Action, Length, Mass, Time, Velocity, Volume},
    length::{centimeter, meter},
    mass::kilogram,
    time::second,
    volume::cubic_meter,
};
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
    // let planklength_meters = 1.1616 * 10_f64.powf(-35_f64);
    // let lp = Length::new::<meter>(planklength_meters);
    let hbar = Action::new::<reduced_planck_constant>(1.0);
    let c = Velocity::new::<natural_unit_of_velocity>(1.0);
    let G = 6.67430 * 10.0_f64.powi(-11) * Volume::new::<cubic_meter>(1.0)
        / (Time::new::<second>(1.0) * Mass::new::<kilogram>(1.0));
    let plank_area = (hbar * G) / (c * c * c);
    // println!("Test quantity equal to plank area: {}", plank_area);
    // let plank_length = plank_area.sqrt();

    let ln2 = std::f64::consts::LN_2;

    // Black hole parameters
    let radius = Length::new::<centimeter>(cm_of_black_hole);

    // Area of event horizon: 4 * π * r^2
    // let area = 4.0 * pi * radius * radius;

    // Bekenstein-Hawking entropy (in units of k_B)
    // let entropy = area / (4.0 * lp2);

    // Number of bits: entropy / ln(2)
    // let bits = entropy / ln2;

    // Print results with unit checks
    println!("Event horizon area: {:.5e} m²", 1.0);
    println!("Bekenstein-Hawking entropy (in units of k_B): {:.3e}", 3.0);
    println!("Number of bits: {:.3e}", 3.0);
}
