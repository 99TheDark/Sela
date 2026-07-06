use std::{borrow::Borrow, str::FromStr};

use crate::{
    ast,
    core::span::Span,
    error::{Diagnostics, ErrorKind, natural::Natural},
    parser::Parser,
    token::Token,
};

use arrayvec::ArrayVec;
use modular_bitfield::bitfield;
use regex::Regex;
use regex_macro::regex;

#[bitfield(filled = false)]
struct IntParsingError {
    under_consecutive: bool,
    under_start: bool, // Is this even possible based on the lexer?
    under_end: bool,
    under_post_radix: bool,
    unsupported_radix: bool,
    too_large: bool,
    non_numeric: bool,
}

impl IntParsingError {
    fn emit(&self, span: Span, diag: &mut Diagnostics) {
        // Should this be SmallVec since usually you have 0-2 errors?
        let mut under_errs = ArrayVec::<&str, 4>::new();
        if self.under_consecutive() {
            under_errs.push("consecutively");
        }
        if self.under_start() {
            under_errs.push("at the start of a number");
        }
        if self.under_end() {
            under_errs.push("at the end of a number");
        }
        if self.under_post_radix() {
            under_errs.push("after the radix symbol");
        }

        let mut errors = ArrayVec::<String, 4>::new();
        errors.push(format!(
            "underscores may not appear {}",
            under_errs.join_natural(",", "or")
        ));

        if self.unsupported_radix() {
            // TODO: Be more helpful than this
            errors.push(
                "the literal is written in a radix other than \
                 0x (hexadecimal), 0o (octal), or 0b (binary)"
                    .to_string(),
            );
        }

        if self.too_large() {
            errors.push(format!("the literal is too large (maximum {})", i64::MAX));
        }

        if self.non_numeric() {
            // TODO: Also be more helpful
            errors.push(
                "The literal contains non-numeric digits or \
                 characters outside its radix"
                    .to_string(),
            )
        }

        diag.emit(
            ErrorKind::Syntax,
            format!("Invalid integer literal: {}", errors.join_natural(";", "and")),
            span,
        );
    }
}

#[bitfield(filled = false)]
struct FloatParsingError {
    under_consecutive: bool,
    under_start: bool,
    under_end: bool,
    under_pre_dot: bool,
    under_post_dot: bool,
    under_pre_exp: bool,
    under_post_exp: bool,
    under_pre_exp_sgn: bool,
    under_post_expr_sgn: bool,
    too_big: bool,
    non_numeric: bool,
}

// Assumes the src string is not empty (ensured by the lexer)
fn parse_int(src: &str) -> Result<i64, IntParsingError> {
    let mut errors = IntParsingError::new();
    let mut res = 0i64;
    let mut last_was_under = false;
    for byte in src.bytes() {
        match byte {
            b'_' if last_was_under => {
                last_was_under = true;
                errors.set_under_consecutive(true);
            }
            b'_' => {
                last_was_under = true;
            }
            b'0'..=b'9' => {
                last_was_under = false;
                let digit = byte - b'0';
                let new_res =
                    res.checked_mul(10).and_then(|res| res.checked_add(digit as i64));

                if let Some(new_res) = new_res {
                    res = new_res;
                } else {
                    res = i64::MAX;
                    errors.set_too_large(true);
                }
            }
            _ => {
                last_was_under = false;
                errors.set_non_numeric(true)
            }
        }
    }

    if errors.bytes == [0] { Ok(res) } else { Err(errors) }
}

impl<'tok, 'ast, 'diag, 'src> Parser<'tok, 'ast, 'diag, 'src>
where
    'src: 'ast,
    'src: 'tok,
{
    #[inline(always)]
    pub(super) fn parse_number<
        R: std::ops::Deref<Target = Regex>,
        T: FromStr,
        F: FnOnce(T) -> ast::NodeKind<'ast>,
        const N: usize,
    >(
        &mut self,
        tok: Token,
        name: &'static str,
        underscore_checks: ArrayVec<(&R, &'static str), N>, // Maybe make ArrayVec<Check>?
        constructor: F,
        failure: ast::NodeKind<'ast>,
    ) -> ast::NodeRef<'ast> {
        let src = tok.src(self.src);

        let mut issues = ArrayVec::<&str, N>::new();
        for (regex, issue) in underscore_checks {
            if regex.borrow().is_match(src) {
                issues.push(issue);
            }
        }

        if issues.is_empty() {
            let Ok(val) = src.replace("_", "").parse() else {
                return self.alloc(ast::Node::failed(tok.span));
            };

            self.alloc(ast::Node::new(constructor(val), tok.span))
        } else {
            // TODO: Make a method
            self.diag.emit(
                ErrorKind::Syntax,
                format!(
                    "Invalid {} literal: underscores may not appear {}",
                    name,
                    issues.join_natural(",", "or")
                ),
                tok.span,
            );
            self.alloc(ast::Node::new(failure, tok.span))
        }
    }

    pub(super) fn parse_int(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        /*self.parse_number(
            tok,
            "integer",
            ArrayVec::from([
                (regex!(r"__"), "consecutively"),
                (regex!(r"^_"), "at the start of a number"),
                (regex!(r"_$"), "at the end of a number"),
            ]),
            ast::NodeKind::Int,
            ast::NodeKind::UnknownInt,
        )*/

        match parse_int(tok.src(self.src)) {
            Ok(int) => self.alloc(ast::Node::new(ast::NodeKind::Int(int), tok.span)),
            Err(errors) => {
                errors.emit(tok.span, &mut self.diag);
                self.alloc(ast::Node::new(ast::NodeKind::UnknownInt, tok.span))
            }
        }
    }

    pub(super) fn parse_float(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        self.parse_number(
            tok,
            "floating-point",
            ArrayVec::from([
                (regex!(r"__"), "consecutively"),
                (regex!(r"^_"), "at the start of a number"),
                (regex!(r"_$"), "at the end of a number"),
                (regex!(r"_\."), "before the decimal point"),
                (regex!(r"\._"), "after the decimal point"),
                (regex!(r"_[eE]"), "before an exponent"),
                (regex!(r"[eE]_"), "after an exponent"),
                (regex!(r"_[+-]"), "before an exponential sign"),
                (regex!(r"[+-]_"), "after an exponential sign"),
            ]),
            ast::NodeKind::Float,
            ast::NodeKind::UnknownFloat,
        )
    }
}
