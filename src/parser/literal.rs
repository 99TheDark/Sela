use crate::{ast, parser::Parser, token::Token};

impl<'a, 'b> Parser<'a, 'b> {
    pub fn try_parse_int(&self, tok: Token) -> ast::Node {
        let span = tok.span;
        let src = tok.src(self.src);

        // TODO: Better integer parsing
        let Ok(val) = src.replace("_", "").parse() else {
            return ast::Node::failed(span);
        };

        ast::Node::new(ast::NodeKind::Int(val), span)
    }
}
