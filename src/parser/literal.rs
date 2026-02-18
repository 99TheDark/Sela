use crate::{ast, parser::Parser, token::Token};

impl<'a, I> Parser<'a, I>
where
    I: Iterator<Item = Token>,
{
    pub fn try_parse_int(&self, tok: Token) -> ast::Node {
        let span = tok.span;
        let src = tok.str_value(self.src);
        let val = src.replace("_", "").parse().unwrap();

        ast::Node::new(ast::NodeKind::Int(val), span)
    }
}
