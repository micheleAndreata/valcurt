use bitm::RankSelect101111;
use rand::{rngs::SmallRng, SeedableRng};
use sux::rank_sel::{Rank9Sel, SimpleSelect};
use valcurt::evaluator::Evaluator;

fn main() {
    let rng = SmallRng::seed_from_u64(0);
    let repetitions = 10;
    let lens = vec![1_000_000];
    let densities = vec![0.5];
    let uniform = false;
    let iterations = 80_000_000;

    let mut simple_evaluator: Evaluator<SimpleSelect> = Evaluator::new(rng.clone());

    let mut rank9sel_evaluator: Evaluator<Rank9Sel> = Evaluator::new(rng.clone());

    let mut rankselect101111_evaluator: Evaluator<RankSelect101111> = Evaluator::new(rng.clone());

    println!("SimpleSelect");
    simple_evaluator.bench(
        "SimpleSelect",
        &lens,
        &densities,
        uniform,
        repetitions,
        iterations,
    );
    println!("Rank9Sel");
    rank9sel_evaluator.bench(
        "Rank9Sel",
        &lens,
        &densities,
        uniform,
        repetitions,
        iterations,
    );
    println!("RankSelect101111");
    rankselect101111_evaluator.bench(
        "RankSelect101111",
        &lens,
        &densities,
        uniform,
        repetitions,
        iterations,
    );
}
