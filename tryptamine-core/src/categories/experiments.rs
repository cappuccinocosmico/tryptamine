fn every_applicable_direction<A, B, C>(x: Result<(A, B), (A, C)>) -> (A, Result<B, C>) {
    match x {
        Ok((a, b)) => (a, Result::Ok(b)),
        Err((a, c)) => (a, Result::Err(c)),
    }
}

fn other_direction_needing_exponentials<A, B, C>(x: (A, Result<B, C>)) -> Result<(A, B), (A, C)> {
    let (a, b_or_c) = x;
    match b_or_c {
        Ok(b) => Result::Ok((a, b)),
        Err(c) => Result::Err((a, c)),
    }
}
