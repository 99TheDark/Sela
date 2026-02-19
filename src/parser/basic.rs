use crate::{
    ast,
    parser::Parser,
    token::{keyword::Keyword, kind::TokenKind},
};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn parse_stmt(&mut self) -> ast::Node {
        self.parse_expr()
    }

    pub fn parse_expr(&mut self) -> ast::Node {
        let kw = Keyword::from_token(self.current(), self.src);
        if kw == NotReserved {
            return self.parse_binop();
        }

        let tok = self.next();

        use Keyword::*;
        match kw {
            If => self.parse_if_else(),
            _ => self.diag.fail(
                format!("Unexpected reserved keyword '{}'", tok.src(self.src)),
                tok.span,
            ),
        }
    }

    pub fn parse_block(&mut self) -> ast::Node {
        todo!()
    }

    pub fn parse_primary(&mut self) -> ast::Node {
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
                ast::Node::new(kind, span)
            }
            TokenKind::Int => self.try_parse_int(tok),
            _ => self
                .diag
                .fail(format!("Unexpected {:?} token", tok.kind), span),
        }
    }
}
