use crate::utils::BenchSelect;

impl BenchSelect for sux::rank_sel::SimpleSelect {
    fn bench_select(&self, input: usize) -> usize {
        unsafe {
            <sux::rank_sel::SimpleSelect as sux::traits::Select>::select_unchecked(&self, input)
        }
    }
}

impl BenchSelect for sux::rank_sel::Rank9Sel {
    fn bench_select(&self, input: usize) -> usize {
        unsafe { <sux::rank_sel::Rank9Sel as sux::traits::Select>::select_unchecked(&self, input) }
    }
}

impl<const LOG2_LOWER_BLOCK_SIZE: usize, const LOG2_ONES_PER_INVENTORY: usize> BenchSelect
    for sux::rank_sel::Rank10Sel<LOG2_LOWER_BLOCK_SIZE, LOG2_ONES_PER_INVENTORY>
{
    fn bench_select(&self, input: usize) -> usize {
        unsafe {
            <sux::rank_sel::Rank10Sel<LOG2_LOWER_BLOCK_SIZE, LOG2_ONES_PER_INVENTORY> as sux::traits::Select>::select_unchecked(
                &self, input,
            )
        }
    }
}

impl BenchSelect
    for bitm::RankSelect101111<bitm::CombinedSampling, bitm::CombinedSampling, Box<[u64]>>
{
    fn bench_select(&self, input: usize) -> usize {
        unsafe { <bitm::RankSelect101111<_, _, _> as bitm::Select>::select_unchecked(&self, input) }
    }
}
