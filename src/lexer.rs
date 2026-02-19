use std::{iter, str};

use crate::{
    error::Diagnostics,
    token::{
        Token,
        kind::TokenKind,
        span::{Location, Span},
    },
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
    diag: &'b mut Diagnostics<'a>,
    loc: Location,
    interp_stack: Vec<usize>,
    just_exited: bool,
    filter_mode: TokenFilterMode,
}

impl<'a, 'b> Lexer<'a, 'b> {
    pub fn new_with_mode(
        src: &'a str,
        diag: &'b mut Diagnostics<'a>,
        filter_mode: TokenFilterMode,
    ) -> Self {
        Self {
            chars: src.chars().peekable(),
            diag,
            loc: Location::ZERO,
            interp_stack: Vec::new(),
            just_exited: false,
            filter_mode,
        }
    }

    pub fn new(src: &'a str, diag: &'b mut Diagnostics<'a>) -> Self {
        Self::new_with_mode(src, diag, TokenFilterMode::WhitespaceAndComments)
    }

    pub fn next(&mut self) -> Option<char> {
        let ch = self.chars.next()?;

        self.loc.idx += ch.len_utf8();
        if ch == '\n' {
            self.loc.col = 0;
            self.loc.row += 1;
        } else {
            self.loc.col += 1;
        }

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
            '0'..='9' => {
                self.eat_while(|ch| matches!(ch, '0'..='9' | '_'));
                TokenKind::Int
            }
            // Lit: TBD
            '+' => Plus,
            '-' => Dash,
            '*' => Star,
            '/' => {
                if self.peek() == Some('/') {
                    self.eat_while(|ch| ch != '\n');
                    LineComment
                } else {
                    Slash
                }
            }
            '%' => Pct,
            '&' => And,
            '|' => Bar,
            '^' => Caret,
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
            '>' => self.fork(Gt, '=', GtEq),
            '<' => self.fork(Lt, '=', LtEq),
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
        while let (start, Some(ch)) = (self.loc, self.next()) {
            let kind = self.lex_token_kind(ch);

            if !self.should_be_filtered(kind) {
                tokens.push(Token::new(kind, Span::new(start, self.loc)));
            }
            if self.just_exited {
                let start = self.loc;
                self.just_exited = false;
                tokens.push(Token::new(self.string(), Span::new(start, self.loc)));
            }
        }
        tokens
    }
}
