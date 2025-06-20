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
enum Void {}

fn explosion<T>(v: Void) -> T {
    let test_result: Result<T, Void> = Err(v);
    let Ok(val) = test_result;
    val
}

static STATIC_EVIDENCE: &'static &'static () = &&();

fn extend_lifetime<'a, 'b, T>(val: &'b T) -> &'a T {
    fn translate_lifetime<'a, 'b, T>(_: &'a &'b (), val: &'b T) -> &'a T {
        val
    }
    let test_lambda: for<'x> fn(_, &'x T) -> &'a T = translate_lifetime;
    test_lambda(STATIC_EVIDENCE, val)
}

fn extend_lifetime_mut<'a, 'b, T>(val: &'b mut T) -> &'a mut T {
    fn translate_lifetime<'a, 'b, T>(_: &'a &'b (), val: &'b mut T) -> &'a mut T {
        val
    }
    let test_lambda: for<'x> fn(_, &'x mut T) -> &'a mut T = translate_lifetime;
    test_lambda(STATIC_EVIDENCE, val)
}

fn make_void_without_panic() -> Void {
    enum Test {
        Enhabited(Option<u8>),
        Void(Option<(Void, u8)>),
    }
    let mut test_val = Test::Void(None);
    let test = &mut test_val;
    let Test::Void(ref_to_empty) = test else {
        unreachable!("The value was declared as a void case when initialized")
    };
    println!("Successfully completed all declarations.");
    let ref_to_empty_extended = extend_lifetime_mut(ref_to_empty);
    println!("illegally extended reference");
    *test = Test::Enhabited(Some(0));
    println!("set value to enhabited value");
    let optional_empty = ref_to_empty_extended.take();
    println!("got optional void value");
    let empty = optional_empty.expect("This should always succeed since we illegally set the value of the enum as being enhabited.");
    println!("got void value tuple");
    empty.0
}

// fn main() {
//     let void = make_void_without_panic();
//     println!("Successfully made void!")
// }
