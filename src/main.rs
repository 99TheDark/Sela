use std::fs;

use crate::{error::Diagnostics, lexer::Lexer, parser::Parser, token::kind::TokenKind};

pub mod ast;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod token;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let src = fs::read_to_string("io/test.qi")?;

    let mut diag = Diagnostics::new();

    let tokens = Lexer::new(&src, &mut diag).lex();

    fs::write("io/tokens.txt", {
        let mut diag = Diagnostics::new();
        Lexer::new(&src, &mut diag)
            .lex()
            .into_iter()
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

    let ast = Parser::new(&src, &tokens, &mut diag).parse();

    fs::write("io/ast.txt", format!("{:#?}", ast).replace("    ", "│ "))?;

    println!("\x1b[31mHello, world! (in red)\x1b[0m");

    Ok(())
}
