use std::{
    hint,
    ops::{Bound, RangeBounds},
    simd::{
        self, Simd,
        cmp::{SimdPartialEq, SimdPartialOrd},
        u8x16,
    },
};

use crate::lexer::{Lexer, words::WordLegal};

trait SimdRange<T: simd::SimdElement, const N: usize> {
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

// TODO: Split among files
impl<'tok, 'src> Lexer<'tok, 'src> {
    fn ident_simd_chunk(&self, offset: usize) -> usize {
        let start = self.idx + offset;
        let bytes = u8x16::from_slice(&self.bytes[start..start + u8x16::LEN]);

        let is_ident = bytes.simd_in_range(b'a'..=b'z')
            | bytes.simd_in_range(b'A'..=b'Z')
            | bytes.simd_eq(u8x16::splat(b'_'))
            | bytes.simd_in_range(b'0'..=b'9');

        is_ident.to_bitmask().trailing_ones() as usize
    }

    fn ident_remaining(&self, offset: usize) -> usize {
        for (idx, byte) in self.bytes[self.idx + offset..].iter().enumerate() {
            if !byte.word_legal() {
                return idx;
            }
        }
        self.bytes.len() - (self.idx + offset)
    }

    pub(super) fn eat_ident_simd(&self, skip: usize) -> usize {
        let mut offset = skip;
        loop {
            let delta = if self.bytes.len() < self.idx + offset + u8x16::LEN {
                hint::cold_path();
                self.ident_remaining(offset)
            } else {
                self.ident_simd_chunk(offset)
            };

            offset += delta;
            if delta != u8x16::LEN {
                break;
            }
        }
        offset
    }
}
