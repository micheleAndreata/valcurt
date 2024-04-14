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
    let lens = vec![
        (1u64 << 20) + 2,
        (1 << 21) + 2,
        (1 << 22) + 2,
        (1 << 23) + 2,
        (1 << 24) + 2,
        (1 << 25) + 2,
        (1 << 26) + 2,
        (1 << 27) + 2,
        (1 << 28) + 2,
        (1 << 29) + 2,
        (1 << 30) + 2,
    ];
    let densities = vec![0.25, 0.5, 0.75];
    let iterations = 70_000_000;

    let mut simple_eval: Evaluator<sux::rank_sel::SimpleSelect> = Evaluator::new(rng.clone());

    let mut rank9sel_eval: Evaluator<sux::rank_sel::Rank9Sel> = Evaluator::new(rng.clone());

    let mut rankselect101111_eval: Evaluator<bitm::RankSelect101111> = Evaluator::new(rng.clone());

    let mut sucds_rank9sel_eval: Evaluator<sucds::bit_vectors::rank9sel::Rank9Sel> =
        Evaluator::new(rng.clone());

    let mut succinct_sel_eval: Evaluator<
        succinct::BinSearchSelect<succinct::rank::Rank9<succinct::BitVector<u64>>>,
    > = Evaluator::new(rng.clone());

    println!("SimpleSelect...");
    simple_eval.bench(
        "SimpleSelect",
        &lens,
        &densities,
        true,
        repetitions,
        iterations,
    );

    println!("Rank9Sel...");
    rank9sel_eval.bench("Rank9Sel", &lens, &densities, true, repetitions, iterations);

    println!("RankSelect101111");
    rankselect101111_eval.bench(
        "RankSelect101111",
        &lens,
        &densities,
        true,
        repetitions,
        iterations,
    );

    println!("Sucds Rank9Sel...");
    sucds_rank9sel_eval.bench(
        "SucdsRank9Sel",
        &lens,
        &densities,
        true,
        repetitions,
        iterations,
    );

    println!("Succinct BinSearchSelect Rank9...");
    succinct_sel_eval.bench(
        "SuccinctBinSearchSelectRank9",
        &lens,
        &densities,
        true,
        repetitions,
        iterations,
    );

    println!("SimpleSelect non uniform...");
    simple_eval.bench(
        "SimpleSelect_non_uniform",
        &lens,
        &densities,
        false,
        repetitions,
        iterations,
    );
    println!("Rank9Sel non uniform...");
    rank9sel_eval.bench("Rank9Sel", &lens, &densities, true, repetitions, iterations);
    println!("RankSelect101111");
    rankselect101111_eval.bench(
        "RankSelect101111_non_uniform",
        &lens,
        &densities,
        false,
        repetitions,
        iterations,
    );

    println!("Sucds Rank9Sel non uniform...");
    sucds_rank9sel_eval.bench(
        "SucdsRank9Sel_non_uniform",
        &lens,
        &densities,
        false,
        repetitions,
        iterations,
    );

    println!("Succinct BinSearchSelect Rank9 non uniform...");
    succinct_sel_eval.bench(
        "SuccinctBinSearchSelectRank9_non_uniform",
        &lens,
        &densities,
        false,
        repetitions,
        iterations,
    );
}
