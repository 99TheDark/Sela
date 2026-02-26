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
        use Keyword::*;
        match Keyword::from_token(self.current(), self.src) {
            If => self.parse_if_else(),
            Loop => self.parse_loop(),
            While => self.parse_while_loop(),
            For => self.parse_for_loop(),
            Let => self.parse_decl(),
            True | False | NotReserved => self.parse_access(),
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
            /*TokenKind::String => {
                // TODO: Parse escape strings
                let src = tok.src(&self.src);
                if src.chars().nth(0) != Some('\"') {
                    self.diag.fail(
                        format!("Unexpected {:?} token", tok.kind),
                        span,
                        &self.arena,
                    )
                } else {
                    println!("str: '{}'", src);
                    self.alloc(ast::Node::failed(tok.span))
                }
            }*/
            TokenKind::LParen => {
                let expr = self.parse_expr();
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
