use bitm::ArrayWithRank101111;
use rand::{rngs::SmallRng, SeedableRng};
use std::env;
use valcurt::evaluator::Evaluator;

fn evaluate_select(lens: &[u64], densities: &[f64], repetitions: usize, iterations: usize) {
    let rng = SmallRng::seed_from_u64(0);

    let mut simple_eval: Evaluator<sux::rank_sel::SimpleSelect> = Evaluator::new(rng.clone());

    let mut rank9sel_eval: Evaluator<sux::rank_sel::Rank9Sel> = Evaluator::new(rng.clone());

    let mut sucds_rank9sel_eval: Evaluator<sucds::bit_vectors::rank9sel::Rank9Sel> =
        Evaluator::new(rng.clone());

    let mut bitm_ranksel101111_eval: Evaluator<
        bitm::RankSelect101111<bitm::CombinedSampling, bitm::CombinedSampling, Box<[u64]>>,
    > = Evaluator::new(rng.clone());

    simple_eval.validate_select();
    rank9sel_eval.validate_select();
    sucds_rank9sel_eval.validate_select();
    bitm_ranksel101111_eval.validate_select();

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

    println!("Sucds Rank9Sel...");
    sucds_rank9sel_eval.bench(
        "SucdsRank9Sel",
        &lens,
        &densities,
        true,
        repetitions,
        iterations,
    );

    println!("BitM RankSelect101111...");
    bitm_ranksel101111_eval.bench(
        "BitMRankSelect101111",
        &lens,
        &densities,
        true,
        repetitions,
        iterations,
    );
}

fn evaluate_rank(lens: &[u64], densities: &[f64], repetitions: usize, iterations: usize) {
    let rng = SmallRng::seed_from_u64(0);

    let mut rank9_eval: Evaluator<sux::rank_sel::Rank9> = Evaluator::new(rng.clone());

    let mut rank11_eval: Evaluator<sux::rank_sel::Rank11> = Evaluator::new(rng.clone());

    let mut rank12_eval: Evaluator<sux::rank_sel::Rank12> = Evaluator::new(rng.clone());

    let mut rank16_eval: Evaluator<sux::rank_sel::Rank16> = Evaluator::new(rng.clone());

    let mut rank101111_eval: Evaluator<ArrayWithRank101111> = Evaluator::new(rng.clone());

    println!("Rank9...");
    rank9_eval.bench("Rank9", &lens, &densities, true, repetitions, iterations);
    rank11_eval.bench("Rank11", &lens, &densities, true, repetitions, iterations);
    rank12_eval.bench("Rank12", &lens, &densities, true, repetitions, iterations);
    rank16_eval.bench("Rank16", &lens, &densities, true, repetitions, iterations);
    rank101111_eval.bench(
        "Rank101111",
        &lens,
        &densities,
        true,
        repetitions,
        iterations,
    );
}

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

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide an argument: 'rank' or 'select'");
        return;
    }

    let evaluation_type = &args[1];
    if evaluation_type != "rank" && evaluation_type != "select" {
        println!("Invalid argument: '{}'", evaluation_type);
        println!("Please provide an argument: 'rank' or 'select'");
        return;
    }

    let lens = vec![
        (1u64 << 20) + 2,
        (1 << 22) + 2,
        (1 << 24) + 2,
        (1 << 26) + 2,
        (1 << 28) + 2,
        (1 << 30) + 2,
        (1 << 32) + 2,
        (1 << 34) + 2,
    ];
    let densities = vec![0.25, 0.5, 0.75];
    let repetitions = 10;
    let iterations = 70_000_000;

    if evaluation_type == "rank" {
        evaluate_rank(&lens, &densities, repetitions, iterations);
    } else if evaluation_type == "select" {
        evaluate_select(&lens, &densities, repetitions, iterations);
    }
}
