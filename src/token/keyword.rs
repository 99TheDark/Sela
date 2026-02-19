use crate::token::Token;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Keyword {
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
    Oper,  //\\ oper
    Macro, //\\ macro
    Quote, //\\ quote
    As,    //\\ as
    True,  //\\ true
    False, //\\ false
    Pulse, //\\ pulse
    And,   //\\ and
    Or,    //\\ or
    Use,   //\\ use
    NotReserved,
}

impl Keyword {
    pub fn from_token(tok: Token, src: &str) -> Self {
        use Keyword::*;
        match tok.src(src) {
            "let" => Let,
            "const" => Const,
            "mut" => Mut,
            "type" => Type,
            "enum" => Enum,
            "class" => Class,
            "idea" => Idea,
            "func" => Func,
            "mod" => Mod,
            "if" => If,
            "else" => Else,
            "loop" => Loop,
            "while" => While,
            "for" => For,
            "match" => Match,
            "break" => Break,
            "continue" => Cont,
            "return" => Ret,
            "self" => LSelf,
            "Self" => BSelf,
            "oper" => Oper,
            "macro" => Macro,
            "quote" => Quote,
            "as" => As,
            "true" => True,
            "false" => False,
            "pulse" => Pulse,
            "and" => And,
            "or" => Or,
            "use" => Use,
            _ => NotReserved,
        }
    }
}
