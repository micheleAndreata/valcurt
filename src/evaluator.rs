use rand::{rngs::SmallRng, Rng};
use std::io::Write;
use std::{
    fs::{create_dir_all, File},
    hint::black_box,
};
use sux::bits::BitVec;

fn fastrange(rng: &mut SmallRng, range: u64) -> u64 {
    ((rng.gen::<u64>() as u128).wrapping_mul(range as u128) >> 64) as u64
}

pub trait Evaluate {
    /// Create a new instance of the struct that is being evaluated.
    /// The struct should be initialized with the given data and length.
    /// The length is the number of bits in the bit vector.
    /// The data is a vector of usize values that represent the bits in the bit vector.
    fn new(data: Vec<usize>, len: usize) -> Self;
    /// Call the function that is being benchmarked.
    fn benched_fn(&self, input: usize) -> usize;
    /// Return the length in bits of the underlying bit vector.
    fn len(&self) -> usize;
    /// Return the memory size of the struct in bytes.
    fn mem_size(&self) -> usize;
}

pub struct Evaluator<X>
where
    X: Evaluate,
{
    rng: SmallRng,
    phantom_x: std::marker::PhantomData<X>,
}

impl<X> Evaluator<X>
where
    X: Evaluate,
{
    pub fn new(rng: SmallRng) -> Self {
        Self {
            rng,
            phantom_x: std::marker::PhantomData,
        }
    }

    /// Benchmark the struct with the given parameters.
    /// The benchmark will be run for each combination of lens and densities.
    pub fn bench(
        &mut self,
        bench_name: &str,
        lens: &[u64],
        densities: &[f64],
        uniform: bool,
        repetitions: usize,
        iterations: usize,
    ) {
        create_dir_all("target/results").unwrap();
        let mut file = File::create(format!("target/results/{}.csv", bench_name)).unwrap();
        let iter: Vec<_> = lens
            .into_iter()
            .copied()
            .flat_map(|x| densities.into_iter().copied().clone().map(move |y| (x, y)))
            .collect();
        for (len, density) in tqdm::tqdm(iter) {
            let (mean, median) = self.bench_single(len, density, uniform, repetitions, iterations);
            let mem_cost = {
                let (_, _, data) = self.create_bitvec(len, density, uniform);
                let val_struct = X::new(data, len as usize);
                self.mem_cost(&val_struct)
            };
            writeln!(
                file,
                "{}, {}, {}, {}, {}",
                len, density, mean, median, mem_cost
            )
            .unwrap();
        }
    }

    pub fn mem_cost(&self, val_struct: &X) -> f64 {
        (((val_struct.mem_size() * 8 - val_struct.len()) * 100) as f64) / (val_struct.len() as f64)
    }

    pub fn bench_single(
        &mut self,
        len: u64,
        density: f64,
        uniform: bool,
        repetitions: usize,
        iterations: usize,
    ) -> (f64, f64) {
        let mut times = vec![0; repetitions];
        for i in 0..repetitions {
            let (num_ones_first_half, num_ones_second_half, data) =
                self.create_bitvec(len, density, uniform);
            let val_struct = X::new(data, len as usize);

            let mut u = 0;
            let begin = std::time::Instant::now();
            for _ in 0..iterations {
                u ^= if u & 1 != 0 {
                    val_struct.benched_fn(
                        (num_ones_first_half + fastrange(&mut self.rng, num_ones_second_half))
                            as usize,
                    )
                } else {
                    val_struct.benched_fn(fastrange(&mut self.rng, num_ones_first_half) as usize)
                };
            }
            let elapsed = begin.elapsed().as_nanos();
            black_box(u);

            times[i] = elapsed as u64;
        }
        times.sort();
        let mean = (times.iter().copied().reduce(|acc, e| acc + e).unwrap() as f64)
            / (iterations * repetitions) as f64;
        let median = if repetitions % 2 == 0 {
            (times[times.len() / 2] + times[times.len() / 2 - 1]) as f64 / (2 * iterations) as f64
        } else {
            times[times.len() / 2] as f64 / iterations as f64
        };
        (mean, median)
    }

    pub fn create_bitvec(
        &mut self,
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
                .map(|_| self.rng.gen_bool(density0))
                .collect::<BitVec>();
            if b.count_ones() > 0 {
                break b;
            }
        };
        let second_half = (0..len2)
            .map(|_| self.rng.gen_bool(density1))
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

    pub fn validate_select(&mut self) {
        print!("Validating select function... ");
        std::io::stdout().flush().unwrap();
        let lens = [1 << 18, 1 << 19, 1 << 20, 1 << 25];
        for len in lens {
            for num_ones in [1, 2, 4, 8, 16, 32, 64, 128, 256] {
                let bits = (0..len)
                    .map(|i| i % (len / num_ones) == 0)
                    .collect::<BitVec>();

                let data = bits.into_raw_parts().0;

                let val_struct = X::new(data, len);
                assert_eq!(val_struct.len(), len);
                for i in 0..num_ones {
                    assert_eq!(val_struct.benched_fn(i), i * (len / num_ones));
                }
            }
        }

        for len in lens {
            let (num_ones_first_half, num_ones_second_half, data) =
                self.create_bitvec(len as u64, 0.5, false);

            let bits = unsafe { BitVec::from_raw_parts(data.clone(), len) };

            let ones = (num_ones_first_half + num_ones_second_half) as usize;
            let mut pos = Vec::with_capacity(ones);
            for i in 0..(len as usize) {
                if bits[i] {
                    pos.push(i);
                }
            }

            let val_struct = X::new(data, len);

            for i in 0..(ones) {
                assert!(val_struct.benched_fn(i) == pos[i]);
            }
        }
        println!("done");
    }
}
