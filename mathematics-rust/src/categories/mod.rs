mod algebra;
mod experiments;

pub trait CandidateProduct<A, B> {
    fn fst(&self) -> &A;
    fn snd(&self) -> &B;
}

pub trait Product<A, B> {
    fn from_candidate(candidate: impl CandidateProduct<A, B>) -> Self;
}

impl<A, B> CandidateProduct<A, B> for (A, B) {
    fn fst(&self) -> &A {
        &self.0
    }
    fn snd(&self) -> &B {
        &self.1
    }
}

impl<A: Clone, B: Clone> Product<A, B> for (A, B) {
    fn from_candidate(candidate: impl CandidateProduct<A, B>) -> Self {
        (candidate.fst().clone(), candidate.snd().clone())
    }
}

pub trait CandidateSum<A, B> {
    fn left(left: A) -> Self;
    fn right(right: B) -> Self;
}

pub trait Sum<A, B> {
    fn from_candidate<S: CandidateSum<A, B>>(sum: Self) -> S;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}

impl<A, B> CandidateSum<A, B> for Either<A, B> {
    fn left(left: A) -> Self {
        Either::Left(left)
    }
    fn right(right: B) -> Self {
        Either::Right(right)
    }
}

impl<A, B> Sum<A, B> for Either<A, B> {
    fn from_candidate<S: CandidateSum<A, B>>(sum: Self) -> S {
        match sum {
            Either::Left(a) => S::left(a),
            Either::Right(b) => S::right(b),
        }
    }
}
