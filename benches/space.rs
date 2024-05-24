use bitm::CombinedSampling;
use std::env;
use valcurt::utils::{save_mem_cost, DENSITIES, LENS};
use valcurt::{SimpleSelect0, SimpleSelect1, SimpleSelect2, SimpleSelect3};

pub fn bench_simple_select0(uniform: bool) {
    let mut name = "simple_select0";
    if !uniform {
        name = "simple_select0_non_uniform";
    }
    save_mem_cost::<SimpleSelect0>(name, &LENS, &DENSITIES, uniform);
}

pub fn bench_simple_select1(uniform: bool) {
    let mut name = "simple_select1";
    if !uniform {
        name = "simple_select1_non_uniform";
    }
    save_mem_cost::<SimpleSelect1>(name, &LENS, &DENSITIES, uniform);
}

pub fn bench_simple_select2(uniform: bool) {
    let mut name = "simple_select2";
    if !uniform {
        name = "simple_select2_non_uniform";
    }
    save_mem_cost::<SimpleSelect2>(name, &LENS, &DENSITIES, uniform);
}

pub fn bench_simple_select3(uniform: bool) {
    let mut name = "simple_select3";
    if !uniform {
        name = "simple_select3_non_uniform";
    }
    save_mem_cost::<SimpleSelect3>(name, &LENS, &DENSITIES, uniform);
}

pub fn bench_rank9sel(uniform: bool) {
    let mut name = "rank9sel";
    if !uniform {
        name = "rank9sel_non_uniform";
    }
    save_mem_cost::<sux::rank_sel::Rank9Sel>(name, &LENS, &DENSITIES, uniform);
}

pub fn bench_rank10sel<const UPPER_BLOCK_SIZE: usize, const LOG2_ONES_PER_INVENTORY: usize>(
    uniform: bool,
) {
    let mut name = format!("rank10sel_{}_{}", UPPER_BLOCK_SIZE, LOG2_ONES_PER_INVENTORY);
    if !uniform {
        name.push_str("_non_uniform");
    }
    save_mem_cost::<sux::rank_sel::Rank10Sel<UPPER_BLOCK_SIZE, LOG2_ONES_PER_INVENTORY>>(
        &name, &LENS, &DENSITIES, uniform,
    );
}

pub fn bench_cs_poppy(uniform: bool) {
    let mut name = "cs_poppy";
    if !uniform {
        name = "cs_poppy_non_uniform";
    }
    save_mem_cost::<
        bitm::RankSelect101111<
            CombinedSampling<bitm::ConstCombinedSamplingDensity>,
            CombinedSampling<bitm::ConstCombinedSamplingDensity>,
            _,
        >,
    >(name, &LENS, &DENSITIES, uniform);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let uniform = !args.contains(&String::from("-nu"));

    bench_simple_select0(uniform);
    bench_simple_select1(uniform);
    bench_simple_select2(uniform);
    bench_simple_select3(uniform);
    bench_cs_poppy(uniform);
}
