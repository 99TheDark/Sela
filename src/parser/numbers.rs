// Parse floats as ints first? make sure the decimal gets chopped off at 17 digits
// 3.141592653589793238462643383279502884197169399 ->
// 314159265358979323 * 10^-17
//
// Maybe this? (Could make float_pow a table as there is a set number of lengths)
// dec_len = min(byte_len(dec_part), 17) <- this needs to cap based on byte_len(int) to prevent overflow, but also even more so if the int part is extra long
// unified = parse_int(int_part) * int_pow(10, dec_len) + parse_int(dec_part[0..dec_len])
// result = float(unified) * float_pow(0.1, dec_len) * float_pow(10.0, exp_part)

mod float;
mod int;
mod radix_int;

use std::{iter::zip, ops};

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
#[derive(PartialEq, Eq)]
struct NumberParsingError {
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
    pub leading_zeros: bool,
    pub invalid_digit: bool,
    pub non_numeric: bool,
    pub too_large: bool,
}

trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl IsEmpty for NumberParsingError {
    fn is_empty(&self) -> bool {
        *self == Self::new()
    }
}

trait ErrorSet {
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

impl NumberParsingError {
    fn emit(&self, span: Span, diag: &mut Diagnostics) {
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
        let mut errors = ArrayVec::<String, 3>::new();
        if !under_errs.is_empty() {
            errors.push(format!(
                "underscores may not appear {}",
                under_errs.join_natural(",", "or")
            ));
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
            format!("Invalid integer literal: {}", errors.join_natural(",", "and")),
            span,
        );
    }
}

impl<'tok, 'ast, 'diag, 'src> Parser<'tok, 'ast, 'diag, 'src>
where
    'src: 'ast,
    'src: 'tok,
{
    pub(super) fn parse_int(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        match int::parse_bytes(tok.byte_src(self.src)) {
            Ok(val) => self.alloc_atom(ast::NodeKind::Int(val), tok),
            Err(errs) => {
                errs.0.emit(tok.span, &mut self.diag);
                self.alloc_atom(ast::NodeKind::UnknownInt, tok)
            }
        }
    }

    pub(super) fn parse_radix_int(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        match radix_int::parse_bytes(tok.byte_src(self.src)) {
            Ok(val) => self.alloc_atom(ast::NodeKind::Int(val), tok),
            Err(errs) => {
                errs.0.emit(tok.span, &mut self.diag);
                self.alloc_atom(ast::NodeKind::UnknownInt, tok)
            }
        }
    }

    pub(super) fn parse_float(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        todo!()
    }
}
