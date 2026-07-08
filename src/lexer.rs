use std::{hint, str};

use smallvec::SmallVec;

use crate::{
    core::span::Span,
    error::Diagnostics,
    token::{Token, kind::TokenKind},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FilterMode {
    ignore_comments: bool,
    ignore_whitespace: bool,
}

impl Default for FilterMode {
    fn default() -> Self {
        Self::WHITESPACE_AND_COMMENTS
    }
}

impl FilterMode {
    pub const WHITESPACE_AND_COMMENTS: Self =
        Self { ignore_comments: true, ignore_whitespace: true };

    pub const WHITESPACE: Self = Self { ignore_comments: false, ignore_whitespace: true };

    pub const NONE: Self = Self { ignore_comments: false, ignore_whitespace: false };

    pub const fn ignore_comments(self) -> bool {
        self.ignore_comments
    }

    pub const fn ignore_whitespace(self) -> bool {
        self.ignore_whitespace
    }
}

trait IdentLegal {
    fn ident_legal(&self) -> bool;
}

impl IdentLegal for u8 {
    #[inline(always)]
    fn ident_legal(&self) -> bool {
        matches!(self, b'0'..=b'9' | b'_' | b'a'..=b'z' | b'A'..=b'Z')
    }
}

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
        // TODO: Speed up
        self.bytes.get(self.idx).copied()
    }

    fn push_interp_stack(&mut self) {
        let Some(parens) = self.interp_stack.last_mut() else {
            return;
        };
        *parens += 1;
    }

    fn pop_interp_stack(&mut self) {
        let Some(parens) = self.interp_stack.last_mut() else {
            return;
        };

        if *parens == 1 {
            self.interp_stack.pop();
            self.just_exited_interp = true;
        } else {
            *parens -= 1;
        }
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

    fn should_filter(&self, kind: TokenKind) -> bool {
        use TokenKind::*;
        if self.filter_mode.ignore_whitespace() && kind == Whitespace {
            true
        } else if self.filter_mode.ignore_comments() && kind.is_comment() {
            true
        } else {
            false
        }
    }

    fn skip_non_ascii(&mut self) {
        let width = self.bytes[self.idx].leading_ones() as usize;
        if 2 <= width && width <= 4 && self.idx + width <= self.bytes.len() {
            // I'm sure I could optimize this
            let char_bytes = &self.bytes[self.idx..self.idx + width];
            if str::from_utf8(char_bytes).is_ok() {
                self.idx += width;
                return;
            }
        }

        // TODO: Warn
        self.idx += 1;
    }

    fn eat_until<P>(&self, skip: usize, predicate: P) -> usize
    where
        P: Fn(&u8) -> bool,
    {
        self.bytes[self.idx + skip..].iter().position(predicate).unwrap_or(0) + skip
    }

    fn line_comment(&self) -> usize {
        self.eat_until(2, |&b| b == b'\n')
    }

    fn block_comment(&self) -> NextToken {
        // TODO: Make SIMD by in chunks bitmasking all /* and */ and counting
        let mut depth = 1;
        let mut offset = 2;
        while depth > 0 {
            let Some((idx, depth_delta)) = self.bytes[self.idx + offset..]
                .windows(2)
                .enumerate()
                .map(|(idx, window)| {
                    let is_start = window == [b'/', b'*'];
                    let is_end = window == [b'*', b'/'];
                    (idx, is_start as i8 - is_end as i8)
                })
                .find(|&(_, delta)| delta != 0)
            else {
                return (TokenKind::UntermComment, self.bytes.len() - self.idx);
            };

            offset += idx + 2;
            depth += depth_delta;

            // This is a fatal error
            if depth == i8::MAX {
                // TODO: Emit error
                return (TokenKind::Unknown, self.bytes.len() - self.idx);
            }
        }

        (TokenKind::BlockComment, offset)
    }

    fn ident_or_keyword(&self) -> NextToken {
        let len = self.eat_until(1, |&b| !matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'_'));
        let ident = &self.bytes[self.idx..self.idx + len];

        use TokenKind::*;
        let kind = match len {
            2 => match ident {
                b"if" => If,
                b"as" => As,
                b"in" => In,
                b"or" => Or,
                _ => Ident,
            },
            3 => match ident {
                b"let" => Let,
                b"mut" => Mut,
                b"mod" => Mod,
                b"pub" => Pub,
                b"inn" => Inn,
                b"pri" => Pri,
                b"for" => For,
                b"use" => Use,
                b"and" => And,
                _ => Ident,
            },
            4 => match ident {
                b"type" => Type,
                b"enum" => Enum,
                b"idea" => Idea,
                b"func" => Func,
                b"else" => Else,
                b"loop" => Loop,
                b"self" => LSelf,
                b"Self" => BSelf,
                b"true" => True,
                _ => Ident,
            },
            5 => match ident {
                b"const" => Const,
                b"class" => Class,
                b"while" => While,
                b"match" => Match,
                b"break" => Break,
                b"macro" => Macro,
                b"charm" => Charm,
                b"false" => False,
                _ => Ident,
            },
            _ => match ident {
                b"return" => Ret,
                b"continue" => Cont,
                _ => Ident,
            },
        };

        (kind, len)
    }

    fn radix_int(&self) -> usize {
        self.eat_until(2, |&b| !b.ident_legal())
    }

    fn number(&self) -> NextToken {
        // Looks all the way to one byte ahead. Make sure to subtract 1 from offset.
        let mut offset = 0;
        let mut seen_dot = false;
        let mut seen_exp = false;
        let mut just_saw_exp_sign = false;
        for bytes in self.bytes[self.idx..].windows(2) {
            match bytes {
                [b'e' | b'E', b'+' | b'-'] => {
                    just_saw_exp_sign = true;
                    seen_exp = true;
                }
                [b'e' | b'E', b'0'..=b'9' | b'_'] => {}
                [b'e' | b'E', b'a'..=b'z' | b'A'..=b'Z'] => {}
                [b'+' | b'-', _] if just_saw_exp_sign => just_saw_exp_sign = false,
                [b'0'..=b'9' | b'_', _] => {}
                [b'a'..=b'z' | b'A'..=b'Z', _] => {}
                [b'.', b'0'..=b'9'] if !seen_dot => seen_dot = true,

                _ => break,
            }

            offset += 1;
        }

        if self.idx + offset == self.bytes.len() - 1 {
            let last_byte = self.bytes[self.idx + offset];
            if last_byte.ident_legal() {
                offset += 1;
            }
        }

        if seen_dot || seen_exp {
            (TokenKind::Float, offset)
        } else {
            (TokenKind::Int, offset)
        }
    }

    fn char_or_lifetime(&self) -> NextToken {
        // Gives 1 if this is the last character
        let remaining = self.bytes.len() - self.idx;
        if remaining == 1 {
            return (TokenKind::UntermChar, 1);
        }

        let can_be_annot = match &self.bytes[self.idx + 1] {
            b'\n' => return (TokenKind::UntermChar, 2),
            b'\'' => return (TokenKind::NoChar, 2),
            b if b.is_ascii_whitespace() => false,
            _ => true,
        };

        let mut offset = 2;
        let kind = 'eater: {
            for bytes in self.bytes[self.idx + 1..].windows(2) {
                match bytes {
                    [b'\\', b'\''] => {}
                    [_, b'\''] => {
                        offset += 1;
                        break 'eater TokenKind::Char;
                    }
                    [_, b] if can_be_annot && !b.ident_legal() => {
                        break 'eater TokenKind::Annot;
                    }
                    [_, b'\n'] => break 'eater TokenKind::UntermChar,
                    _ => {}
                }

                offset += 1;
            }

            TokenKind::UntermChar
        };

        (kind, offset)
    }

    fn string(&mut self, mut offset: usize) -> NextToken {
        let broke = 'eater: {
            let mut just_saw_esc_dollar = false;
            for bytes in self.bytes[self.idx + offset..].windows(2) {
                match bytes {
                    [b'\\', b'"'] => {}
                    [_, b'"'] => {
                        offset += 2;
                        break 'eater true;
                    }
                    [b'\\', b'$'] => just_saw_esc_dollar = true,
                    [b'$', b'('] if !just_saw_esc_dollar => {
                        self.interp_stack.push(0);
                        break 'eater true;
                    }
                    [b'$', b'('] => just_saw_esc_dollar = false,
                    _ => {}
                }
                offset += 1;
            }
            false
        };

        if broke {
            (TokenKind::String, offset)
        } else {
            (TokenKind::UntermStr, self.bytes.len() - self.idx)
        }
    }

    fn exit_string(&mut self) -> (Token, usize) {
        if self.peek() == Some(b'"') {
            let tok = Token::new(
                TokenKind::String,
                Span::new(self.idx as u32, self.idx as u32 + 1),
            );
            (tok, 1)
        } else {
            let (kind, len) = self.string(0);
            let span = Span::new(self.idx as u32, (self.idx + len) as u32);
            let tok = Token::new(kind, span);
            (tok, len)
        }
    }

    pub fn lex_token_kind(&mut self) -> NextToken {
        let window = self.window();

        // Godbolt analysis confirms this is faster than searching an &[u8]
        use TokenKind::*;
        match window {
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
            [b'"', ..] => self.string(1),

            [b'0', b'a'..=b'z' | b'A'..=b'Z', ..] => {
                (TokenKind::RadixInt, self.radix_int())
            }
            [b'0'..=b'9', ..] => self.number(),

            [b'a'..=b'z' | b'A'..=b'Z' | b'_', ..] => self.ident_or_keyword(),

            // Fast SIMD whitespace??
            [w, ..] if w.is_ascii_whitespace() => (Whitespace, 1),

            // Make sure to catch
            _ => (Unknown, 1), // TODO: Throw an error
        }
    }

    pub fn lex(mut self) -> Vec<Token> {
        // Maybe switch to Arena + BumpVec?
        let mut tokens = Vec::new();
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
