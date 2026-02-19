pub mod basic;
pub mod binary;
pub mod control;
pub mod decl;
pub mod literal;
pub mod unary;

use crate::{
    ast,
    error::Diagnostics,
    token::{
        Token,
        keyword::Keyword,
        kind::TokenKind,
        span::{Location, Span},
    },
};

pub struct Parser<'a, 'b> {
    src: &'a str,
    tokens: &'a [Token],
    idx: usize,
    diag: &'a mut Diagnostics<'b>,
    eof_token: Token,
}

impl<'a, 'b> Parser<'a, 'b> {
    pub fn new(src: &'a str, tokens: &'a [Token], diag: &'a mut Diagnostics<'b>) -> Self {
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

    pub fn next_broken(&mut self) -> Token {
        while self.idx < self.tokens.len() {
            let tok = self.tokens[self.idx];
            self.idx += 1;
            if !tok.is_nl() {
                return tok;
            }
        }
        self.eof_token
    }

    pub fn current(&self) -> Token {
        if self.idx < self.tokens.len() {
            self.tokens[self.idx]
        } else {
            self.eof_token
        }
    }

    fn expect(&mut self, expected: TokenKind, can_split: bool) -> Token {
        let tok = if can_split {
            self.next_broken()
        } else {
            self.next()
        };
        if tok.kind == expected {
            tok
        } else {
            self.diag.emit(
                format!(
                    "Expected {:?} token, found {:?} token instead",
                    expected, tok.kind
                ),
                tok.span,
            );
            Token::new(TokenKind::EOF, tok.span)
        }
    }

    pub fn expect_keyword(&mut self, expected: Keyword) -> Token {
        let tok = self.next();
        if Keyword::from_token(tok, self.src).is_keyword() {
            tok
        } else {
            self.diag.emit(
                format!(
                    "Expected {:?} token, found {:?} token instead",
                    expected, tok.kind
                ),
                tok.span,
            );
            Token::new(TokenKind::EOF, tok.span)
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
