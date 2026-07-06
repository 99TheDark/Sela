use crate::{ast, parser::Parser, token::Token};

impl<'tok, 'ast, 'diag, 'src> Parser<'tok, 'ast, 'diag, 'src>
where
    'src: 'ast,
    'src: 'tok,
{
    pub(super) fn parse_bool(&mut self, tok: Token, val: bool) -> ast::NodeRef<'ast> {
        let kind = ast::NodeKind::Bool(val);
        self.alloc(ast::Node::new(kind, tok.span))
    }

    pub(super) fn parse_ident(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let kind = ast::NodeKind::Ident(tok.src(self.src));
        self.alloc(ast::Node::new(kind, tok.span))
    }
}
