use bitm::CombinedSampling;
use criterion::Criterion;
use valcurt::utils::{bench_select, save_mem_cost, DENSITIES, LENS, REPS};
use valcurt::{SimpleSelect0, SimpleSelect1, SimpleSelect2, SimpleSelect3};

pub fn bench_simple_select0(c: &mut Criterion, uniform: bool) {
    let mut name = "simple_select0";
    if !uniform {
        name = "simple_select0_non_uniform";
    }
    let mut group = c.benchmark_group(name);
    bench_select::<SimpleSelect0, _>(&mut group, &LENS, &DENSITIES, REPS, uniform);
    save_mem_cost::<SimpleSelect0>(name, &LENS, &[0.5], uniform);
    group.finish();
}

pub fn bench_simple_select1(c: &mut Criterion, uniform: bool) {
    let mut name = "simple_select1";
    if !uniform {
        name = "simple_select1_non_uniform";
    }
    let mut group = c.benchmark_group(name);
    bench_select::<SimpleSelect1, _>(&mut group, &LENS, &DENSITIES, REPS, uniform);
    save_mem_cost::<SimpleSelect1>(name, &LENS, &[0.5], uniform);
    group.finish();
}

pub fn bench_simple_select2(c: &mut Criterion, uniform: bool) {
    let mut name = "simple_select2";
    if !uniform {
        name = "simple_select2_non_uniform";
    }
    let mut group = c.benchmark_group(name);
    bench_select::<SimpleSelect2, _>(&mut group, &LENS, &DENSITIES, REPS, uniform);
    save_mem_cost::<SimpleSelect2>(name, &LENS, &[0.5], uniform);
    group.finish();
}

pub fn bench_simple_select3(c: &mut Criterion, uniform: bool) {
    let mut name = "simple_select3";
    if !uniform {
        name = "simple_select3_non_uniform";
    }
    let mut group = c.benchmark_group(name);
    bench_select::<SimpleSelect3, _>(&mut group, &LENS, &DENSITIES, REPS, uniform);
    save_mem_cost::<SimpleSelect3>(name, &LENS, &[0.5], uniform);
    group.finish();
}

pub fn bench_rank9sel(c: &mut Criterion, uniform: bool) {
    let mut name = "rank9sel";
    if !uniform {
        name = "rank9sel_non_uniform";
    }
    let mut group = c.benchmark_group(name);
    bench_select::<sux::rank_sel::Rank9Sel, _>(&mut group, &LENS, &DENSITIES, REPS, uniform);
    save_mem_cost::<sux::rank_sel::Rank9Sel>(name, &LENS, &[0.5], uniform);
    group.finish();
}

pub fn bench_rank10sel<const UPPER_BLOCK_SIZE: usize, const LOG2_ONES_PER_INVENTORY: usize>(
    c: &mut Criterion,
    uniform: bool,
) {
    let mut name = format!("rank10sel_{}_{}", UPPER_BLOCK_SIZE, LOG2_ONES_PER_INVENTORY);
    if !uniform {
        name.push_str("_non_uniform");
    }
    let mut group = c.benchmark_group(&name);
    bench_select::<sux::rank_sel::Rank10Sel<UPPER_BLOCK_SIZE, LOG2_ONES_PER_INVENTORY>, _>(
        &mut group, &LENS, &DENSITIES, REPS, uniform,
    );
    save_mem_cost::<sux::rank_sel::Rank10Sel<UPPER_BLOCK_SIZE, LOG2_ONES_PER_INVENTORY>>(
        &name,
        &LENS,
        &[0.5],
        uniform,
    );
    group.finish();
}

pub fn bench_cs_poppy(c: &mut Criterion, uniform: bool) {
    let mut name = "cs_poppy";
    if !uniform {
        name = "cs_poppy_non_uniform";
    }
    let mut group = c.benchmark_group(name);
    bench_select::<bitm::RankSelect101111<_, _, _>, _>(
        &mut group, &LENS, &DENSITIES, REPS, uniform,
    );
    save_mem_cost::<
        bitm::RankSelect101111<
            CombinedSampling<bitm::ConstCombinedSamplingDensity>,
            CombinedSampling<bitm::ConstCombinedSamplingDensity>,
            _,
        >,
    >(name, &LENS, &[0.5], uniform);
    group.finish();
}
