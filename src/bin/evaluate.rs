use rand::{rngs::SmallRng, SeedableRng};
use valcurt::evaluator::Evaluator;

fn main() {
    if let Some(core_ids) = core_affinity::get_core_ids() {
        // Not core 0. Anything goes.
        let core_id = core_ids[1];
        if !core_affinity::set_for_current(core_id) {
            eprintln!("Cannot pin thread to core {:?}", core_id);
        }
    } else {
        eprintln!("Cannot retrieve core ids");
    }

    let rng = SmallRng::seed_from_u64(0);
    let repetitions = 10;
    let lens = vec![1_000_000_000];
    let densities = vec![0.5];
    let uniform = false;
    let iterations = 80_000_000;

    let mut simple_eval: Evaluator<sux::rank_sel::SimpleSelect> = Evaluator::new(rng.clone());

    let mut rank9sel_eval: Evaluator<sux::rank_sel::Rank9Sel> = Evaluator::new(rng.clone());

    let mut rankselect101111_eval: Evaluator<bitm::RankSelect101111> = Evaluator::new(rng.clone());

    let mut sucds_rank9sel_eval: Evaluator<sucds::bit_vectors::rank9sel::Rank9Sel> =
        Evaluator::new(rng.clone());

    let mut succinct_sel_eval: Evaluator<
        succinct::BinSearchSelect<succinct::rank::Rank9<succinct::BitVector<u64>>>,
    > = Evaluator::new(rng.clone());

    println!("SimpleSelect");
    simple_eval.bench(
        "SimpleSelect",
        &lens,
        &densities,
        uniform,
        repetitions,
        iterations,
    );
    println!("Rank9Sel");
    rank9sel_eval.bench(
        "Rank9Sel",
        &lens,
        &densities,
        uniform,
        repetitions,
        iterations,
    );
    println!("RankSelect101111");
    rankselect101111_eval.bench(
        "RankSelect101111",
        &lens,
        &densities,
        uniform,
        repetitions,
        iterations,
    );

    println!("Sucds Rank9Sel");
    sucds_rank9sel_eval.bench(
        "SucdsRank9Sel",
        &lens,
        &densities,
        uniform,
        repetitions,
        iterations,
    );

    println!("Succinct BinSearchSelect Rank9");
    succinct_sel_eval.bench(
        "SuccinctBinSearchSelectRank9",
        &lens,
        &densities,
        uniform,
        repetitions,
        iterations,
    );
}
