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
    PlusEq,   //\\ +=
    Dash,     //\\ -
    DashEq,   //\\ -=
    Star,     //\\ *
    StarEq,   //\\ *=
    Slash,    //\\ /
    SlashEq,  //\\ /=
    Pct,      //\\ %
    PctEq,    //\\ %=
    Gt,       //\\ >
    Lt,       //\\ <
    EqEq,     //\\ ==
    NotEq,    //\\ !=
    GtEq,     //\\ >=
    LtEq,     //\\ <=
    GtGt,     //\\ >>
    GtGtEq,   //\\ >>=
    LtLt,     //\\ <<
    LtLtEq,   //\\ <<=
    Caret,    //\\ ^
    CaretEq,  //\\ ^=
    And,      //\\ &
    AndEq,    //\\ &=
    Bar,      //\\ |
    BarEq,    //\\ |=
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
    Tick,     //\\ `

    NoChar, //\\ ''
    UntermComment,
    UntermQuot,
    UntermQuotEsc,
    UntermStr,
    Unknown,
    EOF,
}

impl TokenKind {
    pub fn is_unknown(self) -> bool {
        self == Self::Unknown
    }

    pub fn is_invalid(&self) -> bool {
        use TokenKind::*;
        matches!(
            self,
            Unknown
                | EOF
                | NoChar
                | UntermComment
                | UntermQuot
                | UntermQuotEsc
                | UntermStr
        )
    }
}
