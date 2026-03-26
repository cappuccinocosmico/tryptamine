use typenum::{P1, Z0};

quantity! {
    quantity: Nutrients; "nutrients";
    dimension: FQ<Z0, Z0, P1, Z0, Z0, Z0>;
    units {
        @nutrients: 1.0; "nt", "nutrients", "nutrients";
    }
}
