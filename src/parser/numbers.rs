mod float;
mod int;
mod radix_int;

use std::{fmt, hint, iter::zip, ops};

use arrayvec::ArrayVec;
use modular_bitfield::bitfield;

use crate::{
    ast,
    core::span::Span,
    diagnostics::{Diagnostics, ErrorKind, natural::Natural},
    parser::Parser,
    token::Token,
};

#[bitfield(filled = false)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct NumberParsingError {
    pub under_consecutive: bool,
    pub under_end: bool,
    pub under_pre_dot: bool,
    pub under_post_dot: bool,
    pub under_pre_exp: bool,
    pub under_post_exp: bool,
    pub under_pre_exp_sgn: bool,
    pub under_post_exp_sgn: bool,
    pub under_post_radix: bool,
    pub incomplete_radix: bool,
    pub unsupported_radix: bool,
    pub missing_exp_val: bool,
    pub second_exp: bool,
    pub exp_before_dot: bool,
    pub leading_zeros: bool,
    pub invalid_digit: bool,
    pub non_numeric: bool,
    pub too_large: bool,
}

impl fmt::Display for NumberParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const FIELDS: [(
            fn(&NumberParsingError) -> <bool as ::modular_bitfield::Specifier>::InOut,
            &'static str,
        ); 15] = [
            (NumberParsingError::under_consecutive, "under_consecutive"),
            (NumberParsingError::under_end, "under_end"),
            (NumberParsingError::under_pre_dot, "under_pre_dot"),
            (NumberParsingError::under_post_dot, "under_post_dot"),
            (NumberParsingError::under_pre_exp, "under_pre_exp"),
            (NumberParsingError::under_post_exp, "under_post_exp"),
            (NumberParsingError::under_pre_exp_sgn, "under_pre_exp_sgn"),
            (NumberParsingError::under_post_exp_sgn, "under_post_exp_sgn"),
            (NumberParsingError::under_post_radix, "under_post_radix"),
            (NumberParsingError::incomplete_radix, "incomplete_radix"),
            (NumberParsingError::unsupported_radix, "unsupported_radix"),
            (NumberParsingError::leading_zeros, "leading_zeros"),
            (NumberParsingError::invalid_digit, "invalid_digit"),
            (NumberParsingError::non_numeric, "non_numeric"),
            (NumberParsingError::too_large, "too_large"),
        ];

        f.write_str("{ ")?;
        let joined = FIELDS
            .iter()
            .filter_map(|(func, name)| if func(&self) { Some(*name) } else { None })
            .collect::<Vec<&'static str>>()
            .join(", ");
        f.write_str(&joined)?;
        f.write_str(" }")?;
        Ok(())
    }
}

trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl IsEmpty for NumberParsingError {
    #[inline(always)]
    fn is_empty(&self) -> bool {
        *self == Self::new()
    }
}

trait ErrorSet
where
    Self: ops::Deref<Target = NumberParsingError> + ops::DerefMut,
{
    type Kind: Copy;

    fn has(&self, kind: Self::Kind) -> bool;
    fn raise(&mut self, kind: Self::Kind); // `flag` maybe instead?
}

impl ops::BitOrAssign<NumberParsingError> for NumberParsingError {
    fn bitor_assign(&mut self, rhs: NumberParsingError) {
        // I hope this loop is unrolled... I need to check
        for (self_chunk, rhs_chunk) in zip(self.bytes.iter_mut(), rhs.bytes) {
            *self_chunk |= rhs_chunk;
        }
    }
}

trait UnwrapAccumulate<T, U: ErrorSet> {
    fn unwrap_acc(self, accumulator: &mut U) -> T;
}

impl<T: Default, U: ErrorSet, V: ErrorSet> UnwrapAccumulate<T, V> for Result<T, U> {
    fn unwrap_acc(self, accumulator: &mut V) -> T {
        match self {
            Ok(val) => val,
            Err(err) => {
                **accumulator |= *err;
                T::default()
            }
        }
    }
}

