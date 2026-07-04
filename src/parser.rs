pub mod basic;
pub mod binary;
pub mod control;
pub mod import;
pub mod literal;
pub mod set;
pub mod unary;

use std::{borrow::Borrow, str::FromStr};

use arrayvec::ArrayVec;
use bumpalo::Bump;
use regex::Regex;

use crate::{
    ast::{self, binary::BinaryKind, unop::UnOpKind},
    core::span::Span,
    error::{Diagnostics, ErrorKind, natural::Natural},
    token::{Token, kind::TokenKind},
};
use regex_macro::regex;

/*pub struct RDParser<'ast, 'diag, 'src> {
    src: &'src str,
    tokens: &'src [Token],
    idx: usize,
    in_recovery: bool,
    diag: &'diag mut Diagnostics<'src>,
    eof_token: Token,
    arena: &'ast Bump,
}

impl<'ast, 'diag, 'src> RDParser<'ast, 'diag, 'src> {
    pub fn new(
        src: &'src str,
        tokens: &'src [Token],
        diag: &'diag mut Diagnostics<'src>,
        arena: &'ast Bump,
    ) -> Self {
        let eof_loc = tokens.last().map_or(0, |tok| tok.span.end);

        Self {
            src,
            tokens,
            idx: 0,
            in_recovery: false,
            diag,
            eof_token: Token::new(TokenKind::EOF, Span::single(eof_loc)),
            arena,
        }
    }

    pub fn advance(&mut self) {
        // Is this even needed?
        if self.idx < self.tokens.len() {
            self.idx += 1;
        }
    }

    pub fn next(&mut self) -> Token {
        self.eat_nls();
        if self.idx < self.tokens.len() {
            let tok = self.tokens[self.idx];
            self.idx += 1;
            tok
        } else {
            self.eof_token
        }
    }

    pub fn eat_until<F>(&mut self, cond: F)
    where
        F: Fn(Token) -> bool,
    {
        while self.idx < self.tokens.len() && cond(self.tokens[self.idx]) {
            self.idx += 1;
        }
    }

    pub fn eat_nls(&mut self) {
        self.eat_until(|tok| tok.is_nl());
    }

    pub fn eat_line(&mut self) {
        self.eat_until(|tok| !tok.is_nl());
    }

    pub fn current(&self) -> Token {
        if self.idx < self.tokens.len() { self.tokens[self.idx] } else { self.eof_token }
    }

    pub fn peek(&self) -> Token {
        if self.idx + 1 < self.tokens.len() {
            self.tokens[self.idx + 1]
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

    pub fn expect_keyword(&mut self, expected: Keyword) -> Token {
        let tok = self.next();

        let kw = tok.to_keyword(self.src);
        if kw != expected {
            if kw == Keyword::NotReserved {
                self.diag.emit(
                    ErrorKind::Syntax,
                    format!(
                        "Expected {:?} keyword, found {:?} token instead",
                        expected, tok.kind
                    ),
                    tok.span,
                );
            } else {
                self.diag.emit(
                    ErrorKind::Syntax,
                    format!(
                        "Expected {:?} keyword, found {:?} keyword instead",
                        expected, kw
                    ),
                    tok.span,
                );
            }
        }

        tok
    }

    pub fn at_keyword(&self, kw: Keyword) -> bool {
        self.current().to_keyword(self.src) == kw
    }

    pub fn at_and_eat(&mut self, kind: TokenKind) -> bool {
        if self.current().kind == kind {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn alloc<T>(&mut self, elem: T) -> &'ast T {
        self.arena.alloc(elem)
    }

    pub fn parse_stmts<F>(&mut self, should_exit: F) -> Vec<&'ast ast::Node<'ast>>
    where
        F: Fn(Token) -> bool,
    {
        let mut stmts = Vec::new();
        loop {
            self.eat_nls();

            if self.current().is_eof() || should_exit(self.current()) {
                break;
            }

            let stmt = self.parse_stmt();
            stmts.push(stmt);
        }
        stmts
    }

    pub fn parse(mut self) -> Vec<ast::NodeRef<'ast>> {
        self.parse_stmts(|_| false)
    }
}*/

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

    pub fn eat_until<F>(&mut self, cond: F)
    where
        F: Fn(Token) -> bool,
    {
        while self.idx < self.tokens.len() && cond(self.tokens[self.idx]) {
            self.idx += 1;
        }
    }

    pub fn eat_nls(&mut self) {
        self.eat_until(|tok| tok.is_nl());
    }

    pub fn eat_line(&mut self) {
        self.eat_until(|tok| !tok.is_nl());
    }

    pub fn current(&self) -> Token {
        // Is this check even needed? Could we make this something else?
        if self.idx < self.tokens.len() { self.tokens[self.idx] } else { self.eof_token }
    }

    pub fn next(&mut self) -> Token {
        self.eat_nls();
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

    pub fn peek_prec(&self) -> u8 {
        self.tokens.get(self.idx).map_or(0, |tok| tok.led_prec())
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
            LParen => self.parse_delimited(
                tok,
                TokenKind::Comma,
                TokenKind::RParen,
                ast::NodeKind::Parens,
            ),
            LBrack => todo!(),
            LBrace => self.parse_delimited(
                tok,
                TokenKind::NewLine,
                TokenKind::RBrace,
                ast::NodeKind::Block,
            ),
            DotDot | DotDotLt | DotDotEq => self.parse_range(tok),
            Dollar => todo!(),
            Tick => todo!(),
            Let => self.parse_decl(tok),
            True => self.parse_bool(tok, true),
            False => self.parse_bool(tok, false),
            NoChar => todo!(),
            UntermQuot => todo!(),
            UntermQuotEsc => todo!(),
            UntermStr => todo!(),
            Unknown => self.alloc(ast::Node::new(ast::NodeKind::Unknown, tok.span)),
            _ => panic!("Illegal or not implemented: {:#?}", tok), // TODO: Rich error messages
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
        } else if matches!(
            tok.kind,
            Eq | PlusEq
                | DashEq
                | StarEq
                | SlashEq
                | PctEq
                | GtGtEq
                | LtLtEq
                | CaretEq
                | AmpEq
                | BarEq
        ) {
            // todo!()
            self.diag.fail(
                ErrorKind::Syntax,
                "This is some equal stuff not yet made".to_string(),
                tok.span,
                self.arena,
            )
        } else {
            match tok.kind {
                LParen => self.parse_invocation(lhs, tok), // Maybe broaden to ( / [ / {
                LBrack => todo!(),
                LBrace => todo!(),
                At => todo!(),
                Colon => self.parse_pair(lhs, tok), // There is a better way for sure than to pass in tok
                Comma => todo!(),
                Arrow => todo!(),
                _ => panic!(),
            }
        }
    }

    pub fn parse_expr(&mut self, min_prec: u8) -> ast::NodeRef<'ast> {
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

            if self.current().is_eof() || should_exit(self.current()) {
                break;
            }

            let stmt = self.parse_expr(0);
            stmts.push(stmt);
        }
        stmts
    }

    pub fn parse(mut self) -> Vec<ast::NodeRef<'ast>> {
        self.parse_stmts(|_| false)
    }
}

