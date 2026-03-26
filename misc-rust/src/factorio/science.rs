use typenum::{P1, Z0};

quantity! {
    quantity: Science; "science";
    dimension: FQ<Z0, Z0, Z0, Z0, P1, Z0>;
    units {
        @science: 1.0; "sc", "science", "science";
    }
}
