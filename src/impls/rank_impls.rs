use crate::utils::BenchRank;

impl BenchRank for sux::rank_sel::Rank9 {
    fn bench_rank(&self, input: usize) -> usize {
        unsafe { <sux::rank_sel::Rank9 as sux::traits::Rank>::rank_unchecked(&self, input) }
    }
}

impl<const LOG2_LOWER_BLOCK_SIZE: usize> BenchRank
    for sux::rank_sel::Rank10<LOG2_LOWER_BLOCK_SIZE>
{
    fn bench_rank(&self, input: usize) -> usize {
        unsafe {
            <sux::rank_sel::Rank10<LOG2_LOWER_BLOCK_SIZE> as sux::traits::Rank>::rank_unchecked(
                &self, input,
            )
        }
    }
}

impl BenchRank for sux::rank_sel::Rank11 {
    fn bench_rank(&self, input: usize) -> usize {
        unsafe { <sux::rank_sel::Rank11 as sux::traits::Rank>::rank_unchecked(&self, input) }
    }
}

impl BenchRank for sux::rank_sel::Rank12 {
    fn bench_rank(&self, input: usize) -> usize {
        unsafe { <sux::rank_sel::Rank12 as sux::traits::Rank>::rank_unchecked(&self, input) }
    }
}

impl BenchRank for sux::rank_sel::Rank16 {
    fn bench_rank(&self, input: usize) -> usize {
        unsafe { <sux::rank_sel::Rank16 as sux::traits::Rank>::rank_unchecked(&self, input) }
    }
}

impl BenchRank for bitm::ArrayWithRank101111 {
    fn bench_rank(&self, input: usize) -> usize {
        unsafe { <bitm::ArrayWithRank101111 as bitm::Rank>::rank_unchecked(&self, input) }
    }
}
