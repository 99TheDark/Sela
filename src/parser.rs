pub mod atoms;
pub mod binary;
pub mod control;
pub mod functions;
pub mod groups;
pub mod numbers;
pub mod properties;
pub mod ranges;
pub mod text;
pub mod unary;
pub mod variables;

use std::{hint, mem::MaybeUninit};

use bumpalo::{Bump, collections::Vec as BumpVec};

use crate::{
    ast::{self, binary::BinaryKind, range::RangeKind, unop::UnOpKind},
    core::span::Span,
    diagnostics::{Diagnostics, ErrorKind},
    token::{Token, kind::TokenKind, precedence::Precedence},
};

pub struct Parser<'tok, 'ast, 'diag, 'src> {
    src: &'src str,
    tokens: &'tok [Token],
    idx: usize,
    diag: &'diag mut Diagnostics<'src>,
    arena: &'ast Bump,
    eof_token: Token,
    brace_stop_depth: usize,
}

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

        Self { src, tokens, idx: 0, diag, arena, eof_token, brace_stop_depth: 0 }
    }

    fn alloc<T>(&mut self, elem: T) -> &'ast T {
        self.arena.alloc(elem)
    }

    fn alloc_node(&mut self, kind: ast::NodeKind<'ast>, span: Span) -> ast::NodeRef<'ast> {
        self.alloc(ast::Node::new(kind, span))
    }

    // What about early returns?? I wish there were linear types too...
    // Could use Bump::alloc_with?
    fn reserve<T>(&self) -> &'ast mut MaybeUninit<T> {
        self.arena.alloc(MaybeUninit::<T>::uninit())
    }

    #[inline]
    fn eat_while<F>(&mut self, cond: F)
    where
        F: Fn(Token) -> bool,
    {
        while self.idx < self.tokens.len() && cond(self.tokens[self.idx]) {
            self.idx += 1;
        }
    }

    // There must be a way to make this more performant...
    #[inline(always)]
    fn eat_nls(&mut self) {
        self.eat_while(|tok| tok.is_nl());
    }

    fn peek(&self) -> Token {
        *self.tokens.get(self.idx).unwrap_or(&self.eof_token)
    }

    fn next(&mut self) -> Token {
        self.eat_nls();
        self.true_next()
    }

    fn true_next(&mut self) -> Token {
        if self.idx < self.tokens.len() {
            let tok = self.tokens[self.idx];
            self.idx += 1;
            tok
        } else {
            hint::cold_path();
            self.eof_token
        }
    }

    #[inline]
    fn recover<F: Fn(TokenKind) -> bool>(&mut self, is_end: F) {
        self.eat_while(|t| !t.is_nl() && !is_end(t.kind));
    }

    fn recover_if_error<F: Fn(TokenKind) -> bool>(&mut self, node: ast::NodeRef<'ast>, is_end: F) {
        if node.is_error() {
            self.recover(is_end);
        }
    }

    fn expect(&mut self, expected: TokenKind, cur_span: Span) -> Result<Token, ast::NodeRef<'ast>> {
        self.eat_nls();
        let tok = self.peek();
        if !tok.is(expected) {
            hint::cold_path();
            self.diag.emit(
                ErrorKind::Syntax,
                format!("Expected {:?} token, found {:?} token instead", expected, tok.kind),
                tok.span,
            );
            self.recover(|t| t == expected);
            Err(self.alloc_node(ast::NodeKind::Error, cur_span))
        } else {
            self.true_next();
            Ok(tok)
        }
    }

    fn expect_invariant(&mut self, expected: TokenKind) -> Token {
        let tok = self.next();
        if !tok.is(expected) {
            hint::cold_path(); // Should be ICE cold
            panic!(
                "Internal compiler error: Lexer invariant contradicted. Expected {:?} token, found {:?} instead",
                expected, tok
            );
        }
        tok
    }

    fn parse_pre_body(&mut self) -> ast::NodeRef<'ast> {
        // This is so ugly but I have no better solution
        self.brace_stop_depth += 1;
        let expr = self.parse_expr(Precedence::None);
        self.brace_stop_depth -= 1;
        expr
    }

    #[inline(always)]
    fn peek_prec(&self) -> Precedence {
        self.tokens.get(self.idx).map_or(Precedence::None, |tok| tok.led_prec())
    }

    fn parse_nud(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        use TokenKind::*;
        match tok.kind {
            Ident => self.parse_ident(tok),
            Int => self.parse_int(tok),
            RadixInt => self.parse_radix_int(tok),
            Float => self.parse_float(tok),
            Char => self.parse_char(tok),
            String => self.parse_string(tok),
            Annot => todo!(),
            Plus => self.parse_unop(tok, UnOpKind::Pos),
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
            LBrace if self.brace_stop_depth != 0 => {
                // But what about something like func(...) -> ({}) {}?
                // I need to have a custom type parser or a stack
                hint::cold_path();
                self.diag.fail(
                    ErrorKind::Syntax,
                    format!("Invalid type: Hit start of block"),
                    tok.span,
                    self.arena,
                )
            }
            LBrace => self.parse_block(tok),
            DotDot => self.parse_nud_range(tok, RangeKind::Full),
            DotDotLt => self.parse_nud_range(tok, RangeKind::Excl),
            DotDotEq => self.parse_nud_range(tok, RangeKind::Incl),
            Tick => todo!(),
            Let => self.parse_decl(tok),
            Mut => self.parse_mut(tok),
            True => self.parse_bool(tok, true),
            False => self.parse_bool(tok, false),
            If => self.parse_if(tok),
            Loop => self.parse_loop(tok),
            While => self.parse_while(tok),
            For => self.parse_for(tok),
            Func => self.parse_func(tok),
            Use => self.parse_use(tok),
            Charm => self.alloc_atom(ast::NodeKind::Charm, tok),
            EmptyChar => todo!(),
            UntermChar => self.alloc_atom(ast::NodeKind::UnknownChar, tok),
            UntermStr => self.alloc_atom(ast::NodeKind::UnknownString, tok),
            Unknown => self.alloc_atom(ast::NodeKind::Error, tok),
            // _ => panic!("Illegal or not implemented: {:#?}", tok), // TODO: Rich error messages
            _ => self.diag.fail(
                ErrorKind::Syntax,
                format!("Invalid Nud {:?}", tok.kind),
                tok.span,
                self.arena,
            ),
        }
    }

    fn parse_led(&mut self, lhs: ast::NodeRef<'ast>, tok: Token) -> ast::NodeRef<'ast> {
        use TokenKind::*;
        // Maybe make this a manual pattern?
        if let Some(op_kind) = BinaryKind::from_token(tok) {
            self.parse_binary(lhs, tok, op_kind)
        } else {
            match tok.kind {
                Dot => self.parse_access(lhs),
                DotDot => self.parse_led_range(lhs, RangeKind::Full),
                DotDotLt => self.parse_led_range(lhs, RangeKind::Excl),
                DotDotEq => self.parse_led_range(lhs, RangeKind::Incl),
                LParen => self.parse_invocation(lhs, tok), // Maybe broaden to ( / [ / {
                LBrack => todo!(),
                At => todo!(),
                Colon => self.parse_pair(lhs, tok), // There is a better way for sure than to pass in tok
                In => self.parse_in(lhs),
                _ => panic!(),
            }
        }
    }

    #[inline(always)]
    fn parse_expr(&mut self, min_prec: Precedence) -> ast::NodeRef<'ast> {
        let left_tok = self.next();
        let mut left = self.parse_nud(left_tok)?;
        while min_prec < self.peek_prec() {
            let tok = self.next();
            left = self.parse_led(left, tok)?;
        }
        left
    }

    fn parse_stmts(
        &mut self,
        closing_kind: Option<TokenKind>, // Could change to a generic for monomorphization
    ) -> BumpVec<'ast, ast::NodeRef<'ast>> {
        let mut stmts = BumpVec::new_in(self.arena);
        loop {
            self.eat_nls();
            if self.peek().is_eof() {
                hint::cold_path();
                break;
            }
            if Some(self.peek().kind) == closing_kind {
                self.true_next();
                break;
            }

            let stmt = self.parse_expr(Precedence::None);
            self.recover_if_error(stmt, |t| Some(t) == closing_kind);
            stmts.push(stmt);
        }
        stmts
    }

    pub fn parse(mut self) -> BumpVec<'ast, ast::NodeRef<'ast>> {
        self.parse_stmts(None)
    }
}
