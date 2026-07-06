use std::{borrow::Borrow, str::FromStr};

use crate::{
    ast,
    error::{ErrorKind, natural::Natural},
    parser::Parser,
    token::Token,
};

use arrayvec::ArrayVec;
use regex::Regex;
use regex_macro::regex;

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
                    issues.join_natural("or")
                ),
                tok.span,
            );
            self.alloc(ast::Node::new(failure, tok.span))
        }
    }

    pub(super) fn parse_int(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        self.parse_number(
            tok,
            "integer",
            ArrayVec::from([
                (regex!(r"__"), "consecutively"),
                (regex!(r"^_"), "at the start of a number"),
                (regex!(r"_$"), "at the end of a number"),
            ]),
            ast::NodeKind::Int,
            ast::NodeKind::UnknownInt,
        )
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
