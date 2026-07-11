use std::hint;

use crate::{
    core::span::Span,
    lexer::{Lexer, NextToken, words::WordLegal},
    token::{Token, kind::TokenKind},
};

impl<'tok, 'src> Lexer<'tok, 'src> {
    pub(super) fn push_interp_stack(&mut self) {
        let Some(parens) = self.interp_stack.last_mut() else {
            return;
        };
        *parens += 1;
    }

    pub(super) fn pop_interp_stack(&mut self) {
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

    pub(super) fn skip_non_ascii(&mut self) {
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

    pub(super) fn char_or_lifetime(&self) -> NextToken {
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
                    [_, b] if can_be_annot && !b.word_legal() => {
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

    pub(super) fn string(&mut self) -> NextToken {
        let mut offset = 0;
        let broke = 'eater: {
            let mut just_saw_esc_dollar = false;
            for bytes in self.bytes[self.idx..].windows(2) {
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
            hint::cold_path();
            (TokenKind::UntermStr, self.bytes.len() - self.idx)
        }
    }

    pub(super) fn exit_string(&mut self) -> (Token, usize) {
        if self.peek() == Some(b'"') {
            let tok = Token::new(
                TokenKind::String,
                Span::new(self.idx as u32, self.idx as u32 + 1),
            );
            (tok, 1)
        } else {
            let (kind, len) = self.string();
            let span = Span::new(self.idx as u32, (self.idx + len) as u32);
            let tok = Token::new(kind, span);
            (tok, len)
        }
    }
}
