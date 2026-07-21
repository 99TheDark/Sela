use std::hint;

use crate::{
    ast,
    core::span::Span,
    diagnostics::ErrorKind,
    parser::Parser,
    token::{Token, kind::TokenKind, precedence::Precedence},
};

pub(super) enum DelimEnclosement {
    Enclosed,
    Unenclosed { start: Span },
}

impl<'tok, 'ast, 'diag, 'src> Parser<'tok, 'ast, 'diag, 'src>
where
    'src: 'ast,
    'src: 'tok,
{
    fn peek_relevant(&mut self, delim: TokenKind) -> Result<Token, ast::NodeRef<'ast>> {
        let tok = if delim == TokenKind::NewLine {
            let peeked = self.peek();
            self.eat_nls();
            peeked
        } else {
            self.eat_nls();
            self.peek()
        };

        if tok.is_eof() {
            hint::cold_path();
            Err(self.diag.fail(
                ErrorKind::Syntax,
                format!("Expected delimiter {:?}, but hit the end of the file", delim),
                self.eof_token.span,
                self.arena,
            ))
        } else {
            Ok(tok)
        }
    }

    #[inline(always)]
    fn skip_delim(&mut self, delim: TokenKind) {
        if delim != TokenKind::NewLine {
            self.advance();
        }
    }

    #[inline(always)]
    pub(super) fn parse_delimited<F: FnOnce(&'ast [ast::NodeRef<'ast>]) -> ast::NodeKind<'ast>>(
        &mut self,
        tok: Token,
        delim: TokenKind,
        end: TokenKind,
        enclosement: DelimEnclosement,
        constructor: F,
    ) -> ast::NodeRef<'ast> {
        'initial: {
            match self.peek_next() {
                first if first.is(end) => {}
                first if first.is(delim) => {
                    // hint::cold_path(); // Maybe? I mean it's valid syntax, just rare
                    self.skip_delim(delim);
                    match self.peek_relevant(delim)? {
                        tok if tok.is(end) => {}
                        _ => self.diag.emit(
                            ErrorKind::Syntax,
                            format!("Leading delimiter {:?} not allowed", tok.kind),
                            tok.span,
                        ),
                    }
                }
                _ => break 'initial,
            }

            let end = self.true_next();
            return self.alloc_node(constructor(&[]), tok.span.to(end.span));
        }

        let mut elems = Vec::<ast::NodeRef<'ast>>::new();
        loop {
            let elem = self.parse_expr(Precedence::None);
            self.recover_if_error(elem, |t| t == delim || t == end);
            elems.push(elem);

            match self.peek_relevant(delim)? {
                tok if tok.is(delim) => self.skip_delim(delim),
                tok if tok.is(end) => break,
                _ => {
                    hint::cold_path();
                    self.diag.emit(
                        ErrorKind::Syntax,
                        format!(
                            "Expected delimiter {:?}, found {:?} token instead",
                            delim, tok.kind
                        ),
                        tok.span,
                    );
                }
            }

            if self.peek_relevant(delim)?.is(end) {
                break;
            }
        }

        let span = match enclosement {
            DelimEnclosement::Enclosed => tok.span.to(self.true_next().span),

            // Guaranteed to have a last element, otherwise it would have returned early
            DelimEnclosement::Unenclosed { start } => start.to(elems.last().unwrap().span),
        };

        let elems = self.alloc(elems);
        self.alloc_node(constructor(&elems), span)
    }

    #[inline]
    fn parse_group<F: FnOnce(&'ast [ast::NodeRef<'ast>]) -> ast::NodeKind<'ast>>(
        &mut self,
        tok: Token,
        delim: TokenKind,
        end: TokenKind,
        constructor: F,
    ) -> ast::NodeRef<'ast> {
        self.parse_delimited(tok, delim, end, DelimEnclosement::Enclosed, constructor)
    }

    #[inline]
    pub(super) fn parse_parens(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        self.parse_group(tok, TokenKind::Comma, TokenKind::RParen, ast::NodeKind::Parens)
    }

    #[inline]
    pub(super) fn parse_bracks(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        self.parse_group(tok, TokenKind::Comma, TokenKind::RBrack, ast::NodeKind::Bracks)
    }

    #[inline]
    pub(super) fn parse_block(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        self.parse_group(tok, TokenKind::NewLine, TokenKind::RBrace, ast::NodeKind::Block)
    }
}
