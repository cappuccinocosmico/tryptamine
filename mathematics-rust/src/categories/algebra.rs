trait Group {
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

// trait NormalSubgroup<G: Group> {
//     fn normalize(element: G) -> G;
// }
// #[derive(PartialEq, Eq, Clone, Debug)]
// struct QuotientGroup<G: Group, N: NormalSubgroup<G>>(G);
//
// impl<G: Group, N: NormalSubgroup<G>> Group for QuotientGroup<G, N> {
//     fn identity() -> Self {
//         QuotientGroup(N::normalize(G::identity()))
//     }
//
//     fn inverse(&self) -> Self {
//         QuotientGroup(N::normalize(self.0.inverse()))
//     }
//
//     fn mul(&self, other: &Self) -> Self {
//         QuotientGroup(N::normalize(self.0.mul(&other.0)))
//     }
// }
//
// impl<G: Finite + Group, N: NormalSubgroup<G>> Finite for QuotientGroup<G, N> {
//     fn listall() -> Vec<Self> {
//         let mut seen = Vec::new();
//         let mut result = Vec::new();
//         for elem in G::listall() {
//             let repr = N::normalize(elem);
//             let q = QuotientGroup(repr);
//             if !seen.contains(&q) {
//                 seen.push(q.clone());
//                 result.push(q);
//             }
//         }
//         result
//     }
// }
//
// // macro_rules!! {
// //     () => {
// //
// //     };
// // }
