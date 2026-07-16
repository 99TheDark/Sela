pub mod comments;
pub mod filtering;
pub mod numbers;
pub mod text;
pub mod vectorized;
pub mod whitespace;
pub mod words;

use std::{hint, str};

use smallvec::SmallVec;

use crate::{
    core::span::Span,
    diagnostics::Diagnostics,
    lexer::filtering::FilterMode,
    token::{Token, kind::TokenKind},
};

pub struct Lexer<'tok, 'src> {
    bytes: &'tok [u8],
    idx: usize,
    interp_stack: SmallVec<[u32; 2]>,
    just_exited_interp: bool,
    diag: &'tok mut Diagnostics<'src>,
    filter_mode: FilterMode,
}

const WINDOW_SIZE: usize = 4;
type Window = [u8; WINDOW_SIZE];

type NextToken = (TokenKind, usize);

impl<'tok, 'src> Lexer<'tok, 'src> {
    pub fn new_with_mode(
        src: &'tok str,
        diag: &'tok mut Diagnostics<'src>,
        filter_mode: FilterMode,
    ) -> Self {
        Self {
            bytes: src.as_bytes(),
            idx: 0,
            interp_stack: SmallVec::new(),
            just_exited_interp: false,
            diag,
            filter_mode,
        }
    }

    pub fn new(src: &'tok str, diag: &'tok mut Diagnostics<'src>) -> Self {
        Self {
            bytes: src.as_bytes(),
            idx: 0,
            interp_stack: SmallVec::new(),
            just_exited_interp: false,
            diag,
            filter_mode: FilterMode::default(),
        }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.idx).copied()
    }

    fn window(&self) -> Window {
        if self.idx >= self.bytes.len() {
            hint::cold_path();
            return [0u8; 4];
        }

        let remaining = &self.bytes[self.idx..];
        if let Some(window) = remaining.first_chunk::<WINDOW_SIZE>() {
            *window
        } else {
            hint::cold_path();
            let mut buf = [0u8; 4];
            buf[..remaining.len()].copy_from_slice(remaining);
            buf
        }
    }

    fn eat_until<P>(&self, skip: usize, predicate: P) -> usize
    where
        P: Fn(&u8) -> bool,
    {
        self.bytes[self.idx + skip..]
            .iter()
            .position(predicate)
            .map_or(self.bytes.len() - self.idx, |pos| pos + skip)
    }

    pub fn lex_token_kind(&mut self) -> NextToken {
        let window = self.window();

        // Godbolt analysis confirms this is faster than searching an &[u8]
        use TokenKind::*;
        match window {
            [b'\n', b'\n', b'\n', b'\n'] => (Whitespace, 3),
            [b'\n', b'\n', b'\n', ..] => (Whitespace, 2),
            [b'\n', b'\n', ..] => (Whitespace, 1),
            [b'\n', ..] => (NewLine, 1),

            [b'+', b'=', ..] => (PlusEq, 2),
            [b'+', ..] => (Plus, 1),

            [b'-', b'=', ..] => (DashEq, 2),
            [b'-', b'>', ..] => (Arrow, 2),
            [b'-', ..] => (Dash, 1),

            [b'*', b'=', ..] => (StarEq, 2),
            [b'*', ..] => (Star, 1),

            [b'/', b'=', ..] => (SlashEq, 2),
            [b'/', b'/', ..] => (LineComment, self.line_comment()),
            [b'/', b'*', ..] => self.block_comment(),
            [b'/', ..] => (Slash, 1),

            [b'%', b'=', ..] => (PctEq, 2),
            [b'%', ..] => (Pct, 1),

            [b'&', b'=', ..] => (AmpEq, 2),
            [b'&', ..] => (Amp, 1),

            [b'|', b'=', ..] => (BarEq, 2),
            [b'|', ..] => (Bar, 1),

            [b'^', b'=', ..] => (CaretEq, 2),
            [b'^', ..] => (Caret, 1),

            [b'(', ..] => {
                self.push_interp_stack();
                (LParen, 1)
            }
            [b')', ..] => {
                self.pop_interp_stack();
                (RParen, 1)
            }

            [b'[', ..] => (LBrack, 1),
            [b']', ..] => (RBrack, 1),

            [b'{', ..] => (LBrace, 1),
            [b'}', ..] => (RBrace, 1),

            [b'@', ..] => (At, 1),
            [b'$', ..] => (Dollar, 1),

            [b'.', b'.', b'<', ..] => (DotDotLt, 3),
            [b'.', b'.', b'=', ..] => (DotDotEq, 3),
            [b'.', b'.', ..] => (DotDot, 2),
            [b'.', ..] => (Dot, 1),
            [b',', ..] => (Comma, 1),

            [b':', ..] => (Colon, 1),
            [b';', ..] => (Semi, 1),

            [b'=', b'=', ..] => (EqEq, 2),
            [b'=', ..] => (Eq, 1),

            [b'>', b'>', b'=', ..] => (GtGtEq, 3),
            [b'>', b'=', ..] => (GtEq, 2),
            [b'>', b'>', ..] => (GtGt, 2),
            [b'>', ..] => (Gt, 1),

            [b'<', b'<', b'=', ..] => (LtLtEq, 3),
            [b'<', b'=', ..] => (LtEq, 2),
            [b'<', b'<', ..] => (LtLt, 2),
            [b'<', ..] => (Lt, 1),

            [b'!', b'=', ..] => (NotEq, 2),
            [b'!', ..] => (Not, 1),

            [b'\'', ..] => self.char_or_lifetime(),
            [b'"', ..] => self.string(),

            [b'0', b'a'..=b'z' | b'A'..=b'Z', ..] => (TokenKind::RadixInt, self.radix_int()),
            [b'0'..=b'9', ..] => self.number(),

            [b'a'..=b'z' | b'A'..=b'Z' | b'_', ..] => self.ident_or_keyword(),

            [b' ', b' ', b' ', b' '] => (Whitespace, 4),
            [b'\t', b'\t', b'\t', b'\t'] => (Whitespace, 4),

            [w, ..] if w.is_ascii_whitespace() => (Whitespace, self.whitespace()),

            _ => (Unknown, 1), // TODO: Throw an error
        }
    }

    pub fn lex(mut self) -> Vec<Token> {
        let mut tokens = Vec::with_capacity(self.bytes.len() / 6); // 4 or 6, hard to say
        while let Some(byte) = self.peek() {
            let start = self.idx;

            if !byte.is_ascii() {
                hint::cold_path();
                self.skip_non_ascii();
            }

            let (kind, len) = self.lex_token_kind();
            self.idx += len;

            if !self.should_filter(kind) {
                tokens.push(Token::new(kind, Span::new(start as u32, self.idx as u32)));
            }

            if self.just_exited_interp {
                hint::cold_path();
                self.just_exited_interp = false;
                let (token, len) = self.exit_string();
                tokens.push(token);
                self.idx += len;
            }
        }
        tokens
    }
}
