use typenum::{P1, Z0};

quantity! {
    quantity: Fluid; "fluid";
    dimension: FQ<Z0, Z0, Z0, Z0, Z0, P1>;
    units {
        @fluid: 1.0; "fl", "fluid", "fluid";
    }
}
