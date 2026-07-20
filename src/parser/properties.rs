use std::hint;

use crate::{
    ast,
    parser::Parser,
    token::{Token, kind::TokenKind, precedence::Precedence},
};

impl<'tok, 'ast, 'diag, 'src> Parser<'tok, 'ast, 'diag, 'src>
where
    'src: 'ast,
    'src: 'tok,
{
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

                // TODO: Fix wasted token copy - use the direct int::parse_bytes or something similar
                let first_access_kind = ast::NodeKind::Access {
                    parent: lhs,
                    child: self.parse_int(Token::new(TokenKind::Int, lhs_span)),
                };
                let first_access = self.alloc_node(first_access_kind, lhs_span);

                let second_access_kind = ast::NodeKind::Access {
                    parent: first_access,
                    child: self.parse_int(Token::new(TokenKind::Int, rhs_span)),
                };
                let second_access = self.alloc_node(second_access_kind, rhs_span);

                return second_access;
            }
        }

        let rhs = self.parse_expr(Precedence::Prop)?;
        let kind = ast::NodeKind::Access { parent: lhs, child: rhs };
        self.alloc_node(kind, lhs.span.to(rhs.span))
    }

    pub(super) fn parse_invoc(
        &mut self,
        lhs: ast::NodeRef<'ast>,
        tok: Token,
    ) -> ast::NodeRef<'ast> {
        let args = self.parse_parens(tok);
        self.alloc_node(ast::NodeKind::Invoc { callee: lhs, args }, lhs.span.to(args.span))
    }

    pub(super) fn parse_select(
        &mut self,
        lhs: ast::NodeRef<'ast>,
        tok: Token,
    ) -> ast::NodeRef<'ast> {
        let disc = self.parse_bracks(tok);
        self.alloc_node(ast::NodeKind::Select { src: lhs, disc }, lhs.span.to(disc.span))
    }
}
