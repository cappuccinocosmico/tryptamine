use const_for::const_for;

trait Group: PartialEq {
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
        struct $name(u32);

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

const fn is_prime(x: u32) -> bool {
    if (x % 2) == 0 {
        if x == 2 {
            return true;
        } else {
            return false;
        }
    }
    const_for!( i in 1..x >> 2 => {
        if x % (2 * i + 1) == 0 {
            return false;
        }
    });
    true
}

trait Field: PartialEq {
    fn zero() -> Self;
    fn one() -> Self;
    fn add(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn add_inv(&self) -> Self;
    fn mul_inv(&self) -> Option<Self>
    where
        Self: Sized;
}

macro_rules! define_prime_field {
    ($name:ident, $p:expr) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        struct $name(u32);

        const _: () = assert!(is_prime($p), "Modulus must be prime");

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
                    // Using Fermat's little theorem for prime fields
                    let mut exponent = $p - 2;
                    let mut result = 1;
                    let mut base = self.0;
                    while exponent > 0 {
                        if exponent % 2 == 1 {
                            result = (result * base) % $p;
                        }
                        base = (base * base) % $p;
                        exponent /= 2;
                    }
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
