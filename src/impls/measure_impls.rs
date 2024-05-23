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

impl<const LOG2_LOWER_BLOCK_SIZE: usize> Measure for sux::rank_sel::Rank10<LOG2_LOWER_BLOCK_SIZE> {
    fn new(data: Vec<usize>, len: usize) -> Self {
        let bitvec = unsafe { sux::bits::BitVec::from_raw_parts(data, len) };
        Self::new(bitvec)
    }

    fn len(&self) -> usize {
        <sux::rank_sel::Rank10<LOG2_LOWER_BLOCK_SIZE> as sux::traits::BitLength>::len(&self)
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

pub struct SimpleSelect0 {
    pub select: sux::rank_sel::SimpleSelect,
}

pub struct SimpleSelect1 {
    pub select: sux::rank_sel::SimpleSelect,
}

pub struct SimpleSelect2 {
    pub select: sux::rank_sel::SimpleSelect,
}

pub struct SimpleSelect3 {
    pub select: sux::rank_sel::SimpleSelect,
}

impl Measure for SimpleSelect0 {
    fn new(data: Vec<usize>, len: usize) -> Self {
        let bitvec = unsafe { sux::bits::BitVec::from_raw_parts(data, len) };
        Self {
            select: sux::rank_sel::SimpleSelect::new(bitvec, 0),
        }
    }

    fn len(&self) -> usize {
        <sux::rank_sel::SimpleSelect as sux::traits::BitLength>::len(&self.select)
    }

    fn mem_size(&self) -> usize {
        MemSize::mem_size(&self.select, mem_dbg::SizeFlags::default())
    }
}

impl Measure for SimpleSelect1 {
    fn new(data: Vec<usize>, len: usize) -> Self {
        let bitvec = unsafe { sux::bits::BitVec::from_raw_parts(data, len) };
        Self {
            select: sux::rank_sel::SimpleSelect::new(bitvec, 1),
        }
    }

    fn len(&self) -> usize {
        <sux::rank_sel::SimpleSelect as sux::traits::BitLength>::len(&self.select)
    }

    fn mem_size(&self) -> usize {
        MemSize::mem_size(&self.select, mem_dbg::SizeFlags::default())
    }
}

impl Measure for SimpleSelect2 {
    fn new(data: Vec<usize>, len: usize) -> Self {
        let bitvec = unsafe { sux::bits::BitVec::from_raw_parts(data, len) };
        Self {
            select: sux::rank_sel::SimpleSelect::new(bitvec, 2),
        }
    }

    fn len(&self) -> usize {
        <sux::rank_sel::SimpleSelect as sux::traits::BitLength>::len(&self.select)
    }

    fn mem_size(&self) -> usize {
        MemSize::mem_size(&self.select, mem_dbg::SizeFlags::default())
    }
}

impl Measure for SimpleSelect3 {
    fn new(data: Vec<usize>, len: usize) -> Self {
        let bitvec = unsafe { sux::bits::BitVec::from_raw_parts(data, len) };
        Self {
            select: sux::rank_sel::SimpleSelect::new(bitvec, 3),
        }
    }

    fn len(&self) -> usize {
        <sux::rank_sel::SimpleSelect as sux::traits::BitLength>::len(&self.select)
    }

    fn mem_size(&self) -> usize {
        MemSize::mem_size(&self.select, mem_dbg::SizeFlags::default())
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

impl<const LOG2_LOWER_BLOCK_SIZE: usize, const LOG2_ONES_PER_INVENTORY: usize> Measure
    for sux::rank_sel::Rank10Sel<LOG2_LOWER_BLOCK_SIZE, LOG2_ONES_PER_INVENTORY>
{
    fn new(data: Vec<usize>, len: usize) -> Self {
        let bitvec = unsafe { sux::bits::BitVec::from_raw_parts(data, len) };
        Self::new(bitvec)
    }

    fn len(&self) -> usize {
        <sux::rank_sel::Rank10Sel<LOG2_LOWER_BLOCK_SIZE, LOG2_ONES_PER_INVENTORY> as sux::traits::BitLength>::len(&self)
    }

    fn mem_size(&self) -> usize {
        MemSize::mem_size(self, mem_dbg::SizeFlags::default())
    }
}

impl Measure
    for bitm::RankSelect101111<
        bitm::CombinedSampling<bitm::ConstCombinedSamplingDensity>,
        bitm::CombinedSampling<bitm::ConstCombinedSamplingDensity>,
        Box<[u64]>,
    >
{
    fn new(data: Vec<usize>, len: usize) -> Self {
        let _ = len;
        bitm::RankSelect101111::<
            bitm::CombinedSampling<bitm::ConstCombinedSamplingDensity>,
            bitm::CombinedSampling<bitm::ConstCombinedSamplingDensity>,
            _,
        >::build(data.iter().map(|e| *e as u64).collect())
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
