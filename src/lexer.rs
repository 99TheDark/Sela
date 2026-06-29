use std::{iter, str};

use smallvec::SmallVec;

use crate::{
    core::span::Span,
    error::Diagnostics,
    token::{Token, kind::TokenKind},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenFilterMode {
    WhitespaceAndComments,
    Whitespace,
    None,
}

impl Default for TokenFilterMode {
    fn default() -> Self {
        Self::WhitespaceAndComments
    }
}

impl TokenFilterMode {
    pub fn ignores_comments(self) -> bool {
        self == Self::WhitespaceAndComments
    }

    pub fn ignores_whitespace(self) -> bool {
        self == Self::WhitespaceAndComments || self == Self::Whitespace
    }
}

pub struct Lexer<'a, 'b> {
    chars: iter::Peekable<str::Chars<'a>>,
    diag: &'a mut Diagnostics<'b>,
    idx: u32,
    interp_stack: SmallVec<[usize; 2]>,
    just_exited: bool,
    filter_mode: TokenFilterMode,
}

impl<'a, 'b> Lexer<'a, 'b> {
    pub fn new_with_mode(
        src: &'a str,
        diag: &'a mut Diagnostics<'b>,
        filter_mode: TokenFilterMode,
    ) -> Self {
        Self {
            chars: src.chars().peekable(),
            diag,
            idx: 0,
            interp_stack: SmallVec::new(),
            just_exited: false,
            filter_mode,
        }
    }

    pub fn new(src: &'a str, diag: &'a mut Diagnostics<'b>) -> Self {
        Self::new_with_mode(src, diag, TokenFilterMode::WhitespaceAndComments)
    }

    pub fn next(&mut self) -> Option<char> {
        let ch = self.chars.next()?;
        self.idx += ch.len_utf8() as u32;
        Some(ch)
    }

    pub fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
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

    pub fn eat_while_mut<F>(&mut self, mut valid: F)
    where
        F: FnMut(char) -> bool,
    {
        while let Some(ch) = self.peek() {
            if !valid(ch) {
                break;
            }
            self.next();
        }
    }

    fn number(&mut self) -> TokenKind {
        let mut seen_dot = false; // This means no .123
        let mut seen_exp = false;
        let mut just_saw_exp = false;
        self.eat_while_mut(|ch| {
            if just_saw_exp {
                just_saw_exp = false;
                if matches!(ch, '+' | '-') {
                    return true;
                }
            }

            match ch {
                '.' => {
                    if seen_dot {
                        false
                    } else {
                        seen_dot = true;
                        true
                    }
                }
                'e' | 'E' => {
                    seen_exp = true;
                    just_saw_exp = true;
                    true
                }
                '0'..='9' | '_' => true,
                _ => false,
            }
        });

        if seen_dot || seen_exp { TokenKind::Float } else { TokenKind::Int }
    }

    fn char_or_lifetime(&mut self) -> TokenKind {
        use TokenKind::*;

        let Some(ch) = self.next() else {
            return UntermQuot;
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
            '\n' => return UntermQuot,
            '\'' => return NoChar,
            _ => {}
        }

        while let Some(ch) = self.peek() {
            if ch == '\n' {
                return UntermQuot;
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
        while let Some(ch) = self.next() {
            match ch {
                '\"' => break,
                '\\' => {
                    self.next();
                }
                '$' if self.peek() == Some('(') => {
                    self.interp_stack.push(0);
                    break;
                }
                _ => {}
            };
        }

        TokenKind::String
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
            'a'..='z' | 'A'..='Z' | '_' => {
                self.eat_while(
                    |ch| matches!(ch, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9'),
                );
                Ident
            }
            // TODO: Implement literally every other numeric representation
            '0' => TokenKind::Int,
            '0'..='9' => self.number(),
            // Lit: TBD
            '+' => self.fork(Plus, '=', PlusEq),
            '-' => match self.peek() {
                Some('=') => DashEq,
                Some('>') => Arrow,
                _ => Dash,
            },
            '*' => self.fork(Star, '=', StarEq),
            '/' => match self.peek() {
                Some('/') => {
                    self.eat_while(|ch| ch != '\n');
                    LineComment
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
        use TokenKind::{BlockComment, LineComment, Whitespace};

        if self.filter_mode.ignores_whitespace() && kind == Whitespace {
            return true;
        }
        if self.filter_mode.ignores_comments()
            && matches!(kind, LineComment | BlockComment)
        {
            return true;
        }

        false
    }

    pub fn lex(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let (start, Some(ch)) = (self.idx, self.next()) {
            let kind = self.lex_token_kind(ch);

            if !self.should_be_filtered(kind) {
                tokens.push(Token::new(kind, Span::new(start, self.idx)));
            }
            if self.just_exited {
                let start = self.idx;
                self.just_exited = false;
                tokens.push(Token::new(self.string(), Span::new(start, self.idx)));
            }
        }
        tokens
    }
}
