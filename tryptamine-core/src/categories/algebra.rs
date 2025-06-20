use konst::for_range;
type Vuint = u128;
const fn modpow(base: Vuint, exp: Vuint, modulus: Vuint) -> Vuint {
    let mut collecter = 1;
    let mut squarer = base;
    let bits = 8 * size_of::<Vuint>() as u32;
    for_range!(i in 0..bits-exp.leading_zeros() => {
        if (exp >> i & 1) == 1 {
            collecter = (squarer * collecter) % modulus
        }
        squarer = (squarer * squarer) % modulus;
    }
    );
    return collecter;
}

const _: () = assert!(modpow(3, 2, 9) == 0);
const _: () = assert!(modpow(3, 0, 9) == 1);
const _: () = assert!(modpow(3, 43, 43) == 3);
const _: () = assert!(modpow(3, 7, 7) == 3);

const fn const_is_prime(p: Vuint) -> bool {
    if p == 2 {
        return true;
    }
    if p == 3 {
        return true;
    }
    const fn miller_rabin_iteration(
        witness: Vuint,
        z: Vuint,
        z_odd: Vuint,
        z_even_exp: Vuint,
        maybe_prime: Vuint,
    ) -> bool {
        // Choose a witness between 2 and n-2
        let mut x = modpow(witness, z_odd, maybe_prime);
        if x == 1 {
            return true;
        }
        for_range!( _ in 0..z_even_exp => {
            if x == z {
                return true;
            }
            x = x.pow(2) % maybe_prime;
            if x == 1 {
                return false;
            }
        });
        false
    }
    // let witnesses = [2, 3];
    // let witnesses = [2, 7, 61];
    let witnesses = [2, 3];
    let z = p - 1;
    let z_even_exp = z.trailing_zeros() as Vuint;
    let z_odd = z >> z_even_exp;
    for_range!( i in 0..witnesses.len()=> {
        if !miller_rabin_iteration(witnesses[i],z,z_odd,z_even_exp,p) {
            return false;
        };
    });
    true
}

// const _: () = assert!(const_is_prime(2), "2 is prime");
// const _: () = assert!(const_is_prime(3), "3 is prime");
// const _: () = assert!(!const_is_prime(4), "4 isn't prime");
// const _: () = assert!(const_is_prime(5), "5 is prime");
// const _: () = assert!(!const_is_prime(6), "6 isn't prime");
// const _: () = assert!(const_is_prime(7), "7 is prime");
// const _: () = assert!(!const_is_prime(8), "8 isn't prime");
// const _: () = assert!(!const_is_prime(9), "9 isn't prime");
// const _: () = assert!(!const_is_prime(10), "10 isn't prime");
// const _: () = assert!(const_is_prime(11), "11 is prime");
// const _: () = assert!(!const_is_prime(12), "12 isn't prime");
// const _: () = assert!(const_is_prime(13), "13 is prime");
// const _: () = assert!(!const_is_prime(14), "14 isn't prime");
// const _: () = assert!(!const_is_prime(15), "15 isn't prime");
// const _: () = assert!(!const_is_prime(16), "16 isn't prime");
// const _: () = assert!(const_is_prime(17), "17 is prime");
// const _: () = assert!(!const_is_prime(18), "18 isn't prime");
// const _: () = assert!(const_is_prime(19), "19 is prime");
// const _: () = assert!(!const_is_prime(20), "20 isn't prime");
// const _: () = assert!(!const_is_prime(21), "21 isn't prime");
// const _: () = assert!(!const_is_prime(22), "22 isn't prime");
// const _: () = assert!(const_is_prime(23), "23 is prime");
// const _: () = assert!(!const_is_prime(24), "24 isn't prime");
// const _: () = assert!(!const_is_prime(25), "25 isn't prime");
// const _: () = assert!(!const_is_prime(26), "26 isn't prime");
// const _: () = assert!(!const_is_prime(27), "27 isn't prime");
// const _: () = assert!(!const_is_prime(28), "28 isn't prime");
// const _: () = assert!(const_is_prime(29), "29 is prime");
// const _: () = assert!(!const_is_prime(30), "30 isn't prime");
// const _: () = assert!(const_is_prime(31), "31 is prime");
// const _: () = assert!(!const_is_prime(32), "32 isn't prime");
// const _: () = assert!(!const_is_prime(33), "33 isn't prime");
// const _: () = assert!(!const_is_prime(34), "34 isn't prime");
// const _: () = assert!(!const_is_prime(35), "35 isn't prime");
// const _: () = assert!(!const_is_prime(36), "36 isn't prime");
// const _: () = assert!(const_is_prime(37), "37 is prime");
// const _: () = assert!(!const_is_prime(38), "38 isn't prime");
// const _: () = assert!(!const_is_prime(39), "39 isn't prime");
// const _: () = assert!(!const_is_prime(40), "40 isn't prime");
// const _: () = assert!(const_is_prime(41), "41 is prime");
// const _: () = assert!(!const_is_prime(42), "42 isn't prime");
// const _: () = assert!(const_is_prime(43), "43 is prime");
// const _: () = assert!(!const_is_prime(44), "44 isn't prime");
// const _: () = assert!(!const_is_prime(45), "45 isn't prime");
// const _: () = assert!(!const_is_prime(46), "46 isn't prime");
// const _: () = assert!(const_is_prime(47), "47 is prime");
// const _: () = assert!(!const_is_prime(48), "48 isn't prime");
// const _: () = assert!(!const_is_prime(49), "49 isn't prime");
// const _: () = assert!(!const_is_prime(50), "50 isn't prime");

