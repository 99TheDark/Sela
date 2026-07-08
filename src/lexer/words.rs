use crate::{
    lexer::{Lexer, NextToken},
    token::kind::TokenKind,
};

pub(super) trait WordLegal {
    fn word_legal(&self) -> bool;
}

impl WordLegal for u8 {
    #[inline(always)]
    fn word_legal(&self) -> bool {
        matches!(self, b'0'..=b'9' | b'_' | b'a'..=b'z' | b'A'..=b'Z')
    }
}

impl<'tok, 'src> Lexer<'tok, 'src> {
    pub(super) fn ident_or_keyword(&self) -> NextToken {
        let len = self.eat_until(1, |&b| !b.word_legal());
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
}
