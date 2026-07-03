/*use crate::{ast, parser::RDParser, token::Token};

impl<'ast, 'diag, 'src> RDParser<'ast, 'diag, 'src> {
    pub fn try_parse_int(&mut self, tok: Token) -> &'ast ast::Node<'ast> {
        let span = tok.span;
        let src = tok.src(self.src);

        // TODO: Better integer parsing
        let Ok(val) = src.replace("_", "").parse() else {
            return self.alloc(ast::Node::failed(span));
        };

        self.alloc(ast::Node::new(ast::NodeKind::Int(val), span))
    }
}
*/
