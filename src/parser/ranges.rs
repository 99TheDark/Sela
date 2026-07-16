use crate::{
    ast::{self, range::RangeKind},
    diagnostics::ErrorKind,
    parser::Parser,
    token::{Token, precedence::Precedence},
};

impl<'tok, 'ast, 'diag, 'src> Parser<'tok, 'ast, 'diag, 'src>
where
    'src: 'ast,
    'src: 'tok,
{
    // TODO: Combine shared logic of nud + led
    pub(super) fn parse_nud_range(&mut self, tok: Token, range: RangeKind) -> ast::NodeRef<'ast> {
        let (to, span) = if self.peek().led_prec() != Precedence::None {
            let rhs = self.parse_expr(Precedence::Range);
            (Some(rhs), tok.span.to(rhs.span))
        } else {
            (None, tok.span)
        };

        let kind = match (to.is_some(), range) {
            // ..a
            (true, RangeKind::Full) => {
                self.diag.emit(
                    ErrorKind::Syntax,
                    format!(
                        "Half ranges up to a value must be strictly \
                         inclusive (..=) or exclusive (..<)"
                    ),
                    span,
                );
                ast::NodeKind::UnknownRange { from: None, range, to }
            }

            // ..<a, ..=a, ..
            (true, RangeKind::Excl | RangeKind::Incl) | (false, RangeKind::Full) => {
                ast::NodeKind::Range { from: None, range, to }
            }

            // ..<
            (false, RangeKind::Excl) => {
                self.diag.emit(
                    ErrorKind::Syntax,
                    format!("An exclusive range must go up to a value"),
                    span,
                );
                ast::NodeKind::UnknownRange { from: None, range, to }
            }

            // ..=
            (false, RangeKind::Incl) => {
                self.diag.emit(
                    ErrorKind::Syntax,
                    format!("An inclusive range must go up to a value"),
                    span,
                );
                ast::NodeKind::UnknownRange { from: None, range, to }
            }
        };

        self.alloc(ast::Node::new(kind, span))
    }

    pub(super) fn parse_led_range(
        &mut self,
        lhs: ast::NodeRef<'ast>,
        range: RangeKind,
    ) -> ast::NodeRef<'ast> {
        let from = Some(lhs);

        let (to, span) = if self.peek().led_prec() == Precedence::None {
            let rhs = self.parse_expr(Precedence::Range);
            (Some(rhs), lhs.span.to(rhs.span))
        } else {
            (None, lhs.span)
        };

        let kind = match (to.is_some(), range) {
            // a..b
            (true, RangeKind::Full) => {
                self.diag.emit(
                    ErrorKind::Syntax,
                    format!(
                        "Ranges between two values must be strictly \
                         inclusive (..=) or exclusive (..<)"
                    ),
                    span,
                );
                ast::NodeKind::UnknownRange { from, range, to }
            }

            // a..<b, a..=b, a..
            (true, RangeKind::Excl) | (true, RangeKind::Incl) | (false, RangeKind::Full) => {
                ast::NodeKind::Range { from, range, to }
            }

            // a..<
            (false, RangeKind::Excl) => {
                self.diag.emit(
                    ErrorKind::Syntax,
                    format!("An exclusive range must go up to a value"),
                    span,
                );
                ast::NodeKind::UnknownRange { from, range, to }
            }

            // b..=
            (false, RangeKind::Incl) => {
                self.diag.emit(
                    ErrorKind::Syntax,
                    format!("An inclusive range must go up to a value"),
                    span,
                );
                ast::NodeKind::UnknownRange { from, range, to }
            }
        };

        self.alloc(ast::Node::new(kind, span))
    }
}
