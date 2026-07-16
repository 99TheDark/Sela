use crate::{
    lexer::{Lexer, NextToken, words::WordLegal},
    token::kind::TokenKind,
};

impl<'tok, 'src> Lexer<'tok, 'src> {
    pub(super) fn radix_int(&self) -> usize {
        self.eat_until(2, |&b| !b.word_legal())
    }

    pub(super) fn number(&self) -> NextToken {
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
                [b'e' | b'E', _] => seen_exp = true,
                [b'+' | b'-', _] if just_saw_exp_sign => just_saw_exp_sign = false,
                [b'0'..=b'9' | b'_', _] => {}
                [b'a'..=b'z' | b'A'..=b'Z', _] => {}
                [b'.', b'0'..=b'9' | b'_'] if !seen_dot => seen_dot = true,

                _ => break,
            }
            offset += 1;
        }

        if self.idx + offset == self.bytes.len() - 1 {
            let last_byte = self.bytes[self.idx + offset];
            if last_byte.word_legal() {
                offset += 1;
            }
        }

        if seen_dot || seen_exp { (TokenKind::Float, offset) } else { (TokenKind::Int, offset) }
    }
}
