use std::iter;

pub mod binary;
pub mod literal;
pub mod unary;

use crate::{
    ast,
    token::{Token, kind::TokenKind, span::Span},
};

pub struct Parser<'a, I>
where
    I: Iterator<Item = Token>,
{
    tokens: iter::Peekable<I>,
    src: &'a str,
}

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = Token>,
{
    pub fn new(tokens: I, src: &'a str) -> Self {
        Self {
            tokens: tokens.peekable(),
            src,
        }
    }

    pub fn parse_next(&mut self) -> Option<ast::Node> {
        if self.tokens.peek().is_none() {
            return None;
        }
        Some(self.parse_stmt())
    }

    pub fn parse_stmt(&mut self) -> ast::Node {
        let Some(tok) = self.tokens.peek() else {
            return ast::Node::EMPTY;
        };
        if tok.kind == TokenKind::Ident {
            let src = tok.src(self.src);
            let tok = self.tokens.next().unwrap();
            let span = tok.span;
            match src {
                "let" => {
                    let vari = self.parse_expr();
                    if self.tokens.next().map(|f| f.kind) != Some(TokenKind::Eq) {
                        panic!("nooo :(");
                    }
                    let end = self.parse_expr();
                    let esp = end.span;
                    ast::Node::new(
                        ast::NodeKind::Let(Box::new(vari), Box::new(end)),
                        span.to(esp),
                    )
                }
                _ => ast::Node::EMPTY,
            }
        } else {
            self.parse_expr()
        }
    }

    pub fn parse_expr(&mut self) -> ast::Node {
        self.parse_binop()
    }

    pub fn parse_primary(&mut self) -> ast::Node {
        let Some(tok) = self.tokens.next() else {
            return ast::Node::new(ast::NodeKind::Unknown, Span::ZERO);
        };

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
            _ => ast::Node::new(ast::NodeKind::Unknown, tok.span),
        }
    }
}

pub fn parse<I>(tokens: I, src: &str) -> impl Iterator<Item = ast::Node>
where
    I: Iterator<Item = Token>,
{
    let mut parser = Parser::new(tokens, src);
    iter::from_fn(move || parser.parse_next())
}
