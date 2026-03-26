use typenum::{P1, Z0};

quantity! {
    quantity: Time; "time";
    dimension: FQ<P1, Z0, Z0, Z0, Z0, Z0>;
    units {
        @second: 1.0;   "s",   "second",  "seconds";
        @minute: 60.0;  "min", "minute",  "minutes";
    }
}
