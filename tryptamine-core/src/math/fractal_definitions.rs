use num_complex::Complex;
use smallvec::{SmallVec, smallvec};
use tracing::debug;

pub type RealType = f64;
pub type Compl = Complex<RealType>;
#[derive(Clone, Copy, Debug)]
pub struct FiniteFatouBasin {
    pub basin: Compl,
    pub neighborhood_sqr: RealType,
}

const SMALL_SIZE: usize = 5;
#[derive(Clone, Debug)]
pub struct FatouBasins {
    pub infinte_basin_radius_sqr: Option<RealType>,
    pub finite_basins: SmallVec<[FiniteFatouBasin; SMALL_SIZE]>,
}

const DEFAULT_MAX_ITERATIONS: u32 = 300;
pub trait ComplexFatouFractal: Copy + Sync {
    fn generate_fatou_basins(&self) -> FatouBasins;
    fn iterate_mut(&self, collector: &mut Compl, original: &Compl);
    fn get_iterations(&self) -> u32;
}

#[derive(Debug, Clone, Copy)]
pub struct RegularJuliaSet {
    pub c: Compl,
    pub iterations: u32,
}

impl Default for RegularJuliaSet {
    fn default() -> Self {
        Self {
            // c: Complex::new(0.2, 0.3),
            // c: Complex::new(-0.8, 0.155),
            c: Complex::new(-1.0, 0.0),
            iterations: DEFAULT_MAX_ITERATIONS,
        }
    }
}
impl ComplexFatouFractal for RegularJuliaSet {
    fn iterate_mut(&self, collector: &mut Compl, _: &Compl) {
        *collector = *collector * *collector + self.c
    }

    fn generate_fatou_basins(&self) -> FatouBasins {
        let mut finite_basins = smallvec![];
        // x+ = x^2 + c
        // 0 = x^2 -x + c
        // 0 = (x^2-x+1/4)-1/4+c
        // 0 = (x-1/2)^2-1/4+c
        // 1/4 -c = (x-1/2)^2
        // \pm sqrt(1/4-c) = x-1/2
        // x = 1/2 \pm sqrt(1/4 - c)
        let val1 = 1.0 / 2.0 + (1.0 / 4.0 - self.c).sqrt();
        let val2 = 1.0 / 2.0 - (1.0 / 4.0 - self.c).sqrt();
        fn validate_basin(
            val: Complex<RealType>,
            basins: &mut SmallVec<[FiniteFatouBasin; SMALL_SIZE]>,
        ) {
            debug!("validating basin: {}", val);
            let valprime = 2.0 * val;
            debug!(
                "basin has derivative: {}, with norm: {}",
                valprime,
                valprime.norm()
            );
            let is_valid = valprime.norm() <= 1.0;
            if is_valid {
                basins.push(FiniteFatouBasin {
                    basin: val,
                    neighborhood_sqr: 0.001,
                });
            }
        }
        validate_basin(val1, &mut finite_basins);
        validate_basin(val2, &mut finite_basins);
        FatouBasins {
            infinte_basin_radius_sqr: Some(4.0),
            finite_basins,
        }
    }
    fn get_iterations(&self) -> u32 {
        self.iterations
    }
}
#[derive(Debug, Clone, Copy)]
pub struct SinJuliaSet {
    pub c: Compl,
    pub iterations: u32,
}

impl Default for SinJuliaSet {
    fn default() -> Self {
        Self {
            c: Complex::new(-0.7, 0.1),
            iterations: DEFAULT_MAX_ITERATIONS,
        }
    }
}
impl ComplexFatouFractal for SinJuliaSet {
    fn iterate_mut(&self, collector: &mut Compl, _: &Compl) {
        *collector = (*collector).sin() * self.c
    }

    fn generate_fatou_basins(&self) -> FatouBasins {
        FatouBasins {
            infinte_basin_radius_sqr: Some(64.0),
            // TODO: Write code to generate interior basins
            finite_basins: smallvec![],
        }
    }
    fn get_iterations(&self) -> u32 {
        self.iterations
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MandelbrotSet {
    pub iterations: u32,
}
impl Default for MandelbrotSet {
    fn default() -> Self {
        MandelbrotSet {
            iterations: DEFAULT_MAX_ITERATIONS,
        }
    }
}

impl ComplexFatouFractal for MandelbrotSet {
    fn iterate_mut(&self, collector: &mut Compl, original: &Compl) {
        *collector = *collector * *collector + original;
    }
    fn generate_fatou_basins(&self) -> FatouBasins {
        FatouBasins {
            infinte_basin_radius_sqr: Some(8.0),
            // TODO: Write code to generate interior basins
            finite_basins: smallvec![],
        }
    }
    fn get_iterations(&self) -> u32 {
        self.iterations
    }
}
