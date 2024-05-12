use criterion::{black_box, measurement::Measurement, BenchmarkGroup, BenchmarkId, Criterion};
use rand::{rngs::SmallRng, SeedableRng};
use valcurt::utils::{create_bitvec, fastrange_non_uniform, BenchSelect};

const LENS: [u64; 11] = [
    1u64 << 20,
    1 << 21,
    1 << 22,
    1 << 23,
    1 << 24,
    1 << 25,
    1 << 26,
    1 << 27,
    1 << 28,
    1 << 29,
    1 << 30,
];

const DENSITIES: [f64; 3] = [0.25, 0.5, 0.75];

const REPS: usize = 5;

fn bench_select<S: BenchSelect, M: Measurement>(
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

pub fn bench_simple_select(c: &mut Criterion, uniform: bool) {
    let mut group = c.benchmark_group("simple_select");
    bench_select::<sux::rank_sel::SimpleSelect, _>(&mut group, &LENS, &DENSITIES, REPS, uniform);
    group.finish();
}

pub fn bench_rank9sel(c: &mut Criterion, uniform: bool) {
    let mut group = c.benchmark_group("rank9sel");
    bench_select::<sux::rank_sel::Rank9Sel, _>(&mut group, &LENS, &DENSITIES, REPS, uniform);
    group.finish();
}

pub fn bench_rank10sel<const UPPER_BLOCK_SIZE: usize, const LOG2_ONES_PER_INVENTORY: usize>(
    c: &mut Criterion,
    uniform: bool,
) {
    let mut group = c.benchmark_group("rank10sel");
    bench_select::<sux::rank_sel::Rank10Sel<UPPER_BLOCK_SIZE, LOG2_ONES_PER_INVENTORY>, _>(
        &mut group, &LENS, &DENSITIES, REPS, uniform,
    );
    group.finish();
}

pub fn bench_cs_poppy(c: &mut Criterion, uniform: bool) {
    let mut group = c.benchmark_group("cs_poppy");
    bench_select::<bitm::RankSelect101111<_, _, _>, _>(
        &mut group, &LENS, &DENSITIES, REPS, uniform,
    );
    group.finish();
}
