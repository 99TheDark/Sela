pub mod basic;
pub mod binary;
pub mod control;
pub mod literal;
pub mod unary;

use crate::{
    ast,
    error::Diagnostics,
    token::{
        Token,
        kind::TokenKind,
        span::{Location, Span},
    },
};

pub struct Parser<'a, 'b> {
    src: &'b str,
    tokens: &'b [Token],
    idx: usize,
    diag: &'b mut Diagnostics<'a>,
    eof_token: Token,
}

impl<'a, 'b> Parser<'a, 'b> {
    pub fn new(src: &'a str, tokens: &'b [Token], diag: &'b mut Diagnostics<'a>) -> Self {
        let eof_loc = if let Some(last) = tokens.last() {
            last.span.end.next()
        } else {
            Location::ZERO
        };

        Self {
            src,
            tokens,
            idx: 0,
            diag,
            eof_token: Token::new(TokenKind::EOF, Span::single(eof_loc)),
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

    pub fn current(&self) -> Token {
        if self.idx < self.tokens.len() {
            self.tokens[self.idx]
        } else {
            self.eof_token
        }
    }

    pub fn parse(mut self) -> Vec<ast::Node> {
        let mut stmts = Vec::new();
        while self.idx < self.tokens.len() {
            stmts.push(self.parse_stmt());
        }
        stmts
    }
}
