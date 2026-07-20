use crate::token::precedence::Precedence;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Whitespace,
    NewLine,
    LineComment,
    DocLineComment, //\\  (Document Line Comment)
    BlockComment,
    DocBlockComment, //\\ (Document Block Comment)
    Ident,           //\\ (Identifier)
    Int,             //\\ (Integer)
    RadixInt,        //\\ (Radix Integer)
    Float,           //\\ (Floating-Point Number)
    Char,            //\\ (Character)
    String,          //\\ (String)
    Annot,           //\\ (Annotation)
    Eq,              //\\ =
    Plus,            //\\ +
    PlusEq,          //\\ +=
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
    Const, //\\ const (Constant)
    Mut,   //\\ mut   (Mutable)  --- Maybe change to `uniq`?
    Type,  //\\ type
    Alias, //\\ alias
    Enum,  //\\ enum  (Enumeration)
    Impl,  //\\ impl  (Implement)
    Idea,  //\\ idea
    Func,  //\\ func  (Function)
    Mod,   //\\ mod   (Module)
    Pub,   //\\ pub   (Public)
    Inn,   //\\ inn   (Inner)
    Pri,   //\\ pri   (Private)
    If,    //\\ if
    Else,  //\\ else
    Loop,  //\\ loop
    While, //\\ while
    For,   //\\ for
    Match, //\\ match
    Break, //\\ break
    Cont,  //\\ continue
    Ret,   //\\ return
    LSelf, //\\ self  (Little Self)
    BSelf, //\\ Self  (Big Self)
    // Par,   //\\ par   (Parallel)
    // Co,    //\\ co    (Concurrent)
    // Async, //\\ async (Asynchronous)
    // Await, //\\ await
    // Macro, //\\ macro
    Use,   //\\ use
    Charm, //\\ charm
    As,    //\\ as
    True,  //\\ true
    False, //\\ false
    In,    //\\ in
    // Is,    //\\ is
    And, //\\ and
    Or,  //\\ or

    EmptyChar,     //\\ ''           (Empty Character)
    UntermComment, //\\ /* blah blah (Unterminated Comment)
    UntermChar,    //\\ ' blah       (Unterminated Character)
    UntermStr,     //\\ "blah blah   (Unterminated String)

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
    // ...regardless, should that be (.Red)..(.Blue) or (.Red..).Blue?
    PsnPipe,    //\\ |>
    PsnTilde,   //\\ ~
    PsnTildeEq, //\\ ~=
    PsnBSlash,  //\\ \

    Unknown,
    EOF, // (End of File)
}

impl TokenKind {
    pub fn is_unknown(self) -> bool {
        self == Self::Unknown
    }

    pub const fn is_comment(&self) -> bool {
        matches!(self, Self::LineComment | Self::BlockComment | Self::UntermComment)
    }

    pub const fn is_invalid(&self) -> bool {
        use TokenKind::*;
        matches!(self, Unknown | EOF | EmptyChar | UntermComment | UntermChar | UntermStr)
    }

    pub const fn is_recovery_terminator(&self) -> bool {
        use TokenKind::*;
        matches!(self, RParen | RBrack | RBrace | NewLine | Comma | Eq | EOF)
    }

    #[inline(always)]
    pub const fn nud_prec(&self) -> Precedence {
        use TokenKind::*;
        match self {
            DotDot | DotDotLt | DotDotEq => Precedence::Range,
            Plus | Dash | Star | Amp | Not => Precedence::Unary,
            _ => Precedence::None,
        }
    }

    #[inline(always)]
    pub const fn led_prec(&self) -> Precedence {
        use TokenKind::*;
        match self {
            Eq | PlusEq | DashEq | StarEq | SlashEq | PctEq | GtGtEq | LtLtEq | CaretEq | AmpEq
            | BarEq => Precedence::Assign,
            DotDot | DotDotLt | DotDotEq => Precedence::Range,
            Or => Precedence::ShortOr,
            And => Precedence::ShortAnd,
            EqEq | NotEq => Precedence::Equal,
            Gt | Lt | GtEq | LtEq | In => Precedence::Relat,
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
