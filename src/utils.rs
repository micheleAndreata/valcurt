use rand::{rngs::SmallRng, Rng};
use std::any::type_name;
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
