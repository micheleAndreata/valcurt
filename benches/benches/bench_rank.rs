use criterion::{black_box, measurement::Measurement, BenchmarkGroup, BenchmarkId, Criterion};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use sux::bits::BitVec;
use valcurt::utils::{fastrange, save_mem_cost, BenchRank};

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

fn bench_rank<R: BenchRank, M: Measurement>(
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

pub fn bench_rank9(c: &mut Criterion) {
    let name = "rank9";
    let mut group = c.benchmark_group(name);
    bench_rank::<sux::rank_sel::Rank9, _>(&mut group, &LENS, &DENSITIES, REPS);
    save_mem_cost::<sux::rank_sel::Rank9>(name, &LENS, &[0.5], true);
    group.finish();
}

pub fn bench_rank10<const LOG2_LOWER_BLOCK_SIZE: usize>(c: &mut Criterion) {
    let name = format!("rank10_{}", LOG2_LOWER_BLOCK_SIZE);
    let mut group = c.benchmark_group(&name);
    bench_rank::<sux::rank_sel::Rank10<LOG2_LOWER_BLOCK_SIZE>, _>(
        &mut group, &LENS, &DENSITIES, REPS,
    );
    save_mem_cost::<sux::rank_sel::Rank10<LOG2_LOWER_BLOCK_SIZE>>(&name, &LENS, &[0.5], true);
    group.finish();
}

pub fn bench_rank11(c: &mut Criterion) {
    let name = "rank11";
    let mut group = c.benchmark_group(name);
    bench_rank::<sux::rank_sel::Rank11, _>(&mut group, &LENS, &DENSITIES, REPS);
    save_mem_cost::<sux::rank_sel::Rank11>(name, &LENS, &[0.5], true);
    group.finish();
}

pub fn bench_rank12(c: &mut Criterion) {
    let name = "rank12";
    let mut group = c.benchmark_group(name);
    bench_rank::<sux::rank_sel::Rank12, _>(&mut group, &LENS, &DENSITIES, REPS);
    save_mem_cost::<sux::rank_sel::Rank12>(name, &LENS, &[0.5], true);
    group.finish();
}

pub fn bench_rank16(c: &mut Criterion) {
    let name = "rank16";
    let mut group = c.benchmark_group(name);
    bench_rank::<sux::rank_sel::Rank16, _>(&mut group, &LENS, &DENSITIES, REPS);
    save_mem_cost::<sux::rank_sel::Rank16>(name, &LENS, &[0.5], true);
    group.finish();
}

pub fn bench_poppy(c: &mut Criterion) {
    let name = "poppy";
    let mut group = c.benchmark_group(name);
    bench_rank::<bitm::ArrayWithRank101111, _>(&mut group, &LENS, &DENSITIES, REPS);
    save_mem_cost::<bitm::ArrayWithRank101111>(name, &LENS, &[0.5], true);
    group.finish();
}
