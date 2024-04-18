use crate::evaluator::Evaluate;
use mem_dbg::{MemSize, SizeFlags};

impl Evaluate for sux::rank_sel::SimpleSelect {
    fn new(data: Vec<usize>, len: usize) -> Self {
        let bitvec = unsafe { sux::bits::BitVec::from_raw_parts(data, len) };
        Self::new(bitvec, 3)
    }

    fn benched_fn(&self, input: usize) -> usize {
        unsafe {
            <sux::rank_sel::SimpleSelect as sux::traits::Select>::select_unchecked(&self, input)
        }
    }

    fn len(&self) -> usize {
        <sux::rank_sel::SimpleSelect as sux::traits::BitLength>::len(&self)
    }

    fn mem_size(&self) -> usize {
        MemSize::mem_size(self, mem_dbg::SizeFlags::default())
    }
}

impl Evaluate for sux::rank_sel::Rank9Sel {
    fn new(data: Vec<usize>, len: usize) -> Self {
        let bitvec = unsafe { sux::bits::BitVec::from_raw_parts(data, len) };
        Self::new(bitvec)
    }

    fn benched_fn(&self, input: usize) -> usize {
        unsafe { <sux::rank_sel::Rank9Sel as sux::traits::Select>::select_unchecked(&self, input) }
    }

    fn len(&self) -> usize {
        <sux::rank_sel::Rank9Sel as sux::traits::BitLength>::len(&self)
    }

    fn mem_size(&self) -> usize {
        MemSize::mem_size(self, mem_dbg::SizeFlags::default())
    }
}

impl Evaluate
    for bitm::RankSelect101111<bitm::CombinedSampling, bitm::CombinedSampling, Box<[u64]>>
{
    fn new(data: Vec<usize>, len: usize) -> Self {
        let _ = len;
        bitm::RankSelect101111::<bitm::CombinedSampling, bitm::CombinedSampling, _>::build(
            data.iter().map(|e| *e as u64).collect(),
        )
        .0
    }

    fn benched_fn(&self, input: usize) -> usize {
        unsafe { <bitm::RankSelect101111<_, _, _> as bitm::Select>::select_unchecked(&self, input) }
    }

    fn len(&self) -> usize {
        <[u64] as bitm::BitAccess>::bit_iter(&self.content).len()
    }

    fn mem_size(&self) -> usize {
        self.content.mem_size(SizeFlags::default())
            + self.l1ranks.mem_size(SizeFlags::default())
            + self.l2ranks.mem_size(SizeFlags::default())
    }
}

impl Evaluate for sucds::bit_vectors::rank9sel::Rank9Sel {
    fn new(data: Vec<usize>, len: usize) -> Self {
        let bitvec = unsafe { sux::bits::BitVec::from_raw_parts(data, len) };
        Self::from_bits(bitvec.into_iter()).select1_hints()
    }

    fn benched_fn(&self, input: usize) -> usize {
        sucds::bit_vectors::Select::select1(self, input as usize).unwrap()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn mem_size(&self) -> usize {
        sucds::Serializable::size_in_bytes(self)
    }
}
