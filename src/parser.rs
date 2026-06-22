pub mod basic;
pub mod binary;
pub mod control;
pub mod import;
pub mod literal;
pub mod set;
pub mod unary;

use bumpalo::Bump;

use crate::{
    ast,
    core::span::Span,
    error::{Diagnostics, ErrorKind},
    token::{Token, keyword::Keyword, kind::TokenKind},
};

pub struct Parser<'ast, 'diag, 'src> {
    src: &'src str,
    tokens: &'src [Token],
    idx: usize,
    in_recovery: bool,
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
            in_recovery: false,
            diag,
            eof_token: Token::new(TokenKind::EOF, Span::single(eof_loc)),
            arena,
        }
    }

    pub fn advance(&mut self) {
        // Is this even needed?
        if self.idx < self.tokens.len() {
            self.idx += 1;
        }
    }

    pub fn next(&mut self) -> Token {
        self.eat_nls();
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
        if self.idx < self.tokens.len() { self.tokens[self.idx] } else { self.eof_token }
    }

    pub fn peek(&self) -> Token {
        if self.idx + 1 < self.tokens.len() {
            self.tokens[self.idx + 1]
        } else {
            self.eof_token
        }
    }

    pub fn expect(&mut self, expected: TokenKind) -> Token {
        let tok = self.next();
        if tok.kind != expected {
            self.diag.emit(
                ErrorKind::Syntax,
                format!(
                    "Expected {:?} token, found {:?} token instead",
                    expected, tok.kind
                ),
                tok.span,
            );
        }
        tok
    }

    pub fn expect_keyword(&mut self, expected: Keyword) -> Token {
        let tok = self.next();

        let kw = Keyword::from_token(tok, &self.src);
        if kw != expected {
            if kw == Keyword::NotReserved {
                self.diag.emit(
                    ErrorKind::Syntax,
                    format!(
                        "Expected {:?} keyword, found {:?} token instead",
                        expected, tok.kind
                    ),
                    tok.span,
                );
            } else {
                self.diag.emit(
                    ErrorKind::Syntax,
                    format!(
                        "Expected {:?} keyword, found {:?} keyword instead",
                        expected, kw
                    ),
                    tok.span,
                );
            }
        }

        tok
    }

    pub fn at_keyword(&self, kw: Keyword) -> bool {
        Keyword::from_token(self.current(), self.src) == kw
    }

    pub fn at_and_eat(&mut self, kind: TokenKind) -> bool {
        if self.current().kind == kind {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn alloc<T>(&mut self, elem: T) -> &'ast T {
        self.arena.alloc(elem)
    }

    pub fn parse_stmts<F>(&mut self, should_exit: F) -> Vec<&'ast ast::Node<'ast>>
    where
        F: Fn(Token) -> bool,
    {
        let mut stmts = Vec::new();
        loop {
            self.eat_nls();

            if self.current().is_eof() || should_exit(self.current()) {
                break;
            }

            let stmt = self.parse_stmt();
            stmts.push(stmt);
        }
        stmts
    }

    pub fn parse(mut self) -> Vec<&'ast ast::Node<'ast>> {
        self.parse_stmts(|_| false)
    }
}
