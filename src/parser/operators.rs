use std::hint;

use crate::{
    ast::{self, binary::BinaryKind, unop::UnOpKind},
    parser::Parser,
    token::{Token, kind::TokenKind, precedence::Precedence},
};

impl<'tok, 'ast, 'diag, 'src> Parser<'tok, 'ast, 'diag, 'src>
where
    'src: 'ast,
    'src: 'tok,
{
    pub(super) fn parse_binary(
        &mut self,
        lhs: ast::NodeRef<'ast>,
        tok: Token,
        binary: BinaryKind,
    ) -> ast::NodeRef<'ast> {
        let rhs = self.parse_expr(tok.led_prec());
        binary.make_node(lhs, rhs, self.arena)
    }

    pub(super) fn parse_access(&mut self, lhs: ast::NodeRef<'ast>) -> ast::NodeRef<'ast> {
        self.eat_nls();
        'double_tuple_access: {
            if self.peek().is(TokenKind::Float) {
                hint::cold_path();
                let rhs = self.true_next();
                let Some(dot_pos) = rhs.byte_src(&self.src).iter().position(|&b| b == b'.') else {
                    break 'double_tuple_access;
                };

                let (lhs_span, rhs_span) = rhs.span.split_relative(dot_pos as u32);

                // TODO: Fix wasted token copy
                let first_access_kind = ast::NodeKind::Access {
                    parent: lhs,
                    child: self.parse_int(Token::new(TokenKind::Int, lhs_span)),
                };
                let first_access = self.alloc(ast::Node::new(first_access_kind, lhs_span));

                let second_access_kind = ast::NodeKind::Access {
                    parent: first_access,
                    child: self.parse_int(Token::new(TokenKind::Int, rhs_span)),
                };
                let second_access = self.alloc(ast::Node::new(second_access_kind, rhs_span));

                return second_access;
            }
        }

        let rhs = self.parse_expr(Precedence::Prop);
        let kind = ast::NodeKind::Access { parent: lhs, child: rhs };
        self.alloc(ast::Node::new(kind, lhs.span.to(rhs.span)))
    }

    pub(super) fn parse_unop(&mut self, tok: Token, op: UnOpKind) -> ast::NodeRef<'ast> {
        let rhs = self.parse_expr(tok.nud_prec());
        let kind = ast::NodeKind::UnOp { op, rhs };
        self.alloc(ast::Node::new(kind, tok.span.to(rhs.span)))
        // let node = self.reserve::<ast::Node>();
        // let rhs = self.parse_expr(tok.nud_prec());
        // let kind = ast::NodeKind::UnOp { op, rhs };
        // self.fill(node, ast::Node::new(kind, tok.span.to(rhs.span)))
    }
}
