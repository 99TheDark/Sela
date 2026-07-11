use std::hint;

use crate::parser::numbers::{ErrorSet, IsEmpty, NumberParsingError};

#[derive(Copy, Clone)]
pub enum ErrorKind {
    UnderConsecutive,
    UnderEnd,
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
            NonNumeric => self.0.non_numeric(),
            TooLarge => self.0.too_large(),
        }
    }

    fn raise(&mut self, kind: Self::Kind) {
        use ErrorKind::*;
        match kind {
            UnderConsecutive => self.0.set_under_consecutive(true),
            UnderEnd => self.0.set_under_end(true),
            NonNumeric => self.0.set_non_numeric(true),
            TooLarge => self.0.set_too_large(true),
        }
    }
}

impl IsEmpty for ParsingError {
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub(super) fn parse_bytes(mut src: &[u8]) -> Result<u64, ParsingError> {
    let mut errors = ParsingError::new();
    let mut result = 0u64;

    while let [byte, rest @ ..] = src {
        'byte_match: {
            match (byte, rest.get(0)) {
                (b'_', Some(b'_')) => errors.raise(ErrorKind::UnderConsecutive),
                (b'_', None) => errors.raise(ErrorKind::UnderEnd),
                (b'_', _) => {}
                (byte @ b'0'..=b'9', _) => {
                    if errors.has(ErrorKind::TooLarge) {
                        hint::cold_path();
                        break 'byte_match;
                    }

                    let digit = byte - b'0';
                    let new_result = result
                        .checked_mul(10)
                        .and_then(|result| result.checked_add(digit as u64));

                    if let Some(new_result) = new_result {
                        result = new_result;
                    } else {
                        errors.raise(ErrorKind::TooLarge);
                    }
                }
                _ => errors.raise(ErrorKind::NonNumeric),
            }
        }
        src = &src[1..];
    }

    if errors.is_empty() { Ok(result) } else { Err(errors) }
}
