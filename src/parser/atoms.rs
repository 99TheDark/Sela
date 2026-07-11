use crate::{ast, parser::Parser, token::Token};

impl<'tok, 'ast, 'diag, 'src> Parser<'tok, 'ast, 'diag, 'src>
where
    'src: 'ast,
    'src: 'tok,
{
    #[inline(always)]
    pub(super) fn alloc_atom(
        &mut self,
        kind: ast::NodeKind<'ast>,
        tok: Token,
    ) -> ast::NodeRef<'ast> {
        self.alloc(ast::Node::new(kind, tok.span))
    }

    pub(super) fn parse_bool(&mut self, tok: Token, val: bool) -> ast::NodeRef<'ast> {
        self.alloc_atom(ast::NodeKind::Bool(val), tok)
    }

    pub(super) fn parse_ident(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        self.alloc_atom(ast::NodeKind::Ident(tok.src(self.src)), tok)
    }
}