trait Group: PartialEq + Clone {
    fn inverse(&self) -> Self;
    fn identity() -> Self;
    fn mul(&self, other: &Self) -> Self;
}

trait Finite {
    fn listall() -> Vec<Self>
    where
        Self: Sized;
}

type DirectProduct<A: Group, B: Group> = (A, B);

impl<A: Group, B: Group> Group for DirectProduct<A, B> {
    fn identity() -> Self {
        (A::identity(), B::identity())
    }
    fn inverse(&self) -> Self {
        (self.0.inverse(), self.1.inverse())
    }
    fn mul(&self, other: &Self) -> Self {
        (self.0.mul(&other.0), self.1.mul(&other.1))
    }
}

macro_rules! define_cyclic_group {
    ($name:ident, $n:expr) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        struct $name(Vuint);

        impl Group for $name {
            fn identity() -> Self {
                $name(0)
            }

            fn inverse(&self) -> Self {
                $name(($n - self.0) % $n)
            }

            fn mul(&self, other: &Self) -> Self {
                $name((self.0 + other.0) % $n)
            }
        }

        impl Finite for $name {
            fn listall() -> Vec<Self> {
                (0..$n).map($name).collect()
            }
        }
    };
}

define_cyclic_group!(Z2, 2);

fn exp<G: Group>(base: G, exp: &Vuint) -> G {
    let mut collector = G::identity();
    let mut squarer = base;
    let bits = 8 * size_of::<Vuint>() as u32;
    for i in 0..bits - exp.leading_zeros() {
        if (exp >> i & 1) == 1 {
            collector = collector.mul(&squarer)
        }
        squarer = squarer.mul(&squarer)
    }
    return collector;
}

trait Field: PartialEq + Clone {
    fn zero() -> Self;
    fn one() -> Self;
    fn add(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn add_inv(&self) -> Self;
    fn mul_inv(&self) -> Option<Self>
    where
        Self: Sized;
    fn sub(&self, other: &Self) -> Self {
        self.add(&other.add_inv())
    }
    fn div(&self, other: &Self) -> Option<Self> {
        Some(self.mul(&other.mul_inv()?))
    }
}

macro_rules! define_prime_field {
    ($name:ident, $p:expr) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        struct $name(Vuint);

        const _: () = assert!(const_is_prime($p), "Modulus must be prime");

        impl Field for $name {
            fn zero() -> Self {
                $name(0)
            }

            fn one() -> Self {
                $name(1)
            }

            fn add(&self, other: &Self) -> Self {
                $name((self.0 + other.0) % $p)
            }

            fn mul(&self, other: &Self) -> Self {
                $name((self.0 * other.0) % $p)
            }

            fn add_inv(&self) -> Self {
                $name(($p - self.0) % $p)
            }

            fn mul_inv(&self) -> Option<Self> {
                if self.0 == 0 {
                    None
                } else {
                    // Use fermats little theorem, if a^p-1 = 1, then a^p-2=a^-1
                    // So calculate a^p-2 quickly with exp by squaring alg
                    let result = modpow(self.0, $p - 2, $p);
                    Some($name(result))
                }
            }
        }

        impl Finite for $name {
            fn listall() -> Vec<Self> {
                (0..$p).map($name).collect()
            }
        }
    };
}

