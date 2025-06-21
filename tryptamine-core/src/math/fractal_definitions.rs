use num_complex::Complex;
use rand::{random, random_range};
use smallvec::{SmallVec, smallvec};
use tracing::debug;

pub type RealType = f64;
pub type Compl = Complex<RealType>;
#[derive(Clone, Copy, Debug)]
pub struct FiniteFatouBasin {
    pub basin: Compl,
    pub neighborhood_sqr: RealType,
}

#[derive(Clone, Debug)]
pub struct FatouBasins {
    pub infinte_basin_radius_sqr: Option<RealType>,
    pub finite_basins: Vec<FiniteFatouBasin>,
}

const DEFAULT_MAX_ITERATIONS: u32 = 300;
pub trait ComplexFatouFractal: Copy + Sync {
    fn generate_fatou_basins(&self) -> FatouBasins;
    fn iterate_mut(&self, collector: &mut Compl, original: &Compl);
    fn get_iterations(&self) -> u32;
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
fn find_basins<T: ComplexFatouFractal>(config: T) -> Vec<Compl> {
    const SEEDS: usize = 100;
    const INSIDE_RANGE: f64 = 256.0;
    let seeds: Vec<Compl> = (1..SEEDS)
        .map(|_| {
            let rand_real = random_range(-INSIDE_RANGE..INSIDE_RANGE);
            let rand_im = random_range(-INSIDE_RANGE..INSIDE_RANGE);
            Complex::new(rand_real, rand_im)
        })
        .collect();
    find_basins_from_iterated_seeds(&seeds, config)
}

fn find_basins_from_iterated_seeds<T: ComplexFatouFractal>(
    seeds: &[Compl],
    config: T,
) -> Vec<Compl> {
    const DEFAULT_COMPUTED_ITERATIONS: usize = 2000;
    const EPSILON: f64 = 0.01;
    const INFINITE_SQR_BAILOUT: f64 = 16.0;
    let mut basinvals = Vec::with_capacity(10);

    for seed in seeds {
        let mut is_own_basin: bool = true;
        let mut coll = *seed;
        for _ in 1..=DEFAULT_COMPUTED_ITERATIONS {
            config.iterate_mut(&mut coll, seed);
            for test in &basinvals {
                let subval: Complex<f64> = test - coll;
                let normsqr: f64 = subval.norm_sqr();
                is_own_basin = is_own_basin && (normsqr < EPSILON);
            }
            is_own_basin = is_own_basin && coll.norm_sqr() < INFINITE_SQR_BAILOUT;
            if is_own_basin {
                break;
            };
        }
        if is_own_basin {
            basinvals.push(coll)
        }
    }

    basinvals
}

impl ComplexFatouFractal for RegularJuliaSet {
    fn iterate_mut(&self, collector: &mut Compl, _: &Compl) {
        *collector = *collector * *collector + self.c
    }

    fn generate_fatou_basins(&self) -> FatouBasins {
        let mut finite_basins = vec![];
        // x+ = x^2 + c
        // 0 = x^2 -x + c
        // 0 = (x^2-x+1/4)-1/4+c
        // 0 = (x-1/2)^2-1/4+c
        // 1/4 -c = (x-1/2)^2
        // \pm sqrt(1/4-c) = x-1/2
        // x = 1/2 \pm sqrt(1/4 - c)
        let val1 = 1.0 / 2.0 + (1.0 / 4.0 - self.c).sqrt();
        let val2 = 1.0 / 2.0 - (1.0 / 4.0 - self.c).sqrt();
        fn validate_basin(val: Complex<RealType>, basins: &mut Vec<FiniteFatouBasin>) {
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
#[derive(Debug, Clone, Copy, PartialEq)]
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
        let raw_basins = find_basins(*self);
        FatouBasins {
            infinte_basin_radius_sqr: Some(256.0),
            finite_basins: raw_basins
                .iter()
                .map(|val| FiniteFatouBasin {
                    basin: *val,
                    neighborhood_sqr: 0.01,
                })
                .collect(),
        }
    }
    fn get_iterations(&self) -> u32 {
        self.iterations
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            finite_basins: vec![],
        }
    }
    fn get_iterations(&self) -> u32 {
        self.iterations
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FractalConfig {
    Mandelbrot(MandelbrotSet),
    Julia(RegularJuliaSet),
    SinJulia(SinJuliaSet),
}
impl Default for FractalConfig {
    fn default() -> Self {
        FractalConfig::Mandelbrot(MandelbrotSet::default())
    }
}

impl ComplexFatouFractal for FractalConfig {
    fn generate_fatou_basins(&self) -> FatouBasins {
        match self {
            FractalConfig::Mandelbrot(m) => m.generate_fatou_basins(),
            FractalConfig::Julia(j) => j.generate_fatou_basins(),
            FractalConfig::SinJulia(s) => s.generate_fatou_basins(),
        }
    }

    fn iterate_mut(&self, collector: &mut Compl, original: &Compl) {
        match self {
            FractalConfig::Mandelbrot(m) => m.iterate_mut(collector, original),
            FractalConfig::Julia(j) => j.iterate_mut(collector, original),
            FractalConfig::SinJulia(s) => s.iterate_mut(collector, original),
        }
    }

    fn get_iterations(&self) -> u32 {
        match self {
            FractalConfig::Mandelbrot(m) => m.get_iterations(),
            FractalConfig::Julia(j) => j.get_iterations(),
            FractalConfig::SinJulia(s) => s.get_iterations(),
        }
    }
}

impl FractalConfig {
    pub fn get_index(&self) -> u8 {
        match self {
            FractalConfig::Mandelbrot(_) => 0,
            FractalConfig::Julia(_) => 1,
            FractalConfig::SinJulia(_) => 2,
        }
    }
    pub fn default_from_index(index: u8) -> Option<Self> {
        match index {
            0 => Some(FractalConfig::Mandelbrot(MandelbrotSet::default())),
            1 => Some(FractalConfig::Julia(RegularJuliaSet::default())),
            2 => Some(FractalConfig::SinJulia(SinJuliaSet::default())),
            _ => None,
        }
    }
}

// type FractalConfigV2 = Box<dyn ComplexFatouFractal>;
