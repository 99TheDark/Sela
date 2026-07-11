use std::{
    hint,
    simd::{cmp::SimdPartialEq, mask8x16, u8x16},
};

use crate::{
    lexer::{Lexer, NextToken, vectorized::SimdRange},
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
    fn ident_simd_chunk(bytes: u8x16) -> mask8x16 {
        bytes.simd_in_range(b'a'..=b'z')
            | bytes.simd_in_range(b'A'..=b'Z')
            | bytes.simd_eq(u8x16::splat(b'_'))
            | bytes.simd_in_range(b'0'..=b'9')
    }

    pub(super) fn ident_or_keyword(&self) -> NextToken {
        // Very gross but after much benchmarking it beats all offsets
        // alongside never SIMD and always SIMD
        let len = if let Some(byte) = self.bytes.get(self.idx + 4) {
            if byte.word_legal() {
                self.eat_until_simd(1, Self::ident_simd_chunk, |&b| !b.word_legal())
            } else {
                self.eat_until(1, |&b| !b.word_legal())
            }
        } else {
            hint::cold_path();
            self.eat_until(1, |&b| !b.word_legal())
        };

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
