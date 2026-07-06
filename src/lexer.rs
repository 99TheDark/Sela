use std::str;

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

pub struct Lexer<'tok, 'src> {
    src: &'src str,
    diag: &'tok mut Diagnostics<'src>,
    idx: u32,
    interp_stack: SmallVec<[usize; 2]>,
    just_exited: bool,
    filter_mode: TokenFilterMode,
}

impl<'tok, 'src> Lexer<'tok, 'src> {
    pub fn new_with_mode(
        src: &'src str,
        diag: &'tok mut Diagnostics<'src>,
        filter_mode: TokenFilterMode,
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
        Self::new_with_mode(src, diag, TokenFilterMode::WhitespaceAndComments)
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
        match ident {
            "let" => Let,
            "const" => Const,
            "mut" => Mut,
            "type" => Type,
            "enum" => Enum,
            "class" => Class,
            "idea" => Idea,
            "func" => Func,
            "mod" => Mod,
            "pub" => Pub,
            "inn" => Inn,
            "pri" => Pri,
            "if" => If,
            "else" => Else,
            "loop" => Loop,
            "while" => While,
            "for" => For,
            "match" => Match,
            "break" => Break,
            "continue" => Cont,
            "return" => Ret,
            "self" => LSelf,
            "Self" => BSelf,
            "macro" => Macro,
            "use" => Use,
            "charm" => Charm,
            "as" => As,
            "true" => True,
            "false" => False,
            "in" => In,
            "and" => And,
            "or" => Or,
            _ => Ident,
        }
    }

    fn number(&mut self) -> TokenKind {
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
                Some('*') => {
                    self.next();
                    let terminated = self.block_comment();

                    if terminated { BlockComment } else { UntermComment }
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
                let start = self.idx;
                self.just_exited = false;
                tokens.push(Token::new(self.string(), Span::new(start, self.idx)));
            }
        }
        tokens
    }
}
