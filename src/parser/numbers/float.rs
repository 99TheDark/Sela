use std::convert;

use crate::parser::numbers::{ErrorSet, IsEmpty, NumberParsingError, int};

#[derive(Copy, Clone)]
pub enum ErrorKind {
    UnderConsecutive,
    UnderEnd,
    UnderPreDot,
    UnderPostDot,
    UnderPreExp,
    UnderPostExp,
    UnderPreExpSgn,
    UnderPostExpSgn,
    LeadingZeros,
    NonNumeric,
    TooLarge,
}

pub struct ParsingError(pub(super) NumberParsingError);

impl ParsingError {
    pub fn new() -> Self {
        Self(NumberParsingError::new())
    }
}

impl ErrorSet for ParsingError {
    type Kind = ErrorKind;

    fn has(&self, kind: Self::Kind) -> bool {
        use ErrorKind::*;
        match kind {
            UnderConsecutive => self.0.under_consecutive(),
            UnderEnd => self.0.under_end(),
            UnderPreDot => self.0.under_pre_dot(),
            UnderPostDot => self.0.under_post_dot(),
            UnderPreExp => self.0.under_pre_exp(),
            UnderPostExp => self.0.under_post_exp(),
            UnderPreExpSgn => self.0.under_pre_exp_sgn(),
            UnderPostExpSgn => self.0.under_post_exp_sgn(),
            LeadingZeros => self.0.leading_zeros(),
            NonNumeric => self.0.non_numeric(),
            TooLarge => self.0.too_large(),
        }
    }

    fn raise(&mut self, kind: Self::Kind) {
        use ErrorKind::*;
        match kind {
            UnderConsecutive => self.0.set_under_consecutive(true),
            UnderEnd => self.0.set_under_end(true),
            UnderPreDot => self.0.set_under_pre_dot(true),
            UnderPostDot => self.0.set_under_post_dot(true),
            UnderPreExp => self.0.set_under_pre_exp(true),
            UnderPostExp => self.0.set_under_post_exp(true),
            UnderPreExpSgn => self.0.set_under_pre_exp_sgn(true),
            UnderPostExpSgn => self.0.set_under_post_exp_sgn(true),
            LeadingZeros => self.0.set_leading_zeros(true),
            NonNumeric => self.0.set_non_numeric(true),
            TooLarge => self.0.set_too_large(true),
        }
    }
}

impl convert::From<int::ParsingError> for ParsingError {
    fn from(value: int::ParsingError) -> Self {
        Self(value.0)
    }
}

impl IsEmpty for ParsingError {
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
