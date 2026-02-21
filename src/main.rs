use std::fs;

use bumpalo::Bump;

use crate::{error::Diagnostics, lexer::Lexer, parser::Parser, token::kind::TokenKind};

pub mod ast;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod token;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let src = fs::read_to_string("io/test.qi")?;
    let mut diag = Diagnostics::new("io/test.qi".to_string(), &src);

    let tokens = Lexer::new(&src, &mut diag).lex();

    // For debugging
    fs::write("io/tokens.txt", {
        tokens
            .clone()
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

    let arena = Bump::new();
    let ast = Parser::new(&src, &tokens, &mut diag, &arena).parse();

    // For debugging
    fs::write(
        "io/ast.txt",
        format!("{:#?}", ast).replace("    ", "  "), // │
    )?;

    println!("{}", &src);
    diag.print();

    Ok(())
}
