// Define our custom Factorio quantity system.
// Dimensions (in order): Time, Bioflux, Nutrients, Eggs, Science, Fluid
system! {
    quantities: FQ {
        time:      second,    T;
        bioflux:   bioflux,   B;
        nutrients: nutrients, N;
        eggs:      eggs,      E;
        science:   science,   S;
        fluid:     fluid,     F;
    }
    units: FU {
        mod time::Time,
        mod bioflux::Bioflux,
        mod nutrients::Nutrients,
        mod eggs::Eggs,
        mod science::Science,
        mod fluid::Fluid,
    }
}

pub mod time;
pub mod bioflux;
pub mod nutrients;
pub mod eggs;
pub mod science;
pub mod fluid;

// Generate public `f64` (and `f32`) modules with type aliases for each quantity,
// mirroring how uom's SI does it with `storage_types! { pub types: All; ISQ!(...) }`.
storage_types! {
    pub types: All;
    FQ!(crate::factorio, V);
}
