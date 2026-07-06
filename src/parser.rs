pub mod basic;
pub mod control;

use std::{borrow::Borrow, str::FromStr};

use arrayvec::ArrayVec;
use bumpalo::Bump;
use regex::Regex;

use crate::{
    ast::{self, binary::BinaryKind, unop::UnOpKind},
    core::span::Span,
    error::{Diagnostics, ErrorKind, natural::Natural},
    token::{Token, kind::TokenKind, precedence::Precedence},
};
use regex_macro::regex;

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
    #[inline(always)]
    pub fn eat_nls(&mut self) {
        self.eat_until(|tok| tok.is_nl());
    }

    #[inline(always)]
    pub fn eat_line(&mut self) {
        self.eat_until(|tok| !tok.is_nl());
    }

    #[inline]
    pub fn peek(&self) -> Token {
        // Is this check even needed? Could we make this something else?
        if self.idx < self.tokens.len() { self.tokens[self.idx] } else { self.eof_token }
    }

    #[inline]
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
            DotDot | DotDotLt | DotDotEq => self.parse_range(tok),
            Tick => todo!(),
            Let => self.parse_decl(tok),
            True => self.parse_bool(tok, true),
            False => self.parse_bool(tok, false),
            Loop => self.parse_loop(tok),
            While => self.parse_while(tok),
            For => self.parse_for(tok),
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
        // TODO: Use precedence, duh
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
                LParen => self.parse_invocation(lhs, tok), // Maybe broaden to ( / [ / {
                LBrack => todo!(),
                At => todo!(),
                Colon => self.parse_pair(lhs, tok), // There is a better way for sure than to pass in tok
                _ => panic!(),
            }
        }
    }

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
            if self.peek().is_eof() || should_exit(self.peek()) {
                break;
            }

            let stmt = self.parse_expr(Precedence::None);
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
        binary: BinaryKind,
    ) -> ast::NodeRef<'ast> {
        let rhs = self.parse_expr(tok.led_prec());
        binary.make_node(lhs, rhs, self.arena)
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
        failure: ast::NodeKind<'ast>,
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
            // TODO: Make a method
            /*self.diag.fail(
                ErrorKind::Syntax,
                format!(
                    "Invalid {} literal: underscores may not appear {}",
                    name,
                    issues.join_natural("or")
                ),
                tok.span,
                self.arena,
            )*/
            self.diag.emit(
                ErrorKind::Syntax,
                format!(
                    "Invalid {} literal: underscores may not appear {}",
                    name,
                    issues.join_natural("or")
                ),
                tok.span,
            );
            self.alloc(ast::Node::new(failure, tok.span))
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
            ast::NodeKind::UnknownInt,
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
            ast::NodeKind::UnknownFloat,
        )
    }

    fn parse_char(&mut self, _tok: Token) -> ast::NodeRef<'ast> {
        todo!()
    }

    fn parse_string(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        // TODO: Unescape
        let mut frags =
            vec![ast::string::Fragment::String(tok.src(self.src).to_string())];
        let mut end = tok.span;
        while self.peek().eof_is(TokenKind::Dollar) {
            self.next();
            self.expect(TokenKind::LParen);
            let interp = self.parse_expr(Precedence::None);
            let rparen = self.expect(TokenKind::RParen);

            frags.push(ast::string::Fragment::Expr(interp));

            if self.peek().is(TokenKind::String) {
                // TODO: Same deal here
                let str_tok = self.next();
                frags.push(ast::string::Fragment::String(
                    str_tok.src(self.src).to_string(),
                ));

                end = str_tok.span;
            } else {
                end = rparen.span;
            }
        }

        let frags = self.alloc(frags);
        self.alloc(ast::Node::new(ast::NodeKind::String(&frags), tok.span.to(end)))
    }

    fn parse_range(&mut self, _tok: Token) -> ast::NodeRef<'ast> {
        todo!()
    }

    fn parse_ident(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let kind = ast::NodeKind::Ident(tok.src(self.src));
        self.alloc(ast::Node::new(kind, tok.span))
    }

    fn parse_delimited<F: FnOnce(&'ast [ast::NodeRef<'ast>]) -> ast::NodeKind<'ast>>(
        &mut self,
        tok: Token,
        delim: TokenKind,
        end: TokenKind,
        constructor: F,
    ) -> ast::NodeRef<'ast> {
        let mut elems = Vec::with_capacity(4);
        self.eat_nls();
        while self.peek().eof_not_is(end) {
            let expr = self.parse_expr(Precedence::None);
            elems.push(expr);
            self.eat_nls();

            if self.peek().kind == end {
                break;
            } else {
                self.expect(delim);
            }
        }
        let end = self.next();

        let elems = self.alloc(elems);
        let kind = constructor(&elems);
        self.alloc(ast::Node::new(kind, tok.span.to(end.span)))
    }

    // Maybe return a Result or Option and propogate that up until it can be handled?
    fn parse_decl(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let pat = self.parse_expr(Precedence::Assign);
        self.expect(TokenKind::Eq);
        let val = self.parse_expr(Precedence::Assign);
        self.alloc(ast::Node::new(
            ast::NodeKind::Decl { pat, val },
            tok.span.to(val.span),
        ))
    }

    fn parse_block(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let elems = self.parse_stmts(|tok| tok.is(TokenKind::RBrace));
        let elems = self.alloc(elems);
        let end = elems.last().map_or(tok.span, |last| last.span);
        self.alloc(ast::Node::new(ast::NodeKind::Block(elems), tok.span.to(end)))
    }

    fn parse_loop(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        self.expect(TokenKind::RBrace);
        let body = self.parse_block(tok);
        self.alloc(ast::Node::new(ast::NodeKind::Loop { body }, tok.span.to(body.span)))
    }

    fn parse_while(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let cond = self.parse_expr(Precedence::None);
        self.expect(TokenKind::RBrace);
        let body = self.parse_block(tok);
        self.alloc(ast::Node::new(
            ast::NodeKind::While { cond, body },
            tok.span.to(body.span),
        ))
    }

    fn parse_for(&mut self, tok: Token) -> ast::NodeRef<'ast> {
        let vari = self.parse_expr(Precedence::None);
        self.expect(TokenKind::In);
        let iter = self.parse_expr(Precedence::None);
        self.expect(TokenKind::RBrace);
        let body = self.parse_block(tok);
        self.alloc(ast::Node::new(
            ast::NodeKind::For { vari, iter, body },
            tok.span.to(body.span),
        ))
    }

    fn parse_pair(&mut self, lhs: ast::NodeRef<'ast>, tok: Token) -> ast::NodeRef<'ast> {
        let rhs = self.parse_expr(tok.led_prec());
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
        let rhs = self.parse_delimited(
            tok,
            TokenKind::Comma,
            TokenKind::RParen,
            ast::NodeKind::Parens,
        );
        self.alloc(ast::Node::new(
            ast::NodeKind::Invoc { callee: lhs, args: rhs },
            lhs.span.to(rhs.span),
        ))
    }
}
