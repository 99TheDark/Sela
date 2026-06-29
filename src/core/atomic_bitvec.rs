/*use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

const BITS: usize = usize::BITS as usize;

pub struct AtomicBitArray<const S: usize> {
    blocks: [AtomicUsize; (S + BITS - 1) / BITS],
    locked: AtomicBool, // TODO: Implement
}

impl<const S: usize> AtomicBitArray<S> {
    pub const fn new() -> Self {
        Self {
            blocks: [const { AtomicUsize::new(0) }; (S + BITS - 1) / BITS],
            locked: AtomicBool::new(false),
        }
    }

    #[inline(always)]
    fn get_indexes(&self, index: usize) -> (usize, usize) {
        let block_idx = index / BITS;
        let inner_idx = index % BITS;

        (block_idx, inner_idx)
    }

    pub fn get(&self, index: usize) -> bool {
        let (block_idx, inner_idx) = self.get_indexes(index);
        let block = self.blocks[block_idx].load(Ordering::Relaxed);

        let mask = 1 << inner_idx;
        block & mask != 0
    }

    pub fn set(&mut self, index: usize, value: bool) {
        let (block_idx, inner_idx) = self.get_indexes(index);
        let block = &mut self.blocks[block_idx];

        let mask = 1 << inner_idx;
        if value {
            block.fetch_or(mask, Ordering::Relaxed);
        } else {
            block.fetch_and(!mask, Ordering::Relaxed);
        }
    }

    pub fn all(&self, value: bool) -> bool {
        for block in &self.blocks {
            if block.load(Ordering::Relaxed) != (value as usize) {
                return false;
            }
        }
        true
    }

    pub fn clear(&mut self) {
        for block in &mut self.blocks {
            block.store(0, Ordering::Relaxed);
        }
    }
}*/
