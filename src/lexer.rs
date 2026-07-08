use std::{hint, str};

use smallvec::SmallVec;

use crate::{
    core::span::Span,
    error::Diagnostics,
    token::{Token, kind::TokenKind},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OldTokenFilterMode {
    WhitespaceAndComments,
    Whitespace,
    None,
}

impl Default for OldTokenFilterMode {
    fn default() -> Self {
        Self::WhitespaceAndComments
    }
}

impl OldTokenFilterMode {
    pub fn ignores_comments(self) -> bool {
        self == Self::WhitespaceAndComments
    }

    pub fn ignores_whitespace(self) -> bool {
        self == Self::WhitespaceAndComments || self == Self::Whitespace
    }
}

// 415ms for 3.5MLOC / 55MB
pub struct SlowLexer<'tok, 'src> {
    src: &'src str,
    diag: &'tok mut Diagnostics<'src>,
    idx: u32,
    interp_stack: SmallVec<[usize; 2]>,
    just_exited: bool,
    filter_mode: OldTokenFilterMode,
}

impl<'tok, 'src> SlowLexer<'tok, 'src> {
    pub fn new_with_mode(
        src: &'src str,
        diag: &'tok mut Diagnostics<'src>,
        filter_mode: OldTokenFilterMode,
    ) -> Self {
        Self {
            src,
            diag,
            idx: 0,
            interp_stack: SmallVec::new(),
            just_exited: false,
            filter_mode,
        }
    }

    pub fn new(src: &'src str, diag: &'tok mut Diagnostics<'src>) -> Self {
        Self::new_with_mode(src, diag, OldTokenFilterMode::WhitespaceAndComments)
    }

    pub fn next(&mut self) -> Option<char> {
        // TODO: Use a fast implementation or a custom structure like Cursor<'a>
        let ch = self.src[self.idx as usize..].chars().next()?;
        self.idx += ch.len_utf8() as u32;
        Some(ch)
    }

    pub fn peek(&mut self) -> Option<char> {
        // TODO: Use a fast implementation or a custom structure like Cursor<'a>
        self.src[self.idx as usize..].chars().next()
    }

    pub fn peek2(&mut self) -> Option<char> {
        // TODO: Use a fast implementation or a custom structure like Cursor<'a>
        let mut chars = self.src[self.idx as usize..].chars();
        chars.next();
        chars.next()
    }

    pub fn peek_n(&mut self, n: usize) -> &'src [u8] {
        let end = usize::min(self.idx as usize + n, self.src.len());
        &self.src.as_bytes()[self.idx as usize..end]
    }

    pub fn fork(&mut self, cur: TokenKind, next: char, new: TokenKind) -> TokenKind {
        if self.peek() == Some(next) {
            self.next();
            new
        } else {
            cur
        }
    }

    pub fn try_consume(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.next();
            true
        } else {
            false
        }
    }

    pub fn eat_while<F>(&mut self, valid: F)
    where
        F: Fn(char) -> bool,
    {
        while let Some(ch) = self.peek() {
            if !valid(ch) {
                break;
            }
            self.next();
        }
    }

    pub fn eat_while_and_collect<F>(&mut self, valid: F) -> &'src str
    where
        F: Fn(char) -> bool,
    {
        let start = self.idx - 1;
        while let Some(ch) = self.peek() {
            if !valid(ch) {
                break;
            }
            self.next();
        }
        &self.src[start as usize..self.idx as usize]
    }

    fn ident_or_keyword(&mut self) -> TokenKind {
        let ident = self.eat_while_and_collect(
            |ch| matches!(ch, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9'),
        );

        use TokenKind::*;
        match ident.len() {
            2 => match ident {
                "if" => If,
                "as" => As,
                "in" => In,
                "or" => Or,
                _ => Ident,
            },
            3 => match ident {
                "let" => Let,
                "mut" => Mut,
                "mod" => Mod,
                "pub" => Pub,
                "inn" => Inn,
                "pri" => Pri,
                "for" => For,
                "use" => Use,
                "and" => And,
                _ => Ident,
            },
            4 => match ident {
                "type" => Type,
                "enum" => Enum,
                "idea" => Idea,
                "func" => Func,
                "else" => Else,
                "loop" => Loop,
                "self" => LSelf,
                "Self" => BSelf,
                "true" => True,
                _ => Ident,
            },
            5 => match ident {
                "const" => Const,
                "class" => Class,
                "while" => While,
                "match" => Match,
                "break" => Break,
                "macro" => Macro,
                "charm" => Charm,
                "false" => False,
                _ => Ident,
            },
            _ => match ident {
                "return" => Ret,
                "continue" => Cont,
                _ => Ident,
            },
        }
    }

    fn number(&mut self) -> TokenKind {
        // TODO: How will this lex 0xABC or 123a5?
        let mut seen_dot = false; // This means no .123
        let mut seen_exp = false;
        let mut just_saw_exp = false;
        while let Some(ch) = self.peek() {
            let valid = 'valid: {
                if just_saw_exp {
                    just_saw_exp = false;
                    if matches!(ch, '+' | '-') {
                        break 'valid true;
                    }
                }

                match ch {
                    '.' if matches!(self.peek2(), Some('0'..='9' | '_')) => {
                        if seen_dot {
                            false
                        } else {
                            seen_dot = true;
                            true
                        }
                    }
                    '.' => false,
                    'e' | 'E' => {
                        seen_exp = true;
                        just_saw_exp = true;
                        true
                    }
                    '0'..='9' | '_' => true,
                    _ => false,
                }
            };

            if !valid {
                break;
            }
            self.next();
        }

        if seen_dot || seen_exp { TokenKind::Float } else { TokenKind::Int }
    }

    fn char_or_lifetime(&mut self) -> TokenKind {
        use TokenKind::*;

        let Some(ch) = self.next() else {
            return UntermChar;
        };

        match ch {
            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                // Might be a lifetime
                if self.try_consume('\'') {
                    return Char;
                }

                // If it wasn't 'x' or something, then it is probably a lifetime
                self.eat_while(
                    |ch| matches!(ch, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9'),
                );

                return if self.try_consume('\'') { Char } else { Annot };
            }
            '\n' => return UntermChar,
            '\'' => return NoChar,
            _ => {}
        }

        while let Some(ch) = self.peek() {
            if ch == '\n' {
                hint::cold_path();
                return UntermChar;
            }

            self.next();
            if ch == '\'' {
                break;
            }

            if ch == '\\' && self.peek() == Some('\'') {
                self.next();
            }
        }

        Char
    }

    fn string(&mut self) -> TokenKind {
        // TODO: There is 100% a way to write this better
        if self.peek_n(2) == b"$(" {
            self.interp_stack.push(0);
            return TokenKind::String;
        }

        while let Some(ch) = self.next() {
            match ch {
                '\"' => break,
                '\\' => {
                    self.next();
                }
                _ if self.peek_n(2) == b"$(" => {
                    self.interp_stack.push(0);
                    break;
                }
                _ => {}
            };
        }

        TokenKind::String
    }

    fn block_comment(&mut self) -> bool {
        let mut depth = 1;
        loop {
            let depth_changed = match self.peek_n(2) {
                b"/*" => {
                    depth += 1;
                    true
                }
                b"*/" => {
                    depth -= 1;
                    true
                }
                _ => false,
            };

            let ch = if depth_changed {
                self.next();
                self.next()
            } else {
                self.next()
            };

            // I could make this `if depth == 0 || self.next().is_none()` but that's gross
            if depth == 0 {
                return true;
            }

            if ch.is_none() {
                hint::cold_path();
                return false;
            }
        }
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
            self.just_exited = true;
        } else {
            *parens -= 1;
        }
    }

    fn lex_token_kind(&mut self, ch: char) -> TokenKind {
        use TokenKind::*;
        let kind = match ch {
            '\n' => NewLine,
            c if c.is_whitespace() => Whitespace,
            // Single-line comment: TBD
            // Multi-line comment: TBD
            'a'..='z' | 'A'..='Z' | '_' => self.ident_or_keyword(),
            // TODO: Implement literally every other numeric representation
            '0' => TokenKind::Int,
            '0'..='9' => self.number(),
            // Lit: TBD
            '+' => self.fork(Plus, '=', PlusEq),
            '-' => match self.peek() {
                Some('=') => {
                    self.next();
                    DashEq
                }
                Some('>') => {
                    self.next();
                    Arrow
                }
                _ => Dash,
            },
            '*' => self.fork(Star, '=', StarEq),
            '/' => match self.peek() {
                Some('/') => {
                    self.eat_while(|ch| ch != '\n');
                    LineComment
                }
                Some('*') => {
                    self.next();
                    let terminated = self.block_comment();

                    if terminated {
                        BlockComment
                    } else {
                        hint::cold_path();
                        UntermComment
                    }
                }
                Some('=') => {
                    self.next();
                    SlashEq
                }
                _ => Slash,
            },
            '%' => self.fork(Pct, '=', PctEq),
            '&' => self.fork(Amp, '=', AmpEq),
            '|' => self.fork(Bar, '=', BarEq),
            '^' => self.fork(Caret, '=', CaretEq),
            '(' => {
                self.push_interp_stack();
                LParen
            }
            ')' => {
                self.pop_interp_stack();
                RParen
            }
            '[' => LBrack,
            ']' => RBrack,
            '{' => LBrace,
            '}' => RBrace,
            '@' => At,
            ',' => Comma,
            ':' => Colon,
            ';' => Semi,
            '$' => Dollar,

            '=' => self.fork(Eq, '=', EqEq),
            '>' => match self.peek() {
                Some('=') => {
                    self.next();
                    GtEq
                }
                Some('>') => {
                    self.next();
                    self.fork(GtGt, '=', GtGtEq)
                }
                _ => Gt,
            },
            '<' => match self.peek() {
                Some('=') => {
                    self.next();
                    LtEq
                }
                Some('<') => {
                    self.next();
                    self.fork(LtLt, '=', LtLtEq)
                }
                _ => Lt,
            },
            '!' => self.fork(Not, '=', NotEq),
            '.' => {
                if !self.try_consume('.') {
                    Dot
                } else if self.try_consume('<') {
                    DotDotLt
                } else if self.try_consume('=') {
                    DotDotEq
                } else {
                    DotDot
                }
            }
            '\'' => self.char_or_lifetime(),
            '"' => self.string(),
            _ => Unknown,
        };

        kind
    }

    fn should_be_filtered(&self, kind: TokenKind) -> bool {
        use TokenKind::*;

        if self.filter_mode.ignores_whitespace() && kind == Whitespace {
            return true;
        }

        if self.filter_mode.ignores_comments()
            && matches!(kind, LineComment | BlockComment | UntermComment)
        {
            return true;
        }

        false
    }

    pub fn lex(mut self) -> Vec<Token> {
        // TODO: Turn this back into an iterator, idk what I was thinking
        // TODO: Then again, what about all the context switching??
        let mut tokens = Vec::new();
        while let (start, Some(ch)) = (self.idx, self.next()) {
            let kind = self.lex_token_kind(ch);

            if !self.should_be_filtered(kind) {
                tokens.push(Token::new(kind, Span::new(start, self.idx)));
            }
            if self.just_exited {
                hint::cold_path();
                let start = self.idx;
                self.just_exited = false;
                tokens.push(Token::new(self.string(), Span::new(start, self.idx)));
            }
        }
        tokens
    }
}

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

