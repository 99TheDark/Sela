use crate::token::precedence::Precedence;

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
    Eq,     //\\ =
    Plus,   //\\ +
    PlusEq, //\\ +=
    // PlusBar,   //\\ +|
    // PlusBarEq, //\\ +|=
    // PlusPct,   //\\ +%
    // PlusPctEq, //\\ +%=
    Dash,   //\\ -
    DashEq, //\\ -=
    // DashBar,   //\\ -|
    // DashBarEq, //\\ -|=
    // DashPct,   //\\ -%
    // DashPctEq, //\\ -%=
    Star,   //\\ *
    StarEq, //\\ *=
    // StarBar,   //\\ *|
    // StarBarEq, //\\ *|=
    // StarPct,   //\\ *%
    // StarPctEq, //\\ *%=
    Slash,   //\\ /
    SlashEq, //\\ /=
    Pct,     //\\ %
    PctEq,   //\\ %=
    Gt,      //\\ >
    Lt,      //\\ <
    EqEq,    //\\ ==
    NotEq,   //\\ !=
    GtEq,    //\\ >=
    LtEq,    //\\ <=
    GtGt,    //\\ >>
    GtGtEq,  //\\ >>=
    // GtGtPct,   //\\ >>%
    // GtGtPctEq, //\\ >>%=
    LtLt,   //\\ <<
    LtLtEq, //\\ <<=
    // LtLtBar,   //\\ <<|
    // LtLtBarEq, //\\ <<|=
    // LtLtPct,   //\\ <<%
    // LtLtPctEq, //\\ <<%=
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

    Let,   //\\ let
    Const, //\\ const
    Mut,   //\\ mut
    Type,  //\\ type
    Enum,  //\\ enum
    Class, //\\ class
    Idea,  //\\ idea
    Func,  //\\ func
    Mod,   //\\ mod
    Pub,   //\\ pub
    Inn,   //\\ inn
    Pri,   //\\ pri
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
    PsnGtGtBar,    //\\ >>|
    PsnGtGtBarEq,  //\\ >>|=
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

    pub const fn is_invalid(&self) -> bool {
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

    pub const fn is_recovery_terminator(&self) -> bool {
        use TokenKind::*;
        matches!(self, RParen | RBrack | RBrace | NewLine | Comma | Eq | EOF)
    }

    pub const fn nud_prec(&self) -> Precedence {
        use TokenKind::*;
        match self {
            DotDot | DotDotLt | DotDotEq => Precedence::Range,
            Dash | Star | Amp | Not => Precedence::Unary,
            _ => Precedence::None,
        }
    }

    pub const fn led_prec(&self) -> Precedence {
        use TokenKind::*;
        match self {
            Eq | PlusEq | DashEq | StarEq | SlashEq | PctEq | GtGtEq | LtLtEq
            | CaretEq | AmpEq | BarEq => Precedence::Assign,
            DotDot | DotDotLt | DotDotEq => Precedence::Range,
            Or => Precedence::ShortOr,
            And => Precedence::ShortAnd,
            EqEq | NotEq => Precedence::Equal,
            Gt | Lt | GtEq | LtEq => Precedence::Inequal,
            Bar => Precedence::EagerOr,
            Caret => Precedence::EagerXor,
            Amp => Precedence::EagerAnd,
            GtGt | LtLt => Precedence::Shift,
            Plus | Dash => Precedence::Addive,
            Star | Slash | Pct => Precedence::Mulive,
            As => Precedence::Cast,
            At => Precedence::Bind,
            Colon => Precedence::Pair,
            Dot | LParen | LBrack => Precedence::Prop,

            _ => Precedence::None,
        }
    }
}
