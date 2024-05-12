use crate::utils::Measure;
use mem_dbg::{MemSize, SizeFlags};

impl Measure for sux::rank_sel::Rank9 {
    fn new(data: Vec<usize>, len: usize) -> Self {
        let bitvec = unsafe { sux::bits::BitVec::from_raw_parts(data, len) };
        Self::new(bitvec)
    }

    fn len(&self) -> usize {
        <sux::rank_sel::Rank9 as sux::traits::BitLength>::len(&self)
    }

    fn mem_size(&self) -> usize {
        MemSize::mem_size(self, mem_dbg::SizeFlags::default())
    }
}

impl<const UPPER_BLOCK_SIZE: usize> Measure for sux::rank_sel::Rank10<UPPER_BLOCK_SIZE> {
    fn new(data: Vec<usize>, len: usize) -> Self {
        let bitvec = unsafe { sux::bits::BitVec::from_raw_parts(data, len) };
        Self::new(bitvec)
    }

    fn len(&self) -> usize {
        <sux::rank_sel::Rank10<UPPER_BLOCK_SIZE> as sux::traits::BitLength>::len(&self)
    }

    fn mem_size(&self) -> usize {
        MemSize::mem_size(self, mem_dbg::SizeFlags::default())
    }
}

impl Measure for sux::rank_sel::Rank11 {
    fn new(data: Vec<usize>, len: usize) -> Self {
        let bitvec = unsafe { sux::bits::BitVec::from_raw_parts(data, len) };
        Self::new(bitvec)
    }

    fn len(&self) -> usize {
        <sux::rank_sel::Rank11 as sux::traits::BitLength>::len(&self)
    }

    fn mem_size(&self) -> usize {
        MemSize::mem_size(self, mem_dbg::SizeFlags::default())
    }
}

impl Measure for sux::rank_sel::Rank12 {
    fn new(data: Vec<usize>, len: usize) -> Self {
        let bitvec = unsafe { sux::bits::BitVec::from_raw_parts(data, len) };
        Self::new(bitvec)
    }

    fn len(&self) -> usize {
        <sux::rank_sel::Rank12 as sux::traits::BitLength>::len(&self)
    }

    fn mem_size(&self) -> usize {
        MemSize::mem_size(self, mem_dbg::SizeFlags::default())
    }
}

impl Measure for sux::rank_sel::Rank16 {
    fn new(data: Vec<usize>, len: usize) -> Self {
        let bitvec = unsafe { sux::bits::BitVec::from_raw_parts(data, len) };
        Self::new(bitvec)
    }

    fn len(&self) -> usize {
        <sux::rank_sel::Rank16 as sux::traits::BitLength>::len(&self)
    }

    fn mem_size(&self) -> usize {
        MemSize::mem_size(self, mem_dbg::SizeFlags::default())
    }
}

impl Measure for bitm::ArrayWithRank101111 {
    fn new(data: Vec<usize>, len: usize) -> Self {
        let _ = len;
        bitm::ArrayWithRank101111::build(data.iter().map(|e| *e as u64).collect()).0
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

impl Measure for sux::rank_sel::SimpleSelect {
    fn new(data: Vec<usize>, len: usize) -> Self {
        let bitvec = unsafe { sux::bits::BitVec::from_raw_parts(data, len) };
        Self::new(bitvec, 3)
    }

    fn len(&self) -> usize {
        <sux::rank_sel::SimpleSelect as sux::traits::BitLength>::len(&self)
    }

    fn mem_size(&self) -> usize {
        MemSize::mem_size(self, mem_dbg::SizeFlags::default())
    }
}

impl Measure for sux::rank_sel::Rank9Sel {
    fn new(data: Vec<usize>, len: usize) -> Self {
        let bitvec = unsafe { sux::bits::BitVec::from_raw_parts(data, len) };
        Self::new(bitvec)
    }

    fn len(&self) -> usize {
        <sux::rank_sel::Rank9Sel as sux::traits::BitLength>::len(&self)
    }

    fn mem_size(&self) -> usize {
        MemSize::mem_size(self, mem_dbg::SizeFlags::default())
    }
}

impl<const UPPER_BLOCK_SIZE: usize, const LOG2_ONES_PER_INVENTORY: usize> Measure
    for sux::rank_sel::Rank10Sel<UPPER_BLOCK_SIZE, LOG2_ONES_PER_INVENTORY>
{
    fn new(data: Vec<usize>, len: usize) -> Self {
        let bitvec = unsafe { sux::bits::BitVec::from_raw_parts(data, len) };
        Self::new(bitvec)
    }

    fn len(&self) -> usize {
        <sux::rank_sel::Rank10Sel<UPPER_BLOCK_SIZE, LOG2_ONES_PER_INVENTORY> as sux::traits::BitLength>::len(&self)
    }

    fn mem_size(&self) -> usize {
        MemSize::mem_size(self, mem_dbg::SizeFlags::default())
    }
}

impl Measure
    for bitm::RankSelect101111<bitm::CombinedSampling, bitm::CombinedSampling, Box<[u64]>>
{
    fn new(data: Vec<usize>, len: usize) -> Self {
        let _ = len;
        bitm::RankSelect101111::<bitm::CombinedSampling, bitm::CombinedSampling, _>::build(
            data.iter().map(|e| *e as u64).collect(),
        )
        .0
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
