use criterion::measurement::Measurement;
use criterion::{black_box, BenchmarkGroup, BenchmarkId};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::io::Write;
use std::{any::type_name, fs::create_dir_all};
use sux::bits::BitVec;

pub trait Measure {
    /// Create a new instance of the struct that is being measured.
    /// The struct should be initialized with the given data and length.
    /// The length is the number of bits in the bit vector.
    /// The data is a vector of usize values that represent the bits in the bit vector.
    fn new(data: Vec<usize>, len: usize) -> Self;
    /// Return the length in bits of the underlying bit vector.
    fn len(&self) -> usize;
    /// Return the memory size of the struct in bytes.
    fn mem_size(&self) -> usize;
}

pub trait BenchRank: Measure {
    /// The rank function that is being benchmarked.
    fn bench_rank(&self, input: usize) -> usize;
}

pub trait BenchSelect: Measure {
    /// The select function that is being benchmarked.
    fn bench_select(&self, input: usize) -> usize;
}

/// Generate a random number in the range [0, range).
#[inline(always)]
pub fn fastrange(rng: &mut SmallRng, range: u64) -> u64 {
    ((rng.gen::<u64>() as u128).wrapping_mul(range as u128) >> 64) as u64
}

#[inline(always)]
pub fn fastrange_non_uniform(rng: &mut SmallRng, first_half: u64, second_half: u64) -> u64 {
    if rng.gen_bool(0.5) {
        ((rng.gen::<u64>() as u128).wrapping_mul(first_half as u128) >> 64) as u64
    } else {
        first_half + ((rng.gen::<u64>() as u128).wrapping_mul(second_half as u128) >> 64) as u64
    }
}

pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

/// Return the memory cost of the struct in percentage.
/// Depends on the length of underlying bit vector and the total memory size of the struct.
pub fn mem_cost(benched_struct: impl Measure) -> f64 {
    (((benched_struct.mem_size() * 8 - benched_struct.len()) * 100) as f64)
        / (benched_struct.len() as f64)
}

/// Save the memory cost of a struct to a CSV file.
pub fn save_mem_cost<M: Measure>(name: &str, lens: &[u64], densities: &[f64], uniform: bool) {
    create_dir_all(format!("target/criterion/{}/", name)).unwrap();
    let mut file =
        std::fs::File::create(format!("target/criterion/{}/mem_cost.csv", name)).unwrap();
    let mut rng = SmallRng::seed_from_u64(0);
    for len in lens {
        for density in densities {
            let (_, _, bits) = create_bitvec(&mut rng, *len, *density, uniform);

            let sel: M = M::new(bits, *len as usize);
            let mem_cost = mem_cost(sel);
            writeln!(file, "{},{},{}", len, density, mem_cost).unwrap();
        }
    }
}

/// Create a bit vector with a given length, density, and uniformity.
/// The density is the probability of a bit being set to 1.
/// The uniformity is a boolean that determines if the density is uniform or not.
/// If the density is not uniform, the first half of the bit vector will have a density of density * 0.01
/// and the second half will have a density of density * 0.99.
pub fn create_bitvec(
    rng: &mut SmallRng,
    len: u64,
    density: f64,
    uniform: bool,
) -> (u64, u64, Vec<usize>) {
    let (density0, density1) = if uniform {
        (density, density)
    } else {
        (density * 0.01, density * 0.99)
    };

    let len1;
    let len2;
    if len % 2 == 0 {
        len1 = len / 2;
        len2 = len / 2;
    } else {
        len1 = len / 2 + 1;
        len2 = len / 2;
    }

    let first_half = loop {
        let b = (0..len1)
            .map(|_| rng.gen_bool(density0))
            .collect::<BitVec>();
        if b.count_ones() > 0 {
            break b;
        }
    };
    let second_half = (0..len2)
        .map(|_| rng.gen_bool(density1))
        .collect::<BitVec>();
    let num_ones_second_half = second_half.count_ones() as u64;
    let num_ones_first_half = first_half.count_ones() as u64;

    let bits = first_half
        .into_iter()
        .chain(second_half.into_iter())
        .collect::<BitVec>();

    (
        num_ones_first_half,
        num_ones_second_half,
        bits.into_raw_parts().0,
    )
}

pub const LENS: [u64; 7] = [
    1_000_000,
    3_000_000,
    10_000_000,
    30_000_000,
    100_000_000,
    300_000_000,
    1_000_000_000,
];

pub const DENSITIES: [f64; 3] = [0.2, 0.5, 0.8];

pub const REPS: usize = 5;

pub fn bench_rank<R: BenchRank, M: Measurement>(
    bench_group: &mut BenchmarkGroup<'_, M>,
    lens: &[u64],
    densities: &[f64],
    reps: usize,
) {
    let mut rng = SmallRng::seed_from_u64(0);
    for len in lens.iter().copied() {
        for density in densities.iter().copied() {
            // possible repetitions
            for i in 0..reps {
                let bits: BitVec = (0..len).map(|_| rng.gen_bool(density)).collect::<BitVec>();
                let (data, len) = bits.into_raw_parts();
                let rank: R = R::new(data, len);
                bench_group.bench_function(
                    BenchmarkId::from_parameter(format!("{}_{}_{}", len, density, i)),
                    |b| {
                        b.iter(|| {
                            let p = fastrange(&mut rng, len as u64) as usize;
                            black_box(rank.bench_rank(p));
                        })
                    },
                );
            }
        }
    }
}

pub fn bench_select<S: BenchSelect, M: Measurement>(
    bench_group: &mut BenchmarkGroup<'_, M>,
    lens: &[u64],
    densities: &[f64],
    reps: usize,
    uniform: bool,
) {
    let mut rng = SmallRng::seed_from_u64(0);
    for len in lens {
        for density in densities {
            // possible repetitions
            for i in 0..reps {
                let (num_ones_first_half, num_ones_second_half, bits) =
                    create_bitvec(&mut rng, *len, *density, uniform);

                let sel: S = S::new(bits, *len as usize);
                let mut routine = || {
                    let r = fastrange_non_uniform(
                        &mut rng,
                        num_ones_first_half as u64,
                        num_ones_second_half as u64,
                    ) as usize;
                    black_box(sel.bench_select(r));
                };
                bench_group.bench_function(
                    BenchmarkId::from_parameter(format!("{}_{}_{}", *len, *density, i)),
                    |b| b.iter(|| routine()),
                );
            }
        }
    }
}
