use typenum::{P1, Z0};

quantity! {
    quantity: Bioflux; "bioflux";
    dimension: FQ<Z0, P1, Z0, Z0, Z0, Z0>;
    units {
        @bioflux: 1.0; "bf", "bioflux", "bioflux";
    }
}
