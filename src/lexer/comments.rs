use std::hint;

use crate::{
    lexer::{Lexer, NextToken},
    token::kind::TokenKind,
};

impl<'tok, 'src> Lexer<'tok, 'src> {
    pub(super) fn line_comment(&self) -> usize {
        self.eat_until(2, |&b| b == b'\n')
    }

    pub(super) fn block_comment(&self) -> NextToken {
        // TODO: Make SIMD by in chunks bitmasking all /* and */ and counting
        // If that even is faster since // SIMD wasn't faster, nor was whitespace...
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
                hint::cold_path();
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
}
