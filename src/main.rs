use std::fs;

use crate::{
    error::Diagnostics,
    lexer::{Lexer, TokenFilterMode},
    parser::Parser,
    token::kind::TokenKind,
};

pub mod ast;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod token;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let src = fs::read_to_string("io/test.qi")?;

    let mut diag = Diagnostics::new();

    let tokens = lexer::lex(&src, &mut diag);

    fs::write("io/tokens.txt", {
        let mut diag = Diagnostics::new();
        lexer::lex(&src, &mut diag)
            .map(|tok| {
                format!(
                    "{}{:?}<{:?}> = `{}`",
                    if tok.kind.is_unknown() { "!! " } else { "" },
                    tok.kind,
                    tok.span,
                    tok.debug_src(&src),
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    })?;

    let ast = parser::parse(tokens, &src, &mut diag);

    fs::write(
        "io/ast.txt",
        format!("{:#?}", ast.collect::<Vec<_>>()).replace("    ", "│ "),
    )?;

    println!("\x1b[31mHello, I am red.\x1b[0m");

    Ok(())
}
