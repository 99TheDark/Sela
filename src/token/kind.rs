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
    Amp,      //\\ &
    AmpEq,    //\\ &=
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
    QMark,    //\\ ?
    Hash,     //\\ #
    Arrow,    //\\ ->

    //? Maybe will use/add/replace with
    Let,   //\\ let
    Const, //\\ const
    Mut,   //\\ mut
    Type,  //\\ type
    Enum,  //\\ enum
    Class, //\\ class
    Idea,  //\\ idea
    Func,  //\\ func
    Mod,   //\\ mod
    If,    //\\ if
    Else,  //\\ else
    Loop,  //\\ loop
    While, //\\ while
    For,   //\\ for
    Match, //\\ match
    Break, //\\ break
    Cont,  //\\ continue
    Ret,   //\\ return
    LSelf, //\\ self
    BSelf, //\\ Self
    Macro, //\\ macro
    Use,   //\\ use
    Charm, //\\ charm
    As,    //\\ as
    True,  //\\ true
    False, //\\ false
    In,    //\\ in
    And,   //\\ and
    Or,    //\\ or

    NoChar,        //\\ ''
    UntermComment, //\\ /* blah blah
    UntermQuot,
    UntermQuotEsc,
    UntermStr,

    // Poison tokens are handled only for better error messages from common mistakes
    PsnAmpAmp,     //\\ && //! &&T + x & &y -> maybe remove + peephole
    PsnBarBar,     //\\ ||
    PsnCaretCaret, //\\ ^^
    PsnNullish,    //\\ ??
    PsnNullishEq,  //\\ ??=
    PsnGtGtGt,     //\\ >>>
    PsnGtGtGtEq,   //\\ >>>=
    PsnLtGt,       //\\ <>
    PsnStarStar,   //\\ ** //! x * *y -> maybe remove?
    PsnStarStarEq, //\\ **=
    PsnColonEq,    //\\ := //! Shouldn't this complain about there being no type?
    PsnPlusPlus,   //\\ ++
    PsnDashDash,   //\\ -- //! a - -b
    PsnEqEqEq,     //\\ ===
    PsnNotEqEq,    //\\ !==
    PsnFatArrow,   //\\ =>
    PsnColonColon, //\\ ::
    PsnSpaceship,  //\\ <=>
    PsnDotDotDot,  //\\ ... //! What of .Red...Blue?
    PsnPipe,       //\\ |>
    PsnTilde,      //\\ ~
    PsnTildeEq,    //\\ ~=
    PsnBSlash,     //\\ \

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

    pub fn is_recovery_terminator(&self) -> bool {
        use TokenKind::*;
        matches!(self, RParen | RBrack | RBrace | NewLine | Comma | Eq | EOF)
    }
}
