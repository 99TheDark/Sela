use core::fmt;
use std::{convert, ops};

use crate::{
    core::scalb::ScalbTen,
    parser::numbers::{ErrorSet, IsEmpty, NumberParsingError, UnwrapAccumulate, int},
};

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
    MissingExpVal,
    SecondExp,
    ExpBeforeDot,
    LeadingZeros,
    NonNumeric,
    TooLarge,
}

#[derive(Debug, Copy, Clone)]
pub struct ParsingError(pub(super) NumberParsingError);

impl ParsingError {
    pub fn new() -> Self {
        Self(NumberParsingError::new())
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl ops::Deref for ParsingError {
    type Target = NumberParsingError;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for ParsingError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
            MissingExpVal => self.0.missing_exp_val(),
            SecondExp => self.0.second_exp(),
            ExpBeforeDot => self.0.exp_before_dot(),
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
            MissingExpVal => self.0.set_missing_exp_val(true),
            SecondExp => self.0.set_second_exp(true),
            ExpBeforeDot => self.0.set_exp_before_dot(true),
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

pub(super) fn parse_bytes(mut src: &[u8]) -> Result<f64, ParsingError> {
    const MAX_DIGITS: usize = 17;

    // One of: X.Y, X.YeZ, X.Ye+Z, X.Ye-Z, XeY, Xe+Y, Xe-Y
    let mut errors = ParsingError::new();

    let int = int::parse_bytes_for_float(&src, MAX_DIGITS, false, true).unwrap_acc(&mut errors);

    // Guaranteed to have another byte (X.X or XeX)
    src = &src[int.len..];
    let dec = if src[0] == b'.' {
        src = &src[1..];
        int::parse_bytes_for_float(&src, MAX_DIGITS - int.num_digits_capped, true, true)
            .unwrap_acc(&mut errors)
    } else {
        int::FloatParsingOutput::default()
    };

    // Might not necessarily have another byte if dec_len > 0
    src = &src[dec.len..];
    let (exp, exp_sign) = if matches!(src.get(0), Some(b'e' | b'E')) {
        let (sign, sign_len) = match src.get(1) {
            Some(b'+') => (1, 1),
            Some(b'-') => (-1, 1),
            Some(_) => (1, 0),
            None => (0, 0),
        };
        src = &src[1 + sign_len..];

        if src.is_empty() {
            errors.raise(ErrorKind::MissingExpVal);
            (int::FloatParsingOutput::default(), 0)
        } else {
            let exp =
                int::parse_bytes_for_float(&src, usize::MAX, false, false).unwrap_acc(&mut errors);
            if exp.num_digits != 0 { (exp, sign) } else { (int::FloatParsingOutput::default(), 0) }
        }
    } else {
        (int::FloatParsingOutput::default(), 0)
    };

    if exp.len != src.len() {
        match src[exp.len] {
            b'.' => errors.raise(ErrorKind::ExpBeforeDot),
            b'e' | b'E' => errors.raise(ErrorKind::SecondExp),
            _ => errors.raise(ErrorKind::NonNumeric),
        }
    }

    let result = 'result: {
        let Some(int_shift_factor) = 10u64
            .checked_pow((dec.num_digits_capped + int.num_digits - int.num_digits_capped) as u32)
        else {
            break 'result f64::INFINITY;
        };

        let significand = int.part * int_shift_factor + dec.part;
        let scaling_factor = exp.part as i64 * exp_sign - dec.num_digits_capped as i64;

        significand.scalb10(scaling_factor as i32)
    };

    if errors.is_empty() { Ok(result) } else { Err(errors) }
}