impl<'tok, 'ast, 'diag, 'src> Parser<'tok, 'ast, 'diag, 'src>
where
    'src: 'ast,
    'src: 'tok,
{
    fn parse_binary(
        &mut self,
        lhs: ast::NodeRef<'ast>,
        tok: Token,
        bin_op: BinaryKind,
    ) -> ast::NodeRef<'ast> {
        let rhs = self.parse_expr(tok.led_prec());
        bin_op.make_node(lhs, rhs, self.arena)
    }

    fn parse_unop(&mut self, tok: Token, op: UnOpKind) -> ast::NodeRef<'ast> {
        let rhs = self.parse_expr(tok.nud_prec());
        let kind = ast::NodeKind::UnOp { op, rhs };
        self.alloc(ast::Node::new(kind, tok.span.to(rhs.span)))
    }

    fn parse_bool(&mut self, tok: Token, val: bool) -> ast::NodeRef<'ast> {
        let kind = ast::NodeKind::Bool(val);
        self.alloc(ast::Node::new(kind, tok.span))
    }

    #[inline(always)]
    fn parse_number<
        R: std::ops::Deref<Target = Regex>,
        T: FromStr,
        F: FnOnce(T) -> ast::NodeKind<'ast>,
        const N: usize,
    >(
        &mut self,
        tok: Token,
        name: &'static str,
        underscore_checks: ArrayVec<(&R, &'static str), N>, // Maybe make ArrayVec<Check>?
        constructor: F,
    ) -> ast::NodeRef<'ast> {
        let src = tok.src(self.src);

        let mut issues = ArrayVec::<&str, N>::new();
        for (regex, issue) in underscore_checks {
            if regex.borrow().is_match(src) {
                issues.push(issue);
            }
        }

        if issues.is_empty() {
            let Ok(val) = src.replace("_", "").parse() else {
                return self.alloc(ast::Node::failed(tok.span));
            };

            self.alloc(ast::Node::new(constructor(val), tok.span))
        } else {
            self.diag.fail(
                ErrorKind::Syntax,
                format!(
                    "Invalid {} literal: underscores may not appear {}",
                    name,
                    issues.join_natural("or")
                ),
                tok.span,
                self.arena,
            )
        }
    }

    fn parse_int(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        self.parse_number(
            tok,
            "integer",
            ArrayVec::from([
                (regex!(r"__"), "consecutively"),
                (regex!(r"^_"), "at the start of a number"),
                (regex!(r"_$"), "at the end of a number"),
            ]),
            ast::NodeKind::Int,
        )
    }

    fn parse_float(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        self.parse_number(
            tok,
            "floating-point",
            ArrayVec::from([
                (regex!(r"__"), "consecutively"),
                (regex!(r"^_"), "at the start of a number"),
                (regex!(r"_$"), "at the end of a number"),
                (regex!(r"_\."), "before the decimal point"),
                (regex!(r"\._"), "after the decimal point"),
                (regex!(r"_[eE]"), "before an exponent"),
                (regex!(r"[eE]_"), "after an exponent"),
                (regex!(r"_[+-]"), "before an exponential sign"),
                (regex!(r"[+-]_"), "after an exponential sign"),
            ]),
            ast::NodeKind::Float,
        )
    }

    fn parse_char(&mut self, _tok: Token) -> ast::NodeRef<'ast> {
        todo!()
    }

    fn parse_string(&mut self, _tok: Token) -> ast::NodeRef<'ast> {
        todo!()
    }

    fn parse_range(&mut self, _tok: Token) -> ast::NodeRef<'ast> {
        todo!()
    }

    fn parse_ident(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let kind = ast::NodeKind::Ident(tok.src(self.src));
        self.alloc(ast::Node::new(kind, tok.span))
    }

    fn parse_delimited<F: FnOnce(Vec<ast::NodeRef<'ast>>) -> ast::NodeKind<'ast>>(
        &mut self,
        tok: Token,
        delim: TokenKind,
        end: TokenKind,
        constructor: F,
    ) -> ast::NodeRef<'ast> {
        let mut elems = Vec::new();
        while !self.current().is(end) {
            elems.push(self.parse_expr(0));
            self.eat_nls(); // TODO: Also arbitrary AF

            if self.current().is(end) {
                break;
            } else {
                self.expect(delim);
            }
        }
        let end = self.next();
        self.alloc(ast::Node::new(constructor(elems), tok.span.to(end.span)))
    }

    // Maybe return a Result or Option and propogate that up until it can be handled?
    fn parse_decl(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let pat = self.parse_expr(0);
        self.expect(TokenKind::Eq);
        let val = self.parse_expr(0);
        self.alloc(ast::Node::new(
            ast::NodeKind::Decl { pat, val },
            tok.span.to(val.span),
        ))
    }

    fn parse_pair(&mut self, lhs: ast::NodeRef<'ast>, tok: Token) -> ast::NodeRef<'ast> {
        let rhs = self.parse_expr(tok.nud_prec());
        self.alloc(ast::Node::new(
            ast::NodeKind::Pair { lhs, rhs },
            lhs.span.to(rhs.span),
        ))
    }

    fn parse_invocation(
        &mut self,
        lhs: ast::NodeRef<'ast>,
        tok: Token,
    ) -> ast::NodeRef<'ast> {
        let rhs = self.parse_parens(tok);
        self.alloc(ast::Node::new(
            ast::NodeKind::Invoc { callee: lhs, args: rhs },
            lhs.span.to(rhs.span),
        ))
    }

    fn parse_parens(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let mut elems = Vec::with_capacity(4);
        loop {
            elems.push(self.parse_expr(0));

            self.eat_nls(); // Feels kind of random. I feel like I could streamline this
            if self.current().is(TokenKind::RParen) {
                break;
            }

            self.expect(TokenKind::Comma);
            self.eat_nls(); // Same
            if self.current().is(TokenKind::RParen) {
                break;
            }
        }

        let end = self.next();

        self.alloc(ast::Node::new(ast::NodeKind::Parens(elems), tok.span.to(end.span)))
    }
}
