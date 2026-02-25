use crate::{
    ast,
    parser::Parser,
    token::{keyword::Keyword, kind::TokenKind},
};

impl<'ast, 'diag, 'src> Parser<'ast, 'diag, 'src> {
    pub fn parse_stmt(&mut self) -> &'ast ast::Node<'ast> {
        self.parse_expr()
    }

    pub fn parse_expr(&mut self) -> &'ast ast::Node<'ast> {
        let kw = Keyword::from_token(self.current(), self.src);
        if kw == NotReserved {
            return self.parse_binop();
        }

        use Keyword::*;
        match kw {
            If => self.parse_if_else(),
            Let => self.parse_decl(),
            True => {
                let tok = self.next();
                self.alloc(ast::Node::new(ast::NodeKind::Bool(true), tok.span))
            }
            False => {
                let tok = self.next();
                self.alloc(ast::Node::new(ast::NodeKind::Bool(false), tok.span))
            }
            _ => {
                let tok = self.next();
                self.diag.fail(
                    format!("Unexpected reserved keyword '{}'", tok.src(self.src)),
                    tok.span,
                    self.arena,
                )
            }
        }
    }

    pub fn parse_block(&mut self) -> &'ast ast::Node<'ast> {
        let start = self.expect(TokenKind::LBrace).span;
        let body = self.parse_stmts(|tok| tok.kind == TokenKind::RBrace);
        let end = self.expect(TokenKind::RBrace).span;

        self.alloc(ast::Node::new(ast::NodeKind::Block(body), start.to(end)))
    }

    pub fn parse_primary(&mut self) -> &'ast ast::Node<'ast> {
        let tok = self.next();
        let span = tok.span;

        let src = tok.src(self.src);

        match tok.kind {
            TokenKind::Ident => {
                let kind = match src {
                    "true" => ast::NodeKind::Bool(true),
                    "false" => ast::NodeKind::Bool(false),
                    _ => ast::NodeKind::Ident(src.to_string()), // TODO: temporary str store
                };
                self.alloc(ast::Node::new(kind, span))
            }
            TokenKind::Int => self.try_parse_int(tok),
            TokenKind::LParen => {
                //self.eat_nls();
                let expr = self.parse_expr();
                //self.eat_nls();
                self.expect(TokenKind::RParen);
                expr
            }
            _ => self.diag.fail(
                format!("Unexpected {:?} token", tok.kind),
                span,
                &self.arena,
            ),
        }
    }
}
