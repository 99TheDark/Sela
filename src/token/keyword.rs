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
    Macro, //\\ macro
    Use,   //\\ use
    Charm, //\\ charm
    As,    //\\ as
    True,  //\\ true
    False, //\\ false
    In,    //\\ in
    And,   //\\ and
    Or,    //\\ or
    NotReserved,
}

impl Keyword {
    pub fn is_keyword(self) -> bool {
        self != Self::NotReserved
    }

    pub fn from_token(tok: Token, src: &str) -> Self {
        use Keyword::*;

        if !tok.is_ident() {
            return NotReserved;
        }

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
            "macro" => Macro,
            "use" => Use,
            "charm" => Charm,
            "as" => As,
            "true" => True,
            "false" => False,
            "in" => In,
            "and" => And,
            "or" => Or,
            _ => NotReserved,
        }
    }
}
