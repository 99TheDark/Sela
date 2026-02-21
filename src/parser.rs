pub mod basic;
pub mod binary;
pub mod control;
pub mod decl;
pub mod literal;
pub mod unary;

use std::fmt;

use bumpalo::Bump;

use crate::{
    ast,
    error::Diagnostics,
    token::{Token, keyword::Keyword, kind::TokenKind, span::Span},
};

pub struct Parser<'ast, 'diag, 'src> {
    src: &'src str,
    tokens: &'src [Token],
    idx: usize,
    diag: &'diag mut Diagnostics<'src>,
    eof_token: Token,
    arena: &'ast Bump,
}

impl<'ast, 'diag, 'src> Parser<'ast, 'diag, 'src> {
    pub fn new(
        src: &'src str,
        tokens: &'src [Token],
        diag: &'diag mut Diagnostics<'src>,
        arena: &'ast Bump,
    ) -> Self {
        let eof_loc = tokens.last().map_or(0, |tok| tok.span.end);

        Self {
            src,
            tokens,
            idx: 0,
            diag,
            eof_token: Token::new(TokenKind::EOF, Span::single(eof_loc)),
            arena,
        }
    }

    pub fn advance(&mut self) {
        if self.idx < self.tokens.len() {
            self.idx += 1;
        }
    }

    pub fn next(&mut self) -> Token {
        if self.idx < self.tokens.len() {
            let tok = self.tokens[self.idx];
            self.idx += 1;
            tok
        } else {
            self.eof_token
        }
    }

    pub fn eat_until<F>(&mut self, cond: F)
    where
        F: Fn(Token) -> bool,
    {
        while self.idx < self.tokens.len() && cond(self.tokens[self.idx]) {
            self.idx += 1;
        }
    }

    pub fn eat_nls(&mut self) {
        self.eat_until(|tok| tok.is_nl());
    }

    pub fn eat_line(&mut self) {
        self.eat_until(|tok| !tok.is_nl());
    }

    pub fn current(&self) -> Token {
        if self.idx < self.tokens.len() {
            self.tokens[self.idx]
        } else {
            self.eof_token
        }
    }

    fn expect_raw<T, F>(
        &mut self,
        expected: T,
        is_expected: F,
        always_consume: bool,
        can_split: bool,
    ) -> Token
    where
        T: fmt::Debug,
        F: Fn(Token) -> bool,
    {
        if can_split {
            self.eat_nls();
        };
        let tok = self.current();

        if is_expected(tok) {
            self.advance();
            return tok;
        } else if always_consume {
            self.advance();
        }

        self.diag.emit(
            format!(
                "Expected {:?} token, found {:?} token instead",
                expected, tok.kind
            ),
            tok.span,
        );
        Token::new(TokenKind::EOF, tok.span)
    }

    pub fn expect(&mut self, expected: TokenKind) -> Token {
        self.expect_raw(expected, |tok| tok.kind == expected, true, true)
    }

    pub fn expect_peeking(&mut self, expected: TokenKind) -> Token {
        self.expect_raw(expected, |tok| tok.kind == expected, false, true)
    }

    pub fn expect_same_line(&mut self, expected: TokenKind) -> Token {
        self.expect_raw(expected, |tok| tok.kind == expected, true, false)
    }

    // When is this needed?
    pub fn expect_keyword(&mut self, expected: Keyword) -> Token {
        self.expect_raw(
            expected,
            |tok| Keyword::from_token(tok, self.src).is_keyword(),
            false,
            true,
        )
    }

    pub fn alloc<T>(&mut self, elem: T) -> &'ast T {
        self.arena.alloc(elem)
    }

    pub fn parse(mut self) -> Vec<&'ast ast::Node<'ast>> {
        let mut stmts = Vec::new();
        loop {
            if self.idx >= self.tokens.len() {
                break;
            }
            let stmt = self.parse_stmt();
            stmts.push(stmt);
        }
        stmts
    }
}
