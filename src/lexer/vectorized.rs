use std::{
    hint,
    ops::{Bound, RangeBounds},
    simd::{self, Simd, cmp::SimdPartialOrd, mask8x16, u8x16},
};

use crate::lexer::Lexer;

pub(super) trait SimdRange<T: simd::SimdElement, const N: usize> {
    fn simd_in_range(&self, range: impl RangeBounds<T>) -> simd::Mask<T::Mask, N>;
    // fn simd_out_range(&self, range: impl RangeBounds<T>) -> simd::Mask<T::Mask, N>;
}

impl<T: simd::SimdElement, const N: usize> SimdRange<T, N> for Simd<T, N>
where
    Self: SimdPartialOrd<Mask = simd::Mask<T::Mask, N>>,
{
    fn simd_in_range(&self, range: impl RangeBounds<T>) -> simd::Mask<T::Mask, N> {
        let start_ok = match range.start_bound() {
            Bound::Included(start) => self.simd_ge(Self::splat(*start)),
            Bound::Excluded(start) => self.simd_gt(Self::splat(*start)),
            Bound::Unbounded => simd::Mask::splat(true),
        };
        let end_ok = match range.end_bound() {
            Bound::Included(end) => self.simd_le(Self::splat(*end)),
            Bound::Excluded(end) => self.simd_lt(Self::splat(*end)),
            Bound::Unbounded => simd::Mask::splat(true),
        };

        start_ok & end_ok
    }
}

impl<'tok, 'src> Lexer<'tok, 'src> {
    #[inline(always)]
    pub(super) fn eat_until_simd(
        &self,
        skip: usize,
        simd_chunk: impl Fn(u8x16) -> mask8x16,
        eat_remaining_until: impl Fn(&u8) -> bool,
    ) -> usize {
        let mut offset = skip;
        loop {
            let delta = 'delta: {
                if self.bytes.len() < self.idx + offset + u8x16::LEN {
                    hint::cold_path();

                    for (idx, byte) in self.bytes[self.idx + offset..].iter().enumerate() {
                        if eat_remaining_until(byte) {
                            break 'delta idx;
                        }
                    }
                    self.bytes.len() - (self.idx + offset)
                } else {
                    let start = self.idx + offset;
                    let bytes = u8x16::from_slice(&self.bytes[start..start + u8x16::LEN]);
                    simd_chunk(bytes).to_bitmask().trailing_ones() as usize
                }
            };

            offset += delta;
            if delta != u8x16::LEN {
                break;
            }
        }
        offset
    }
}
