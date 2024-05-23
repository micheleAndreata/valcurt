use criterion::Criterion;
use valcurt::utils::{bench_rank, save_mem_cost, DENSITIES, LENS, REPS};

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
