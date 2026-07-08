use crate::{lexer::Lexer, token::kind::TokenKind};

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
}

impl<'tok, 'src> Lexer<'tok, 'src> {
    pub(super) fn should_filter(&self, kind: TokenKind) -> bool {
        use TokenKind::*;
        if self.filter_mode.ignore_whitespace && kind == Whitespace {
            true
        } else if self.filter_mode.ignore_comments && kind.is_comment() {
            true
        } else {
            false
        }
    }
}
