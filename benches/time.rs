use criterion::{black_box, BenchmarkId, Criterion};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use sux::{
    bits::BitVec,
    rank_sel::{SelectFixed2, SimpleSelect},
    traits::Select,
};
use valcurt::utils::{DENSITIES, LENS};

const LOG2_ONES_PER_INVENTORY: usize = 10;
const LOG2_U64_PER_SUBINVENTORY: usize = 3;

pub fn compare_simple_fixed(c: &mut Criterion) {
    let mut group = c.benchmark_group(format!(
        "select_fixed2_{}_{}",
        LOG2_ONES_PER_INVENTORY, LOG2_U64_PER_SUBINVENTORY,
    ));

    let mut bitvecs = Vec::<BitVec>::new();
    let mut bitvec_ids = Vec::<(u64, f64)>::new();
    let mut rng = SmallRng::seed_from_u64(0);
    for len in LENS {
        for density in DENSITIES {
            let bitvec = (0..len).map(|_| rng.gen_bool(density)).collect::<BitVec>();
            bitvecs.push(bitvec);
            bitvec_ids.push((len, density));
        }
    }

    let mut rng = SmallRng::seed_from_u64(0);
    for (bitvec, bitvec_id) in std::iter::zip(&bitvecs, &bitvec_ids) {
        let bits = bitvec.clone();
        let num_ones = bits.count_ones();
        let sel: SelectFixed2<
            BitVec,
            Vec<u64>,
            LOG2_ONES_PER_INVENTORY,
            LOG2_U64_PER_SUBINVENTORY,
        > = SelectFixed2::new(bits);
        group.bench_function(
            BenchmarkId::from_parameter(format!("{}_{}_0", bitvec_id.0, bitvec_id.1)),
            |b| {
                b.iter(|| {
                    // use fastrange
                    let r =
                        ((rng.gen::<u64>() as u128).wrapping_mul(num_ones as u128) >> 64) as usize;
                    black_box(unsafe { sel.select_unchecked(r) });
                })
            },
        );
    }
    group.finish();

    let mut rng = SmallRng::seed_from_u64(0);
    let mut group = c.benchmark_group(format!(
        "simple_select_{}_{}",
        LOG2_ONES_PER_INVENTORY, LOG2_U64_PER_SUBINVENTORY
    ));
    for (bitvec, bitvec_id) in std::iter::zip(&bitvecs, &bitvec_ids) {
        let bits = bitvec.clone();
        let num_ones = bits.count_ones();
        let sel: SimpleSelect = SimpleSelect::with_inv(
            bits,
            num_ones,
            LOG2_ONES_PER_INVENTORY,
            LOG2_U64_PER_SUBINVENTORY,
        );
        group.bench_function(
            BenchmarkId::from_parameter(format!("{}_{}_0", bitvec_id.0, bitvec_id.1)),
            |b| {
                b.iter(|| {
                    // use fastrange
                    let r =
                        ((rng.gen::<u64>() as u128).wrapping_mul(num_ones as u128) >> 64) as usize;
                    black_box(unsafe { sel.select_unchecked(r) });
                })
            },
        );
    }
    group.finish();
}

fn main() {
    let mut criterion = Criterion::default().configure_from_args();
    compare_simple_fixed(&mut criterion);
    criterion.final_summary();
}
