pub mod atoms;
pub mod control;
pub mod groups;
pub mod numbers;
pub mod operators;
pub mod properties;
pub mod ranges;
pub mod text;
pub mod variables;

use bumpalo::Bump;

use crate::{
    ast::{self, binary::BinaryKind, range::RangeKind, unop::UnOpKind},
    core::span::Span,
    error::{Diagnostics, ErrorKind},
    token::{Token, kind::TokenKind, precedence::Precedence},
};

pub struct Parser<'tok, 'ast, 'diag, 'src> {
    src: &'src str,
    tokens: &'tok [Token],
    idx: usize,
    diag: &'diag mut Diagnostics<'src>,
    arena: &'ast Bump,
    eof_token: Token,
}

pub struct ParserError;
pub type PResult<'ast> = Result<ast::NodeRef<'ast>, ParserError>;

impl<'tok, 'ast, 'diag, 'src> Parser<'tok, 'ast, 'diag, 'src>
where
    'src: 'ast,
    'src: 'tok,
{
    pub fn new(
        src: &'src str,
        tokens: &'tok [Token],
        diag: &'diag mut Diagnostics<'src>,
        arena: &'ast Bump,
    ) -> Self {
        let eof_loc = tokens.last().map_or(0, |tok| tok.span.end);
        let eof_token = Token::new(TokenKind::EOF, Span::single(eof_loc));

        Self { src, tokens, idx: 0, diag, arena, eof_token }
    }

    pub fn alloc<T>(&mut self, elem: T) -> &'ast T {
        self.arena.alloc(elem)
    }

    #[inline]
    pub fn eat_until<F>(&mut self, cond: F)
    where
        F: Fn(Token) -> bool,
    {
        while self.idx < self.tokens.len() && cond(self.tokens[self.idx]) {
            self.idx += 1;
        }
    }

    // There must be a way to make this more performant...
    pub fn eat_nls(&mut self) {
        self.eat_until(|tok| tok.is_nl());
    }

    pub fn eat_line(&mut self) {
        self.eat_until(|tok| !tok.is_nl());
    }

    pub fn peek(&self) -> Token {
        // Is this check even needed? Could we make this something else?
        if self.idx < self.tokens.len() { self.tokens[self.idx] } else { self.eof_token }
    }

    pub fn next(&mut self) -> Token {
        self.eat_nls();
        self.true_next()
    }

    pub fn true_next(&mut self) -> Token {
        if self.idx < self.tokens.len() {
            let tok = self.tokens[self.idx];
            self.idx += 1;
            tok
        } else {
            self.eof_token
        }
    }

    pub fn expect(&mut self, expected: TokenKind) -> Token {
        let tok = self.next();
        if tok.kind != expected {
            self.diag.emit(
                ErrorKind::Syntax,
                format!(
                    "Expected {:?} token, found {:?} token instead",
                    expected, tok.kind
                ),
                tok.span,
            );
        }
        tok
    }

    pub fn peek_prec(&self) -> Precedence {
        self.tokens.get(self.idx).map_or(Precedence::None, |tok| tok.led_prec())
    }

    pub fn parse_nud(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        use TokenKind::*;
        match tok.kind {
            Ident => self.parse_ident(tok),
            Int => self.parse_int(tok),
            Float => self.parse_float(tok),
            Char => self.parse_char(tok),
            String => self.parse_string(tok),
            Annot => todo!(),
            Dash => self.parse_unop(tok, UnOpKind::Neg),
            Star => self.parse_unop(tok, UnOpKind::Deref),
            Amp => self.parse_unop(tok, UnOpKind::Ref),
            Not => self.parse_unop(tok, UnOpKind::Not),
            // Maybe make these use both parse_stmts() and parse_delimited()?
            LParen => self.parse_delimited(
                tok,
                TokenKind::Comma,
                TokenKind::RParen,
                ast::NodeKind::Parens,
            ),
            LBrack => todo!(),
            LBrace => self.parse_block(tok),
            DotDot => self.parse_nud_range(tok, RangeKind::Full),
            DotDotLt => self.parse_nud_range(tok, RangeKind::Excl),
            DotDotEq => self.parse_nud_range(tok, RangeKind::Incl),
            Tick => todo!(),
            Let => self.parse_decl(tok),
            True => self.parse_bool(tok, true),
            False => self.parse_bool(tok, false),
            If => self.parse_if(tok),
            Loop => self.parse_loop(tok),
            While => self.parse_while(tok),
            For => self.parse_for(tok),
            Use => self.parse_use(tok),
            Charm => self.alloc(ast::Node::new(ast::NodeKind::Charm, tok.span)),
            NoChar => todo!(),
            UntermQuot => todo!(),
            UntermQuotEsc => todo!(),
            UntermStr => todo!(),
            Unknown => self.alloc(ast::Node::new(ast::NodeKind::Unknown, tok.span)),
            // _ => panic!("Illegal or not implemented: {:#?}", tok), // TODO: Rich error messages
            _ => self.diag.fail(
                ErrorKind::Syntax,
                "Invalid Nud".to_string(),
                tok.span,
                self.arena,
            ),
        }
    }

    pub fn parse_led(
        &mut self,
        lhs: ast::NodeRef<'ast>,
        tok: Token,
    ) -> ast::NodeRef<'ast> {
        use TokenKind::*;
        // Maybe make this a manual pattern?
        if let Some(op_kind) = BinaryKind::from_token(tok) {
            self.parse_binary(lhs, tok, op_kind)
        } else {
            match tok.kind {
                DotDot => self.parse_led_range(lhs, RangeKind::Full),
                DotDotLt => self.parse_led_range(lhs, RangeKind::Excl),
                DotDotEq => self.parse_led_range(lhs, RangeKind::Incl),
                LParen => self.parse_invocation(lhs, tok), // Maybe broaden to ( / [ / {
                LBrack => todo!(),
                At => todo!(),
                Colon => self.parse_pair(lhs, tok), // There is a better way for sure than to pass in tok
                _ => panic!(),
            }
        }
    }

    #[inline(always)]
    pub fn parse_expr(&mut self, min_prec: Precedence) -> ast::NodeRef<'ast> {
        let left_tok = self.next();
        let mut left = self.parse_nud(left_tok);
        while min_prec < self.peek_prec() {
            let tok = self.next();
            left = self.parse_led(left, tok);
        }
        left
    }

    pub fn parse_stmts<F>(&mut self, should_exit: F) -> Vec<ast::NodeRef<'ast>>
    where
        F: Fn(Token) -> bool,
    {
        let mut stmts = Vec::new();
        loop {
            self.eat_nls();
            if self.peek().is_eof() {
                break;
            }
            if should_exit(self.peek()) {
                self.next();
                break;
            }

            let stmt = self.parse_expr(Precedence::None);
            stmts.push(stmt);
        }
        stmts
    }

    pub fn parse(mut self) -> Vec<ast::NodeRef<'ast>> {
        let stmts = self.parse_stmts(|_| false);
        stmts
    }
}
