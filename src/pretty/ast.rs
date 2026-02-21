use crate::{
    ast::{self, symbol::Symbol},
    pretty::{AnsiColor, Pretty},
};

impl<'a> Pretty for ast::Node<'a> {
    fn title(&self) -> String {
        use ast::NodeKind::*;
        match &self.kind {
            Ident(name) => format!("Identifier ({})", name),
            BinOp(..) => "Binary Operation".to_string(),
            KwBinOp(..) => "Binary Keyword Operation".to_string(),
            Comp(..) => "Comparison".to_string(),
            Range(..) => "Range".to_string(),
            UnOp(..) => "Unary Operation".to_string(),
            Int(val) => format!("Integer ({})", val),
            Bool(val) => format!("Boolean ({})", val),
            Decl(..) => "Declaration".to_string(),
            Unknown => "Unknown".to_string(),
        }
    }

    fn color(&self) -> Option<AnsiColor> {
        use ast::NodeKind::*;
        let col = match self.kind {
            // BinOp(..) | Comp(..) | Range(..) | UnOp(..) => AnsiColor::White,
            KwBinOp(..) => AnsiColor::Purple,
            Ident(_) => AnsiColor::Cyan,
            Decl(..) => AnsiColor::Green,
            Unknown => AnsiColor::Red,
            _ => return None,
        };
        Some(col)
    }

    fn children(&self) -> Vec<&dyn Pretty> {
        use ast::NodeKind::*;
        match self.kind {
            BinOp(left, ref op, right) => {
                vec![left, op as &dyn Pretty, right]
            }
            KwBinOp(left, ref op, right) => vec![left, op as &dyn Pretty, right],
            Comp(left, ref comp, right) => vec![left, comp as &dyn Pretty, right],
            Range(left, ref range, right) => {
                vec![left, range as &dyn Pretty, right]
            }
            UnOp(ref op, right) => vec![op as &dyn Pretty, right],
            Decl(vari, val) => vec![vari, val],
            _ => vec![],
        }
    }
}

impl<T: Symbol> Pretty for T {
    fn title(&self) -> String {
        format!("{} ({})", self.name(), self.as_str())
    }

    fn color(&self) -> Option<AnsiColor> {
        None
    }

    fn children(&self) -> Vec<&dyn Pretty> {
        vec![]
    }
}

impl<'a> Pretty for Vec<&'a ast::Node<'a>> {
    fn title(&self) -> String {
        "List".to_string()
    }

    fn color(&self) -> Option<AnsiColor> {
        None
    }

    fn children(&self) -> Vec<&dyn Pretty> {
        self.iter().map(|node| *node as &dyn Pretty).collect()
    }
}
