pub mod binary;
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

pub struct Parser<'a> {
    src: &'a str,
    tokens: &'a [Token],
    idx: usize,
    diag: &'a mut Diagnostics,
    eof_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str, tokens: &'a [Token], diag: &'a mut Diagnostics) -> Self {
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

    pub fn parse_stmt(&mut self) -> ast::Node {
        self.parse_expr()
    }

    pub fn parse_expr(&mut self) -> ast::Node {
        self.parse_binop()
    }

    pub fn parse_primary(&mut self) -> ast::Node {
        let tok = self.next();

        match tok.kind {
            TokenKind::Ident => {
                let span = tok.span;
                let src = tok.src(self.src);
                let kind = match src {
                    "true" => ast::NodeKind::Bool(true),
                    "false" => ast::NodeKind::Bool(false),
                    _ => ast::NodeKind::Ident(src.to_string()), // TODO: temporary str store
                };
                ast::Node::new(kind, span)
            }
            TokenKind::Int => self.try_parse_int(tok),
            _ => ast::Node::failed(tok.span),
        }
    }

    pub fn parse(&mut self) -> Vec<ast::Node> {
        let mut stmts = Vec::new();
        while self.idx < self.tokens.len() {
            stmts.push(self.parse_stmt());
        }
        stmts
    }
}