impl NumberParsingError {
    pub fn emit(&self, lit_kind: &str, span: Span, diag: &mut Diagnostics) {
        let mut under_errs = ArrayVec::<&str, 9>::new();
        if self.under_consecutive() {
            under_errs.push("consecutively");
        }
        if self.under_end() {
            under_errs.push("at the end of a number");
        }
        if self.under_pre_dot() {
            under_errs.push("before the decimal point");
        }
        if self.under_post_dot() {
            under_errs.push("after the decimal point");
        }
        if self.under_pre_exp() {
            under_errs.push("before the exponent");
        }
        if self.under_post_exp() {
            under_errs.push("after the exponent");
        }
        if self.under_pre_exp_sgn() {
            under_errs.push("before the sign of the exponent");
        }
        if self.under_post_exp_sgn() {
            under_errs.push("after the sign of the exponent");
        }
        if self.under_post_radix() {
            under_errs.push("after the radix");
        }

        // Should this be SmallVec since usually you have 0-2 errors?
        let mut errors = ArrayVec::<String, 10>::new();
        if !under_errs.is_empty() {
            errors
                .push(format!("underscores may not appear {}", under_errs.join_natural(",", "or")));
        }

        if self.incomplete_radix() {
            errors.push("the radix integer contains no value".to_string());
        }

        // Maybe in a future Diagnostics implementation I can not convert everything
        // to a string...
        if self.unsupported_radix() {
            // TODO: Be more helpful than this
            errors.push(
                "the literal is written with a radix other than \
                 0x (hexadecimal), 0o (octal), or 0b (binary)"
                    .to_string(),
            );
        }

        if self.missing_exp_val() {
            errors.push("the exponent is missing from the literal".to_string());
        }

        if self.second_exp() {
            errors.push("you appear to have attempted to scale more than once".to_string());
        }

        if self.exp_before_dot() {
            errors.push(
                "the literal contains an exponent scale before the decimal point".to_string(),
            );
        }

        if self.invalid_digit() {
            // TODO: Be a tiny bit more specific?
            errors.push("the literal contains characters outside its radix".to_string());
        }

        if self.leading_zeros() {
            errors.push("the literal begins with leading zero(s)".to_string());
        }

        if self.non_numeric() {
            // TODO: Maybe be a bit more specific?
            errors.push("the literal contains non-numeric digits".to_string())
        }

        if self.too_large() {
            errors.push(format!("the literal is too large (maximum {})", u64::MAX));
        }

        diag.emit(
            ErrorKind::Syntax,
            format!("Invalid {} literal: {}", lit_kind, errors.join_natural(",", "and")),
            span,
        );
    }
}

impl<'tok, 'ast, 'diag, 'src> Parser<'tok, 'ast, 'diag, 'src>
where
    'src: 'ast,
    'src: 'tok,
{
    // Could be a trait...
    pub(super) fn parse_int(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        match int::parse_bytes(tok.byte_src(self.src)) {
            Ok(val) => self.alloc_atom(ast::NodeKind::Int(val), tok),
            Err(errs) => {
                hint::cold_path();
                errs.0.emit("integer", tok.span, &mut self.diag);
                self.alloc_atom(ast::NodeKind::UnknownInt, tok)
            }
        }
    }

    pub(super) fn parse_radix_int(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        match radix_int::parse_bytes(tok.byte_src(self.src)) {
            Ok(val) => self.alloc_atom(ast::NodeKind::Int(val), tok),
            Err(errs) => {
                hint::cold_path();
                errs.0.emit("radix integer", tok.span, &mut self.diag);
                self.alloc_atom(ast::NodeKind::UnknownInt, tok)
            }
        }
    }

    pub(super) fn parse_float(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        match float::parse_bytes(tok.byte_src(self.src)) {
            Ok(val) => self.alloc_atom(ast::NodeKind::Float(val), tok),
            Err(errs) => {
                hint::cold_path();
                errs.0.emit("floating-point", tok.span, &mut self.diag);
                self.alloc_atom(ast::NodeKind::UnknownFloat, tok)
            }
        }
    }
}
