#[macro_use]
extern crate uom;

mod factorio;

use factorio::f64 as F;
use factorio::time::minute;

fn main() {
    let min = F::Time::new::<minute>(1.0);

    // --- STEP 1: Define Machine Recipes (Output per Machine per Minute) ---
    let machine_bio_to_nut_out =
        F::Nutrients::new::<factorio::nutrients::nutrients>(60.0 * 30.0) / min;
    let machine_bio_to_nut_in = F::Bioflux::new::<factorio::bioflux::bioflux>(150.0) / min;

    let machine_nut_to_egg_out = F::Eggs::new::<factorio::eggs::eggs>(8.0) / min;
    let machine_nut_to_egg_in =
        F::Nutrients::new::<factorio::nutrients::nutrients>(60.0 * 2.0) / min;

    let machine_egg_to_sci_out = F::Science::new::<factorio::science::science>(23.0) / min;
    let machine_egg_to_sci_in = F::Eggs::new::<factorio::eggs::eggs>(15.0) / min;

    let machine_canning_out = F::Fluid::new::<factorio::fluid::fluid>(60.0 * 25.0) / min;
    let machine_canning_in = F::Nutrients::new::<factorio::nutrients::nutrients>(60.0) / min;

    // --- STEP 2: Calculate Requirements ---
    let target_sci_rate = F::Science::new::<factorio::science::science>(180.0) / min;
    let target_fluid_rate = F::Fluid::new::<factorio::fluid::fluid>(180.0) / min;

    let sci_machines = target_sci_rate / machine_egg_to_sci_out;
    let eggs_needed = sci_machines * machine_egg_to_sci_in;
    let egg_machines = eggs_needed / machine_nut_to_egg_out;
    let nutrients_for_eggs = egg_machines * machine_nut_to_egg_in;
    let canning_machines = target_fluid_rate / machine_canning_out;
    let nutrients_for_fluid = canning_machines * machine_canning_in;
    let total_nutrients = nutrients_for_eggs + nutrients_for_fluid;
    let bio_to_nut_machines = total_nutrients / machine_bio_to_nut_out;
    let total_bioflux_needed = bio_to_nut_machines * machine_bio_to_nut_in;

    println!("--- Production Report ---");
    println!("> Science Stations:    {:.2}", sci_machines.value);
    println!("> Egg Creation Labs:   {:.2}", egg_machines.value);
    println!("> Canning Plants:      {:.2}", canning_machines.value);
    println!("> Bioflux Processors:  {:.2}", bio_to_nut_machines.value);
    println!(
        "Total Bioflux/min:     {:.2}",
        (total_bioflux_needed * min).value
    );
}