define_prime_field!(F3, 3); // Creates GF(3) finite field

// Define a trait for elliptic curve parameters
trait EllipticCurveParams: Clone {
    type F: Field;
    const A: Self::F;
    const B: Self::F;
}

// Elliptic curve point struct parameterized by the curve parameters
// y^2 = x^3 + ax+b
#[derive(Debug, PartialEq, Clone)]
enum EllipticCurve<P: EllipticCurveParams> {
    Finite { x: P::F, y: P::F },
    Infinity,
}

// Given 2 curves, x and y, there is a line between them

impl<P: EllipticCurveParams> EllipticCurve<P> {
    fn valid_point(x: P::F, y: P::F) -> Option<Self> {
        let is_on_curve = y.mul(&y) == x.mul(&x.mul(&x)).add(&x.mul(&P::A)).add(&P::B);
        match is_on_curve {
            true => return Some(Self::Finite { x, y }),
            false => return None,
        }
    }
}

// Actually you have 4 cases.
// Infinity * Infinity = Infinity
// and
// Finite * Infinity = Finite, and vice versa via the
// So you have 2 points P and S, consisting of px and yy, and sx and sy.
//
// Then the slope of the line connecting the two points is going to be
//  S  = px - sx / py-sy
//  if the mul inverse of py-sy then the two points must have the same x value, meaning you should
//  return the point at infinity.
//  so the line between the 2 points is y = S*x+d (d the y intercept shouldnt matter)
//
//  So we want to find solutions in x by solving
//  (S*x+d)^2 = x^3 + A*x+B
//
//  S^2 * x^2 + 2*S*x*d+d^2 + d^2 - x^3 - A*x - B = 0
//
//  However, this is a third degree polynomial
//
fn mul_by_int<F: Field>(x: &F, mulint: u64) -> F {
    if mulint == 0 {
        return F::one();
    }
    let mut collector = x.clone();
    for _ in 1..mulint - 1 {
        collector = collector.add(&x);
    }
    return collector;
}

impl<P: EllipticCurveParams + PartialEq> Group for EllipticCurve<P> {
    fn mul(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Infinity, _) => other.clone(),
            (_, Self::Infinity) => self.clone(),
            (Self::Finite { x: px, y: py }, Self::Finite { x: sx, y: sy }) => {
                if px == sx {
                    if py != sy {
                        return Self::Infinity;
                    };
                    let Some(s) = mul_by_int(&px.mul(px), 3).div(&mul_by_int(py, 2)) else {
                        return Self::Infinity;
                    };
                    return Self::Finite { x: s.clone(), y: s };
                };
                let Some(slope_denom) = py.sub(sy).mul_inv() else {
                    return Self::Infinity;
                };
                // Check if yp isnt zero
                let slope = (px.sub(sx)).mul(&slope_denom);
                let rx = slope.mul(&slope).sub(&px.add(sx));
                let ry = py.add(&slope.mul(&px.sub(py)));
                Self::Finite { x: rx, y: ry }
            }
        }
    }
    fn identity() -> Self {
        EllipticCurve::Infinity
    }
    // Apparently the inverse of a thing is just the point flipped across the x axis
    fn inverse(&self) -> Self {
        match self {
            Self::Infinity => return Self::Infinity,
            Self::Finite { x, y } => {
                return Self::Finite {
                    x: x.clone(),
                    y: y.add_inv(),
                };
            }
        }
    }
}
