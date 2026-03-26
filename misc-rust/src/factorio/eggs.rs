use typenum::{P1, Z0};

quantity! {
    quantity: Eggs; "eggs";
    dimension: FQ<Z0, Z0, Z0, P1, Z0, Z0>;
    units {
        @eggs: 1.0; "eg", "eggs", "eggs";
    }
}
