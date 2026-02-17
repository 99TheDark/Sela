use std::{error, fs};

use crate::token::kind::TokenKind;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod token;

fn main() -> Result<(), Box<dyn error::Error>> {
    let src = fs::read_to_string("io/test.qi")?;
    let tokens = lexer::lex(&src);

    fs::write("io/tokens.txt", {
        lexer::lex(&src)
            .map(|tok| {
                format!(
                    "{}{:?}<{:?}> = `{}`",
                    if tok.kind.is_unknown() { "!! " } else { "" },
                    tok.kind,
                    tok.span,
                    tok.str_value(&src),
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    })?;

    let ast = parser::parse(tokens, &src);

    fs::write(
        "io/ast.txt",
        format!("{:#?}", ast.collect::<Vec<_>>()).replace("    ", "│ "),
    )?;

    Ok(())
}
