use crate::evaluator::Evaluate;
use mem_dbg::{MemSize, SizeFlags};
use succinct::BitVec;

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

impl Evaluate for bitm::RankSelect101111 {
    fn new(data: Vec<usize>, len: usize) -> Self {
        let _ = len;
        bitm::RankSelect101111::build(data.iter().map(|e| *e as u64).collect()).0
    }

    fn benched_fn(&self, input: usize) -> usize {
        unsafe { <bitm::RankSelect101111 as bitm::Select>::select_unchecked(&self, input) }
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

impl Evaluate for succinct::BinSearchSelect<succinct::rank::Rank9<succinct::BitVector<u64>>> {
    fn new(data: Vec<usize>, len: usize) -> Self {
        let bv = unsafe { sux::bits::BitVec::from_raw_parts(data, len) };
        let mut bitvec = succinct::BitVector::<u64>::new();
        for b in bv.into_iter() {
            <succinct::BitVector<u64> as succinct::BitVecPush>::push_bit(&mut bitvec, b);
        }
        Self::new(succinct::rank::Rank9::new(bitvec))
    }

    fn benched_fn(&self, input: usize) -> usize {
        succinct::Select1Support::select1(self, input as u64).unwrap() as usize
    }

    fn len(&self) -> usize {
        self.inner().inner().bit_len() as usize
    }

    fn mem_size(&self) -> usize {
        succinct::SpaceUsage::total_bytes(self)
    }
}

impl Evaluate for vers_vecs::RsVec {
    fn new(data: Vec<usize>, len: usize) -> Self {
        let bv = unsafe { sux::bits::BitVec::from_raw_parts(data, len) };
        let bitvec =
            vers_vecs::BitVec::from_bits(&bv.into_iter().map(|b| b as u8).collect::<Vec<u8>>());
        Self::from_bit_vec(bitvec)
    }

    fn benched_fn(&self, input: usize) -> usize {
        self.select1(input)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn mem_size(&self) -> usize {
        todo!()
    }
}
