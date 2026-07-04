use crate::ast::NodeRef;

#[derive(Debug, Clone)]
pub enum Fragment<'a> {
    String(String),
    Expr(NodeRef<'a>),
}
