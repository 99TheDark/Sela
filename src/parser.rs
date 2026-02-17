use std::iter;

use crate::{
    ast::{self, symbol::BinaryKind, unop::UnOpKind},
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

    pub fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    pub fn parse_next(&mut self) -> Option<ast::Node> {
        if self.tokens.peek().is_none() {
            return None;
        }
        Some(self.parse_expr())
    }

    pub fn parse_expr(&mut self) -> ast::Node {
        self.parse_binop()
    }

    pub fn parse_binop(&mut self) -> ast::Node {
        self.parse_binop_pratt(0)
    }

    pub fn parse_binop_pratt(&mut self, min_prec: u8) -> ast::Node {
        let mut left = self.parse_unop();
        while let Some(token) = self.tokens.peek() {
            let Some(op) = BinaryKind::from_token(*token, self.src) else {
                break;
            };

            let right_prec = op.precedence();
            if right_prec < min_prec {
                break;
            }
            self.next();

            let right = self.parse_binop_pratt(right_prec);
            left = op.make_node(left, right);
        }
        left
    }

    pub fn parse_unop(&mut self) -> ast::Node {
        let Some(token) = self.tokens.peek() else {
            return ast::Node::ZERO;
        };

        let span = token.span;
        let sym = match token.kind {
            TokenKind::Dash => UnOpKind::Neg,
            TokenKind::Not => UnOpKind::Not,
            TokenKind::And => UnOpKind::Ref,
            TokenKind::Star => UnOpKind::Deref,
            _ => return self.parse_primary(),
        };

        self.tokens.next();
        let operand = Box::new(self.parse_expr());
        return ast::Node::new(ast::NodeKind::UnOp(sym, operand), span);
    }

    pub fn parse_primary(&mut self) -> ast::Node {
        if let Some(token) = self.tokens.next() {
            let span = token.span;
            ast::Node::new(ast::NodeKind::Unknown, span)
        } else {
            ast::Node::new(ast::NodeKind::Unknown, Span::ZERO)
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