pub struct Lexer<'tok, 'src> {
    bytes: &'tok [u8],
    idx: usize,
    diag: &'tok mut Diagnostics<'src>,
    filter_mode: FilterMode,
}

const WINDOW_SIZE: usize = 4;
type Window = [u8; WINDOW_SIZE];

impl<'tok, 'src> Lexer<'tok, 'src> {
    pub fn new_with_mode(
        src: &'tok str,
        diag: &'tok mut Diagnostics<'src>,
        filter_mode: FilterMode,
    ) -> Self {
        Self { bytes: src.as_bytes(), idx: 0, diag, filter_mode }
    }

    pub fn new(src: &'tok str, diag: &'tok mut Diagnostics<'src>) -> Self {
        Self { bytes: src.as_bytes(), idx: 0, diag, filter_mode: FilterMode::default() }
    }

    fn peek(&mut self) -> Option<u8> {
        // TODO: Speed up
        self.bytes.get(self.idx).copied()
    }

    fn peek_n(&mut self, n: usize) -> Option<u8> {
        // TODO: Speed up
        self.bytes.get(self.idx + n).copied()
    }

    fn skip(&mut self, n: usize) {
        self.idx += n;
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

    fn eat_until<P>(&self, skip: usize, predicate: P) -> usize
    where
        P: Fn(&u8) -> bool,
    {
        self.bytes[self.idx + skip..].iter().position(predicate).unwrap_or(0) + skip
    }

    fn line_comment(&self) -> usize {
        self.eat_until(2, |&b| b == b'\n')
    }

    fn block_comment(&self) -> (TokenKind, usize) {
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

    // Maybe make this into a two-part struct?
    fn ident_or_keyword(&self) -> (TokenKind, usize) {
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
        self.eat_until(
            2,
            |&b| !matches!(b, b'0'..=b'9' | b'_' | b'a'..=b'z' | b'A'..=b'Z'),
        )
    }

    fn number(&self) -> (TokenKind, usize) {
        // Looks all the way to one byte ahead. Make sure to subtract 1 from offset.
        let mut offset = 0;
        let mut seen_dot = false;
        let mut seen_exp = false;
        let mut last_was_exp_sign = false;
        for bytes in self.bytes[self.idx..].windows(2) {
            match bytes {
                [b'e' | b'E', b'+' | b'-'] => {
                    last_was_exp_sign = true;
                    seen_exp = true;
                }
                [b'e' | b'E', b'0'..=b'9' | b'_'] => {}
                [b'e' | b'E', b'a'..=b'z' | b'A'..=b'Z'] => {}
                [b'+' | b'-', _] if last_was_exp_sign => last_was_exp_sign = false,
                [b'0'..=b'9' | b'_', _] => {}
                [b'a'..=b'z' | b'A'..=b'Z', _] => {}
                [b'.', b'0'..=b'9'] if !seen_dot => seen_dot = true,

                _ => break,
            }

            offset += 1;
        }

        if self.idx + offset == self.bytes.len() - 1 {
            let last_byte = self.bytes[self.idx + offset];
            if matches!(last_byte, b'0'..=b'9' | b'_' | b'a'..=b'z' | b'A'..=b'Z') {
                offset += 1;
            }
        }

        if seen_dot || seen_exp {
            (TokenKind::Float, offset)
        } else {
            (TokenKind::Int, offset)
        }
    }

    fn char_or_lifetime(&self) -> (TokenKind, usize) {
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
                    [_, ws] if can_be_annot && ws.is_ascii_whitespace() => {
                        break 'eater TokenKind::Annot;
                    }
                    [_, b'\n'] => break 'eater TokenKind::UntermChar,
                    [b'\\', b'\''] => {}
                    [_, b'\''] => {
                        offset += 1;
                        break 'eater TokenKind::Char;
                    }
                    _ => {}
                }

                offset += 1;
            }

            TokenKind::UntermChar
        };

        (kind, offset)
    }

    pub fn lex_token_kind(&mut self) -> (TokenKind, usize) {
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

            [b'(', ..] => (LParen, 1), // TODO: Track interp parens
            [b')', ..] => (RParen, 1), // TODO: Track interp parens
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
            [b'"', ..] => (String, 1), // TODO: Handle strings

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
                panic!("I will implement non-ascii eating later...");
            }

            let (kind, len) = self.lex_token_kind();
            self.idx += len;

            if !self.should_filter(kind) {
                tokens.push(Token::new(kind, Span::new(start as u32, self.idx as u32)));
            }
        }

        tokens
    }
}
