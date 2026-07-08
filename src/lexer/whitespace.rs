use crate::lexer::Lexer;

impl<'tok, 'src> Lexer<'tok, 'src> {
    pub(super) fn whitespace(&self) -> usize {
        self.eat_until(1, |&b| !b.is_ascii_whitespace())
    }
}
