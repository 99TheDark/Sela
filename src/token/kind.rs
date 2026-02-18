#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Whitespace,
    NewLine,
    LineComment,
    BlockComment,
    Ident,
    Int,
    Float,
    Char,
    String,
    Annot,
    Eq,       //\\ =
    Plus,     //\\ +
    Dash,     //\\ -
    Star,     //\\ *
    Slash,    //\\ /
    Pct,      //\\ %
    Gt,       //\\ >
    Lt,       //\\ <
    EqEq,     //\\ ==
    NotEq,    //\\ !=
    GtEq,     //\\ >=
    LtEq,     //\\ <=
    GtGt,     //\\ >>
    LtLt,     //\\ <<
    Caret,    //\\ ^
    And,      //\\ &
    Bar,      //\\ |
    Not,      //\\ !
    LParen,   //\\ (
    RParen,   //\\ )
    LBrack,   //\\ [
    RBrack,   //\\ ]
    LBrace,   //\\ {
    RBrace,   //\\ }
    At,       //\\ @
    Colon,    //\\ :
    Semi,     //\\ ;
    Comma,    //\\ ,
    Dot,      //\\ .
    DotDot,   //\\ ..
    DotDotLt, //\\ ..<
    DotDotEq, //\\ ..=
    Dollar,   //\\ $

    NoChar, //\\ ''
    UntermComment,
    UntermQuot,
    UntermQuotEsc,
    UntermStr,
    Unknown,
}

impl TokenKind {
    pub fn is_unknown(&self) -> bool {
        *self == Self::Unknown
    }

    pub fn is_invalid(&self) -> bool {
        use TokenKind::*;
        matches!(
            self,
            Unknown | NoChar | UntermComment | UntermQuot | UntermQuotEsc | UntermStr
        )
    }
}
